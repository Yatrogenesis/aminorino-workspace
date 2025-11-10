//! Network analysis tools and measures.

use crate::error::{NeuralDynamicsError, Result};
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Calculate Kuramoto order parameter for synchrony measurement.
///
/// R = |1/N Σ exp(iθ_j)| where θ_j are phases
///
/// # Returns
///
/// Order parameter R ∈ [0, 1] where 1 = perfect synchrony, 0 = no synchrony
pub fn kuramoto_order_parameter(phases: &[f64]) -> f64 {
    if phases.is_empty() {
        return 0.0;
    }

    let n = phases.len() as f64;
    let sum: Complex64 = phases
        .iter()
        .map(|&theta| Complex64::new(0.0, theta).exp())
        .sum();

    (sum / n).norm()
}

/// Calculate phases from spike times using linear interpolation.
///
/// Phase is defined as 2π * (t - t_last) / ISI
pub fn spike_phases(spike_times: &[f64], current_time: f64) -> Vec<f64> {
    if spike_times.len() < 2 {
        return vec![0.0];
    }

    let mut phases = Vec::new();

    for i in 1..spike_times.len() {
        let t_prev = spike_times[i - 1];
        let t_next = spike_times[i];
        let isi = t_next - t_prev;

        if current_time >= t_prev && current_time < t_next {
            let phase = 2.0 * PI * (current_time - t_prev) / isi;
            phases.push(phase);
        }
    }

    // For time after last spike
    if current_time >= spike_times[spike_times.len() - 1] && spike_times.len() >= 2 {
        let t_prev = spike_times[spike_times.len() - 2];
        let t_last = spike_times[spike_times.len() - 1];
        let isi = t_last - t_prev;
        let phase = 2.0 * PI * (current_time - t_last) / isi;
        phases.push(phase % (2.0 * PI));
    }

    if phases.is_empty() {
        vec![0.0]
    } else {
        phases
    }
}

/// Calculate population firing rate from spike trains.
///
/// # Arguments
///
/// * `spike_trains` - Vector of spike times for each neuron
/// * `time_window` - (start, end) time window in ms
///
/// # Returns
///
/// Average firing rate in Hz
pub fn population_firing_rate(spike_trains: &[Vec<f64>], time_window: (f64, f64)) -> f64 {
    if spike_trains.is_empty() {
        return 0.0;
    }

    let (t_start, t_end) = time_window;
    let duration = t_end - t_start;

    if duration <= 0.0 {
        return 0.0;
    }

    let total_spikes: usize = spike_trains
        .iter()
        .map(|spikes| spikes.iter().filter(|&&t| t >= t_start && t < t_end).count())
        .sum();

    (total_spikes as f64 / (spike_trains.len() as f64 * duration)) * 1000.0 // Hz
}

/// Calculate cross-correlation between two spike trains.
///
/// # Arguments
///
/// * `spikes1` - First spike train
/// * `spikes2` - Second spike train
/// * `max_lag` - Maximum time lag in ms
/// * `bin_size` - Bin size for histogram in ms
pub fn cross_correlation(
    spikes1: &[f64],
    spikes2: &[f64],
    max_lag: f64,
    bin_size: f64,
) -> Result<Vec<(f64, f64)>> {
    if bin_size <= 0.0 {
        return Err(NeuralDynamicsError::InvalidParameter {
            parameter: "bin_size".to_string(),
            value: bin_size,
            reason: "must be positive".to_string(),
        });
    }

    let n_bins = (2.0 * max_lag / bin_size).ceil() as usize;
    let mut histogram = vec![0; n_bins];

    // Calculate all pairwise time differences
    for &t1 in spikes1 {
        for &t2 in spikes2 {
            let lag = t2 - t1;
            if lag.abs() <= max_lag {
                let bin = ((lag + max_lag) / bin_size).floor() as usize;
                if bin < n_bins {
                    histogram[bin] += 1;
                }
            }
        }
    }

    // Convert to (lag, count) pairs
    let result: Vec<(f64, f64)> = histogram
        .iter()
        .enumerate()
        .map(|(i, &count)| {
            let lag = i as f64 * bin_size - max_lag;
            (lag, count as f64)
        })
        .collect();

    Ok(result)
}

/// Calculate coefficient of variation of interspike intervals (CV_ISI).
///
/// CV_ISI = std(ISI) / mean(ISI)
///
/// Regular firing: CV ≈ 0, Poisson: CV ≈ 1, Bursting: CV > 1
pub fn cv_isi(spike_times: &[f64]) -> f64 {
    if spike_times.len() < 2 {
        return 0.0;
    }

    let isis: Vec<f64> = spike_times.windows(2).map(|w| w[1] - w[0]).collect();

    let mean = isis.iter().sum::<f64>() / isis.len() as f64;
    if mean == 0.0 {
        return 0.0;
    }

    let variance = isis.iter().map(|&isi| (isi - mean).powi(2)).sum::<f64>() / isis.len() as f64;
    let std = variance.sqrt();

    std / mean
}

/// Detect avalanches in network activity.
///
/// An avalanche is a contiguous period of activity separated by quiescent periods.
///
/// # Returns
///
/// Vector of (avalanche_size, avalanche_duration) tuples
pub fn detect_avalanches(
    spike_trains: &[Vec<f64>],
    bin_size: f64,
    threshold: usize,
) -> Result<Vec<(usize, f64)>> {
    if bin_size <= 0.0 {
        return Err(NeuralDynamicsError::InvalidParameter {
            parameter: "bin_size".to_string(),
            value: bin_size,
            reason: "must be positive".to_string(),
        });
    }

    // Find time range
    let mut min_time: f64 = f64::INFINITY;
    let mut max_time: f64 = f64::NEG_INFINITY;

    for spikes in spike_trains {
        if let Some(&first) = spikes.first() {
            min_time = min_time.min(first);
        }
        if let Some(&last) = spikes.last() {
            max_time = max_time.max(last);
        }
    }

    if !min_time.is_finite() || !max_time.is_finite() {
        return Ok(Vec::new());
    }

    // Bin spike counts
    let n_bins = ((max_time - min_time) / bin_size).ceil() as usize;
    let mut binned_activity = vec![0; n_bins];

    for spikes in spike_trains {
        for &t in spikes {
            let bin = ((t - min_time) / bin_size).floor() as usize;
            if bin < n_bins {
                binned_activity[bin] += 1;
            }
        }
    }

    // Detect avalanches
    let mut avalanches = Vec::new();
    let mut in_avalanche = false;
    let mut avalanche_size = 0;
    let mut avalanche_start = 0;

    for (i, &activity) in binned_activity.iter().enumerate() {
        if activity >= threshold && !in_avalanche {
            // Start of avalanche
            in_avalanche = true;
            avalanche_size = activity;
            avalanche_start = i;
        } else if activity >= threshold && in_avalanche {
            // Continue avalanche
            avalanche_size += activity;
        } else if activity < threshold && in_avalanche {
            // End of avalanche
            let duration = (i - avalanche_start) as f64 * bin_size;
            avalanches.push((avalanche_size, duration));
            in_avalanche = false;
            avalanche_size = 0;
        }
    }

    // Handle avalanche extending to end
    if in_avalanche {
        let duration = (n_bins - avalanche_start) as f64 * bin_size;
        avalanches.push((avalanche_size, duration));
    }

    Ok(avalanches)
}

/// Calculate branching parameter for criticality assessment.
///
/// σ = <n_{t+1}> / <n_t> where n_t is number of active neurons at time t
///
/// σ = 1 indicates critical state
pub fn branching_parameter(spike_trains: &[Vec<f64>], bin_size: f64) -> Result<f64> {
    if bin_size <= 0.0 {
        return Err(NeuralDynamicsError::InvalidParameter {
            parameter: "bin_size".to_string(),
            value: bin_size,
            reason: "must be positive".to_string(),
        });
    }

    // Find time range
    let mut max_time: f64 = 0.0;
    for spikes in spike_trains {
        if let Some(&last) = spikes.last() {
            max_time = max_time.max(last);
        }
    }

    let n_bins = (max_time / bin_size).ceil() as usize;
    let mut activity = vec![0; n_bins];

    // Bin activity
    for spikes in spike_trains {
        for &t in spikes {
            let bin = (t / bin_size).floor() as usize;
            if bin < n_bins {
                activity[bin] += 1;
            }
        }
    }

    // Calculate branching parameter
    let mut sum_ratio = 0.0;
    let mut count = 0;

    for i in 0..n_bins - 1 {
        if activity[i] > 0 {
            sum_ratio += activity[i + 1] as f64 / activity[i] as f64;
            count += 1;
        }
    }

    if count > 0 {
        Ok(sum_ratio / count as f64)
    } else {
        Ok(0.0)
    }
}

/// Avalanche statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvalancheStats {
    pub n_avalanches: usize,
    pub mean_size: f64,
    pub mean_duration: f64,
    pub size_std: f64,
    pub duration_std: f64,
}

impl AvalancheStats {
    pub fn from_avalanches(avalanches: &[(usize, f64)]) -> Self {
        if avalanches.is_empty() {
            return Self {
                n_avalanches: 0,
                mean_size: 0.0,
                mean_duration: 0.0,
                size_std: 0.0,
                duration_std: 0.0,
            };
        }

        let sizes: Vec<f64> = avalanches.iter().map(|&(s, _)| s as f64).collect();
        let durations: Vec<f64> = avalanches.iter().map(|&(_, d)| d).collect();

        let mean_size = sizes.iter().sum::<f64>() / sizes.len() as f64;
        let mean_duration = durations.iter().sum::<f64>() / durations.len() as f64;

        let size_variance = sizes.iter()
            .map(|&s| (s - mean_size).powi(2))
            .sum::<f64>() / sizes.len() as f64;

        let duration_variance = durations.iter()
            .map(|&d| (d - mean_duration).powi(2))
            .sum::<f64>() / durations.len() as f64;

        Self {
            n_avalanches: avalanches.len(),
            mean_size,
            mean_duration,
            size_std: size_variance.sqrt(),
            duration_std: duration_variance.sqrt(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_kuramoto_order_parameter() {
        // Perfect synchrony
        let phases = vec![0.0, 0.0, 0.0, 0.0];
        let r = kuramoto_order_parameter(&phases);
        assert_relative_eq!(r, 1.0, epsilon = 1e-10);

        // Uniform distribution (no synchrony)
        let phases = vec![0.0, PI / 2.0, PI, 3.0 * PI / 2.0];
        let r = kuramoto_order_parameter(&phases);
        assert!(r < 0.1);
    }

    #[test]
    fn test_population_firing_rate() {
        let spike_trains = vec![
            vec![10.0, 20.0, 30.0],
            vec![15.0, 25.0],
            vec![12.0, 22.0, 32.0],
        ];

        let rate = population_firing_rate(&spike_trains, (0.0, 40.0));
        // 8 spikes / (3 neurons * 40 ms) = 66.67 Hz
        assert_relative_eq!(rate, 66.67, epsilon = 0.1);
    }

    #[test]
    fn test_cv_isi() {
        // Regular firing
        let regular_spikes = vec![0.0, 10.0, 20.0, 30.0, 40.0];
        let cv = cv_isi(&regular_spikes);
        assert_relative_eq!(cv, 0.0, epsilon = 1e-10);

        // Irregular firing
        let irregular_spikes = vec![0.0, 5.0, 15.0, 18.0, 30.0];
        let cv = cv_isi(&irregular_spikes);
        assert!(cv > 0.5); // High variability
    }

    #[test]
    fn test_cross_correlation() {
        let spikes1 = vec![10.0, 20.0, 30.0];
        let spikes2 = vec![12.0, 22.0, 32.0]; // Slightly delayed

        let cc = cross_correlation(&spikes1, &spikes2, 10.0, 1.0).unwrap();

        // Should have peak at lag ≈ 2 ms
        assert!(!cc.is_empty());
    }

    #[test]
    fn test_detect_avalanches() {
        let spike_trains = vec![
            vec![1.0, 2.0, 10.0],
            vec![1.5, 2.5, 10.5],
            vec![2.0, 11.0],
        ];

        let avalanches = detect_avalanches(&spike_trains, 1.0, 1).unwrap();

        // Should detect at least 2 avalanches (around t=1-2 and t=10-11)
        assert!(avalanches.len() >= 2);
    }

    #[test]
    fn test_branching_parameter() {
        let spike_trains = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![1.5, 2.5, 3.5],
            vec![2.0, 3.0, 4.0],
        ];

        let sigma = branching_parameter(&spike_trains, 0.5).unwrap();

        // Should be positive
        assert!(sigma > 0.0);
    }

    #[test]
    fn test_avalanche_stats() {
        let avalanches = vec![(10, 5.0), (20, 10.0), (15, 7.5)];
        let stats = AvalancheStats::from_avalanches(&avalanches);

        assert_eq!(stats.n_avalanches, 3);
        assert_relative_eq!(stats.mean_size, 15.0, epsilon = 0.1);
        assert_relative_eq!(stats.mean_duration, 7.5, epsilon = 0.1);
    }
}
