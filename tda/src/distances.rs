//! Distance metrics for point clouds and spike trains.
//!
//! This module provides various distance metrics used in TDA, including:
//! - Euclidean distance for point clouds
//! - Correlation distance
//! - Victor-Purpura distance for spike trains
//! - van Rossum distance for spike trains

use crate::error::{Result, TdaError};
use nalgebra::DMatrix;
use rayon::prelude::*;

/// Compute the Euclidean distance matrix for a set of points.
///
/// # Arguments
/// * `points` - Matrix where each row is a point in d-dimensional space
///
/// # Returns
/// Symmetric distance matrix where entry (i,j) is the Euclidean distance between points i and j
///
/// # Example
/// ```
/// use nalgebra::DMatrix;
/// use tda::distances::euclidean_distance_matrix;
///
/// let points = DMatrix::from_row_slice(3, 2, &[
///     0.0, 0.0,
///     1.0, 0.0,
///     0.0, 1.0,
/// ]);
/// let dist_matrix = euclidean_distance_matrix(&points).unwrap();
/// assert_eq!(dist_matrix.nrows(), 3);
/// assert_eq!(dist_matrix.ncols(), 3);
/// ```
pub fn euclidean_distance_matrix(points: &DMatrix<f64>) -> Result<DMatrix<f64>> {
    let n = points.nrows();
    if n == 0 {
        return Err(TdaError::EmptyDataset);
    }

    let mut dist_matrix = DMatrix::zeros(n, n);

    // Parallel computation of distance matrix
    let distances: Vec<(usize, usize, f64)> = (0..n)
        .into_par_iter()
        .flat_map(|i| {
            (i + 1..n).into_par_iter().map(move |j| {
                let diff = points.row(i) - points.row(j);
                let dist = diff.norm();
                (i, j, dist)
            })
        })
        .collect();

    for (i, j, dist) in distances {
        dist_matrix[(i, j)] = dist;
        dist_matrix[(j, i)] = dist;
    }

    Ok(dist_matrix)
}

/// Compute the correlation distance matrix.
///
/// The correlation distance between vectors x and y is defined as:
/// d_corr(x, y) = 1 - |corr(x, y)|
///
/// where corr is the Pearson correlation coefficient.
///
/// # Arguments
/// * `data` - Matrix where each row is a data vector
///
/// # Returns
/// Symmetric distance matrix based on correlation
pub fn correlation_distance_matrix(data: &DMatrix<f64>) -> Result<DMatrix<f64>> {
    let n = data.nrows();
    if n == 0 {
        return Err(TdaError::EmptyDataset);
    }

    let mut dist_matrix = DMatrix::zeros(n, n);

    // Compute means for each row
    let means: Vec<f64> = (0..n).map(|i| data.row(i).mean()).collect();

    // Parallel computation
    let distances: Vec<(usize, usize, f64)> = (0..n)
        .into_par_iter()
        .flat_map(|i| {
            let means_clone = means.clone();
            (i + 1..n).into_par_iter().map(move |j| {
                let x = data.row(i);
                let y = data.row(j);
                let mean_x = means_clone[i];
                let mean_y = means_clone[j];

                let mut numerator = 0.0;
                let mut sum_sq_x = 0.0;
                let mut sum_sq_y = 0.0;

                for k in 0..x.len() {
                    let dx = x[k] - mean_x;
                    let dy = y[k] - mean_y;
                    numerator += dx * dy;
                    sum_sq_x += dx * dx;
                    sum_sq_y += dy * dy;
                }

                let denominator = (sum_sq_x * sum_sq_y).sqrt();
                let corr = if denominator > 1e-10 {
                    (numerator / denominator).abs()
                } else {
                    0.0
                };

                let dist = 1.0 - corr;
                (i, j, dist)
            })
        })
        .collect();

    for (i, j, dist) in distances {
        dist_matrix[(i, j)] = dist;
        dist_matrix[(j, i)] = dist;
    }

    Ok(dist_matrix)
}

/// Represents a spike train as a sorted vector of spike times.
#[derive(Debug, Clone)]
pub struct SpikeTrain {
    pub times: Vec<f64>,
}

impl SpikeTrain {
    /// Create a new spike train from a vector of spike times.
    ///
    /// The times will be sorted automatically.
    pub fn new(mut times: Vec<f64>) -> Result<Self> {
        if times.iter().any(|&t| t < 0.0) {
            return Err(TdaError::InvalidSpikeTrain(
                "Spike times must be non-negative".to_string(),
            ));
        }
        times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Ok(SpikeTrain { times })
    }

    /// Get the number of spikes in the train.
    pub fn len(&self) -> usize {
        self.times.len()
    }

    /// Check if the spike train is empty.
    pub fn is_empty(&self) -> bool {
        self.times.is_empty()
    }
}

/// Compute the Victor-Purpura distance between two spike trains.
///
/// The Victor-Purpura distance is a spike train metric that considers
/// the cost of inserting, deleting, and shifting spikes. The parameter q
/// controls the cost of shifting spikes in time.
///
/// # Arguments
/// * `train1` - First spike train
/// * `train2` - Second spike train
/// * `q` - Cost parameter for shifting spikes (1/q is the time scale)
///
/// # Returns
/// Distance between the two spike trains
///
/// # Reference
/// Victor, J. D., & Purpura, K. P. (1997). Metric-space analysis of spike trains.
pub fn victor_purpura_distance(train1: &SpikeTrain, train2: &SpikeTrain, q: f64) -> Result<f64> {
    if q < 0.0 {
        return Err(TdaError::InvalidParameter(
            "q parameter must be non-negative".to_string(),
        ));
    }

    let n = train1.len();
    let m = train2.len();

    // Dynamic programming matrix
    let mut dp = vec![vec![0.0; m + 1]; n + 1];

    // Initialize boundaries
    for i in 0..=n {
        dp[i][0] = i as f64;
    }
    for j in 0..=m {
        dp[0][j] = j as f64;
    }

    // Fill DP table
    for i in 1..=n {
        for j in 1..=m {
            let time_diff = (train1.times[i - 1] - train2.times[j - 1]).abs();
            let shift_cost = q * time_diff;

            dp[i][j] = (dp[i - 1][j] + 1.0) // Delete from train1
                .min(dp[i][j - 1] + 1.0) // Insert into train1
                .min(dp[i - 1][j - 1] + shift_cost); // Shift spike
        }
    }

    Ok(dp[n][m])
}

/// Compute the van Rossum distance between two spike trains.
///
/// The van Rossum distance convolves each spike train with an exponential kernel
/// and computes the L2 distance between the resulting functions.
///
/// # Arguments
/// * `train1` - First spike train
/// * `train2` - Second spike train
/// * `tau` - Time constant of the exponential kernel
///
/// # Returns
/// Distance between the two spike trains
///
/// # Reference
/// van Rossum, M. C. W. (2001). A novel spike distance.
pub fn van_rossum_distance(train1: &SpikeTrain, train2: &SpikeTrain, tau: f64) -> Result<f64> {
    if tau <= 0.0 {
        return Err(TdaError::InvalidParameter(
            "tau parameter must be positive".to_string(),
        ));
    }

    // Combine and sort all spike times
    let mut all_times = Vec::new();
    all_times.extend(train1.times.iter().copied());
    all_times.extend(train2.times.iter().copied());
    all_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if all_times.is_empty() {
        return Ok(0.0);
    }

    let t_max = all_times.last().unwrap() + 5.0 * tau;
    let dt = tau / 10.0; // Time step for numerical integration
    let n_steps = ((t_max - all_times[0]) / dt).ceil() as usize;

    let mut sum_sq = 0.0;

    // Numerical integration
    for step in 0..n_steps {
        let t = all_times[0] + (step as f64) * dt;

        // Convolve train1
        let mut f1 = 0.0;
        for &spike_time in &train1.times {
            if t >= spike_time {
                f1 += (-(t - spike_time) / tau).exp();
            }
        }

        // Convolve train2
        let mut f2 = 0.0;
        for &spike_time in &train2.times {
            if t >= spike_time {
                f2 += (-(t - spike_time) / tau).exp();
            }
        }

        sum_sq += (f1 - f2).powi(2) * dt;
    }

    Ok(sum_sq.sqrt() / tau.sqrt())
}

/// Compute a distance matrix for a collection of spike trains using Victor-Purpura distance.
pub fn victor_purpura_matrix(trains: &[SpikeTrain], q: f64) -> Result<DMatrix<f64>> {
    let n = trains.len();
    if n == 0 {
        return Err(TdaError::EmptyDataset);
    }

    let mut dist_matrix = DMatrix::zeros(n, n);

    let distances: Vec<(usize, usize, f64)> = (0..n)
        .into_par_iter()
        .flat_map(|i| {
            (i + 1..n)
                .into_par_iter()
                .map(move |j| {
                    let dist = victor_purpura_distance(&trains[i], &trains[j], q).unwrap_or(0.0);
                    (i, j, dist)
                })
        })
        .collect();

    for (i, j, dist) in distances {
        dist_matrix[(i, j)] = dist;
        dist_matrix[(j, i)] = dist;
    }

    Ok(dist_matrix)
}

/// Compute a distance matrix for a collection of spike trains using van Rossum distance.
pub fn van_rossum_matrix(trains: &[SpikeTrain], tau: f64) -> Result<DMatrix<f64>> {
    let n = trains.len();
    if n == 0 {
        return Err(TdaError::EmptyDataset);
    }

    let mut dist_matrix = DMatrix::zeros(n, n);

    let distances: Vec<(usize, usize, f64)> = (0..n)
        .into_par_iter()
        .flat_map(|i| {
            (i + 1..n)
                .into_par_iter()
                .map(move |j| {
                    let dist = van_rossum_distance(&trains[i], &trains[j], tau).unwrap_or(0.0);
                    (i, j, dist)
                })
        })
        .collect();

    for (i, j, dist) in distances {
        dist_matrix[(i, j)] = dist;
        dist_matrix[(j, i)] = dist;
    }

    Ok(dist_matrix)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_euclidean_distance_matrix() {
        let points = DMatrix::from_row_slice(3, 2, &[0.0, 0.0, 1.0, 0.0, 0.0, 1.0]);
        let dist = euclidean_distance_matrix(&points).unwrap();

        assert_abs_diff_eq!(dist[(0, 0)], 0.0, epsilon = 1e-10);
        assert_abs_diff_eq!(dist[(0, 1)], 1.0, epsilon = 1e-10);
        assert_abs_diff_eq!(dist[(0, 2)], 1.0, epsilon = 1e-10);
        assert_abs_diff_eq!(dist[(1, 2)], 2.0_f64.sqrt(), epsilon = 1e-10);
    }

    #[test]
    fn test_spike_train() {
        let train = SpikeTrain::new(vec![1.0, 3.0, 2.0]).unwrap();
        assert_eq!(train.times, vec![1.0, 2.0, 3.0]);
        assert_eq!(train.len(), 3);
        assert!(!train.is_empty());
    }

    #[test]
    fn test_victor_purpura_distance() {
        let train1 = SpikeTrain::new(vec![1.0, 2.0, 3.0]).unwrap();
        let train2 = SpikeTrain::new(vec![1.1, 2.1, 3.1]).unwrap();

        let dist = victor_purpura_distance(&train1, &train2, 1.0).unwrap();
        assert!(dist > 0.0);
        assert!(dist < 3.0); // Should be less than deleting and inserting all spikes

        // Distance to self should be zero
        let dist_self = victor_purpura_distance(&train1, &train1, 1.0).unwrap();
        assert_abs_diff_eq!(dist_self, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_van_rossum_distance() {
        let train1 = SpikeTrain::new(vec![1.0, 2.0, 3.0]).unwrap();
        let train2 = SpikeTrain::new(vec![1.1, 2.1, 3.1]).unwrap();

        let dist = van_rossum_distance(&train1, &train2, 0.5).unwrap();
        assert!(dist > 0.0);

        // Distance to self should be zero
        let dist_self = van_rossum_distance(&train1, &train1, 0.5).unwrap();
        assert_abs_diff_eq!(dist_self, 0.0, epsilon = 1e-6);
    }
}
