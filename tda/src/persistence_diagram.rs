//! Persistence diagrams and their distance metrics.
//!
//! A persistence diagram is a multiset of points (birth, death) that represent
//! topological features. This module provides data structures for persistence diagrams
//! and implements distance metrics between them (bottleneck and Wasserstein distances).

use crate::error::{Result, TdaError};
use crate::persistent_homology::PersistentPair;

/// A persistence diagram represented as a collection of birth-death points.
#[derive(Debug, Clone)]
pub struct PersistenceDiagram {
    /// Points in the diagram (birth, death, dimension)
    pub points: Vec<(f64, f64, usize)>,
}

impl PersistenceDiagram {
    /// Create a new empty persistence diagram.
    pub fn new() -> Self {
        PersistenceDiagram { points: Vec::new() }
    }

    /// Create a persistence diagram from persistence pairs.
    ///
    /// # Arguments
    /// * `pairs` - Vector of persistent pairs
    /// * `dimension` - Only include pairs of this dimension (None for all)
    pub fn from_pairs(pairs: &[PersistentPair], dimension: Option<usize>) -> Self {
        let points = pairs
            .iter()
            .filter(|p| dimension.map_or(true, |d| p.dimension == d))
            .filter(|p| !p.is_infinite()) // Exclude infinite features for diagram
            .map(|p| (p.birth, p.death, p.dimension))
            .collect();

        PersistenceDiagram { points }
    }

    /// Add a point to the diagram.
    pub fn add_point(&mut self, birth: f64, death: f64, dimension: usize) {
        self.points.push((birth, death, dimension));
    }

    /// Get the number of points in the diagram.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Check if the diagram is empty.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Get all points of a specific dimension.
    pub fn points_by_dimension(&self, dimension: usize) -> Vec<(f64, f64)> {
        self.points
            .iter()
            .filter(|(_, _, d)| *d == dimension)
            .map(|(b, d, _)| (*b, *d))
            .collect()
    }

    /// Compute the persistence (lifetime) of each point.
    pub fn persistence_values(&self) -> Vec<f64> {
        self.points.iter().map(|(b, d, _)| d - b).collect()
    }

    /// Get points with persistence above a threshold.
    pub fn significant_features(&self, threshold: f64) -> Vec<(f64, f64, usize)> {
        self.points
            .iter()
            .filter(|(b, d, _)| d - b >= threshold)
            .copied()
            .collect()
    }

    /// Convert to barcode representation.
    ///
    /// Returns a vector of (dimension, birth, death) tuples suitable for plotting.
    pub fn to_barcode(&self) -> Vec<(usize, f64, f64)> {
        self.points
            .iter()
            .map(|(b, d, dim)| (*dim, *b, *d))
            .collect()
    }
}

impl Default for PersistenceDiagram {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute the bottleneck distance between two persistence diagrams.
///
/// The bottleneck distance is the infimum over all matchings between points
/// of the maximum distance between matched points.
///
/// This is a simplified O(n^3) implementation using the Hungarian algorithm concept.
/// For large diagrams, more sophisticated algorithms should be used.
///
/// # Arguments
/// * `diagram1` - First persistence diagram
/// * `diagram2` - Second persistence diagram
/// * `dimension` - Compute distance only for this dimension (None for all)
///
/// # Returns
/// The bottleneck distance
pub fn bottleneck_distance(
    diagram1: &PersistenceDiagram,
    diagram2: &PersistenceDiagram,
    dimension: Option<usize>,
) -> Result<f64> {
    let points1 = if let Some(d) = dimension {
        diagram1.points_by_dimension(d)
    } else {
        diagram1
            .points
            .iter()
            .map(|(b, d, _)| (*b, *d))
            .collect()
    };

    let points2 = if let Some(d) = dimension {
        diagram2.points_by_dimension(d)
    } else {
        diagram2
            .points
            .iter()
            .map(|(b, d, _)| (*b, *d))
            .collect()
    };

    // Add diagonal points for unmatched features
    let mut extended_points1 = points1.clone();
    let mut extended_points2 = points2.clone();

    // Each point can match to its projection on the diagonal
    for &(b, d) in &points1 {
        extended_points2.push(((b + d) / 2.0, (b + d) / 2.0));
    }
    for &(b, d) in &points2 {
        extended_points1.push(((b + d) / 2.0, (b + d) / 2.0));
    }

    if extended_points1.is_empty() || extended_points2.is_empty() {
        return Ok(0.0);
    }

    // Compute distance matrix
    let n = extended_points1.len();
    let m = extended_points2.len();
    let size = n.max(m);

    let mut dist_matrix = vec![vec![0.0; size]; size];

    for i in 0..n {
        for j in 0..m {
            let (b1, d1) = extended_points1[i];
            let (b2, d2) = extended_points2[j];
            // L-infinity distance
            let dist = ((b1 - b2).abs()).max((d1 - d2).abs());
            dist_matrix[i][j] = dist;
        }
    }

    // Simple greedy matching (not optimal but reasonable approximation)
    let mut matched1 = vec![false; n];
    let mut matched2 = vec![false; m];
    let mut max_distance = 0.0;

    // Sort pairs by distance
    let mut pairs: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..n {
        for j in 0..m {
            pairs.push((i, j, dist_matrix[i][j]));
        }
    }
    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    // Greedy matching
    for (i, j, dist) in pairs {
        if !matched1[i] && !matched2[j] {
            matched1[i] = true;
            matched2[j] = true;
            max_distance = max_distance.max(dist);
        }
    }

    Ok(max_distance)
}

/// Compute the p-Wasserstein distance between two persistence diagrams.
///
/// The p-Wasserstein distance is the infimum over all matchings of the
/// p-th root of the sum of p-th powers of distances.
///
/// # Arguments
/// * `diagram1` - First persistence diagram
/// * `diagram2` - Second persistence diagram
/// * `p` - The order of the Wasserstein distance (typically 1 or 2)
/// * `dimension` - Compute distance only for this dimension (None for all)
///
/// # Returns
/// The p-Wasserstein distance
pub fn wasserstein_distance(
    diagram1: &PersistenceDiagram,
    diagram2: &PersistenceDiagram,
    p: f64,
    dimension: Option<usize>,
) -> Result<f64> {
    if p <= 0.0 {
        return Err(TdaError::InvalidParameter(
            "p must be positive for Wasserstein distance".to_string(),
        ));
    }

    let points1 = if let Some(d) = dimension {
        diagram1.points_by_dimension(d)
    } else {
        diagram1
            .points
            .iter()
            .map(|(b, d, _)| (*b, *d))
            .collect()
    };

    let points2 = if let Some(d) = dimension {
        diagram2.points_by_dimension(d)
    } else {
        diagram2
            .points
            .iter()
            .map(|(b, d, _)| (*b, *d))
            .collect()
    };

    // Simple greedy matching for approximation
    let mut extended_points1 = points1.clone();
    let mut extended_points2 = points2.clone();

    // Add diagonal projections
    for &(b, d) in &points1 {
        extended_points2.push(((b + d) / 2.0, (b + d) / 2.0));
    }
    for &(b, d) in &points2 {
        extended_points1.push(((b + d) / 2.0, (b + d) / 2.0));
    }

    if extended_points1.is_empty() || extended_points2.is_empty() {
        return Ok(0.0);
    }

    // Compute distance matrix
    let n = extended_points1.len();
    let m = extended_points2.len();

    let mut dist_matrix = vec![vec![0.0; m]; n];

    for i in 0..n {
        for j in 0..m {
            let (b1, d1) = extended_points1[i];
            let (b2, d2) = extended_points2[j];
            let dist = ((b1 - b2).abs()).max((d1 - d2).abs());
            dist_matrix[i][j] = dist;
        }
    }

    // Greedy matching
    let mut matched1 = vec![false; n];
    let mut matched2 = vec![false; m];
    let mut sum_powered_distances = 0.0;

    let mut pairs: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..n {
        for j in 0..m {
            pairs.push((i, j, dist_matrix[i][j]));
        }
    }
    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    for (i, j, dist) in pairs {
        if !matched1[i] && !matched2[j] {
            matched1[i] = true;
            matched2[j] = true;
            sum_powered_distances += dist.powf(p);
        }
    }

    Ok(sum_powered_distances.powf(1.0 / p))
}

/// Compute persistence landscape values.
///
/// Persistence landscapes are functional representations of persistence diagrams
/// that enable statistical analysis and averaging.
///
/// # Arguments
/// * `diagram` - Persistence diagram
/// * `k` - Which landscape function (k=0 is the first, k=1 is the second, etc.)
/// * `t` - Time parameter at which to evaluate the landscape
///
/// # Returns
/// Value of the k-th landscape function at time t
pub fn persistence_landscape(diagram: &PersistenceDiagram, k: usize, t: f64) -> f64 {
    let mut values: Vec<f64> = diagram
        .points
        .iter()
        .map(|(b, d, _)| {
            if t < *b || t > *d {
                0.0
            } else {
                (t - b).min(d - t) // Tent function
            }
        })
        .collect();

    values.sort_by(|a, b| b.partial_cmp(a).unwrap()); // Sort in descending order

    values.get(k).copied().unwrap_or(0.0)
}

/// Compute persistence entropy.
///
/// A scalar summary statistic that measures the "spread" of persistence values.
/// Useful for comparing diagrams and detecting changes in topology.
///
/// # Arguments
/// * `diagram` - Persistence diagram
///
/// # Returns
/// Persistence entropy value
pub fn persistence_entropy(diagram: &PersistenceDiagram) -> f64 {
    let persistences: Vec<f64> = diagram.persistence_values();

    if persistences.is_empty() {
        return 0.0;
    }

    let total: f64 = persistences.iter().sum();

    if total == 0.0 {
        return 0.0;
    }

    let mut entropy = 0.0;
    for &p in &persistences {
        let prob = p / total;
        if prob > 0.0 {
            entropy -= prob * prob.ln();
        }
    }

    entropy
}

/// Compute persistent Betti numbers as a function.
///
/// Returns a vector of (filtration_value, betti_number) pairs.
///
/// # Arguments
/// * `pairs` - Persistence pairs
/// * `dimension` - Which Betti number to compute
/// * `num_points` - Number of points to sample in the filtration
///
/// # Returns
/// Vector of (filtration, betti_number) pairs
pub fn persistent_betti_curve(
    pairs: &[PersistentPair],
    dimension: usize,
    num_points: usize,
) -> Vec<(f64, usize)> {
    let finite_pairs: Vec<_> = pairs.iter().filter(|p| !p.is_infinite()).collect();

    if finite_pairs.is_empty() {
        return vec![(0.0, 0)];
    }

    let min_val = finite_pairs
        .iter()
        .map(|p| p.birth)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0);
    let max_val = finite_pairs
        .iter()
        .map(|p| p.death)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(1.0);

    let step = (max_val - min_val) / (num_points as f64);

    (0..=num_points)
        .map(|i| {
            let t = min_val + (i as f64) * step;
            let betti = crate::persistent_homology::betti_number(pairs, dimension, t);
            (t, betti)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persistence_diagram() {
        let mut diagram = PersistenceDiagram::new();
        diagram.add_point(0.0, 1.0, 0);
        diagram.add_point(0.5, 1.5, 1);
        diagram.add_point(0.2, 0.8, 0);

        assert_eq!(diagram.len(), 3);
        assert_eq!(diagram.points_by_dimension(0).len(), 2);
        assert_eq!(diagram.points_by_dimension(1).len(), 1);
    }

    #[test]
    fn test_persistence_values() {
        let mut diagram = PersistenceDiagram::new();
        diagram.add_point(0.0, 1.0, 0);
        diagram.add_point(0.5, 1.5, 0);

        let pers = diagram.persistence_values();
        assert_eq!(pers.len(), 2);
        assert!((pers[0] - 1.0).abs() < 1e-10);
        assert!((pers[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_significant_features() {
        let mut diagram = PersistenceDiagram::new();
        diagram.add_point(0.0, 1.0, 0); // persistence 1.0
        diagram.add_point(0.5, 0.6, 0); // persistence 0.1
        diagram.add_point(0.0, 2.0, 1); // persistence 2.0

        let significant = diagram.significant_features(0.5);
        assert_eq!(significant.len(), 2); // Two features with persistence >= 0.5
    }

    #[test]
    fn test_bottleneck_distance() {
        let mut diagram1 = PersistenceDiagram::new();
        diagram1.add_point(0.0, 1.0, 0);
        diagram1.add_point(0.5, 1.5, 0);

        let mut diagram2 = PersistenceDiagram::new();
        diagram2.add_point(0.0, 1.0, 0);
        diagram2.add_point(0.5, 1.5, 0);

        let dist = bottleneck_distance(&diagram1, &diagram2, None).unwrap();
        assert!(dist < 1e-6); // Should be very close to 0 for identical diagrams
    }

    #[test]
    fn test_persistence_landscape() {
        let mut diagram = PersistenceDiagram::new();
        diagram.add_point(0.0, 2.0, 0);

        // At t=1.0, the tent function should be at its peak
        let value = persistence_landscape(&diagram, 0, 1.0);
        assert!((value - 1.0).abs() < 1e-10);

        // Outside the interval, should be 0
        let value_outside = persistence_landscape(&diagram, 0, 3.0);
        assert!(value_outside.abs() < 1e-10);
    }

    #[test]
    fn test_persistence_entropy() {
        let mut diagram = PersistenceDiagram::new();
        diagram.add_point(0.0, 1.0, 0);
        diagram.add_point(0.0, 1.0, 0);

        let entropy = persistence_entropy(&diagram);
        assert!(entropy >= 0.0); // Entropy should be non-negative
    }
}
