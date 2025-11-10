//! Persistent homology computation.
//!
//! This module implements algorithms for computing persistent homology, which tracks
//! how topological features (connected components, loops, voids) appear and disappear
//! as a parameter varies in a filtered simplicial complex.

use crate::error::{Result, TdaError};
use crate::simplicial_complex::{FilteredSimplex, SimplicialComplex};
use std::collections::HashMap;

/// Represents a persistent homology feature (birth-death pair).
#[derive(Debug, Clone)]
pub struct PersistentPair {
    /// Dimension of the homology class (0 for components, 1 for loops, etc.)
    pub dimension: usize,
    /// Birth time (filtration value when feature appears)
    pub birth: f64,
    /// Death time (filtration value when feature disappears)
    pub death: f64,
}

impl PersistentPair {
    /// Create a new persistent pair.
    pub fn new(dimension: usize, birth: f64, death: f64) -> Self {
        PersistentPair {
            dimension,
            birth,
            death,
        }
    }

    /// Get the persistence (lifetime) of this feature.
    pub fn persistence(&self) -> f64 {
        self.death - self.birth
    }

    /// Check if this feature is still alive (death = infinity).
    pub fn is_infinite(&self) -> bool {
        self.death.is_infinite()
    }
}

/// Union-Find data structure for tracking connected components.
#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    birth_time: Vec<f64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            birth_time: vec![0.0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize, time: f64) -> Option<PersistentPair> {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return None;
        }

        let (younger, older) = if self.birth_time[root_x] > self.birth_time[root_y] {
            (root_x, root_y)
        } else {
            (root_y, root_x)
        };

        // Create persistence pair for the younger component
        let pair = PersistentPair::new(0, self.birth_time[younger], time);

        // Union by rank
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }

        Some(pair)
    }

    fn get_birth_time(&mut self, x: usize) -> f64 {
        let root = self.find(x);
        self.birth_time[root]
    }
}

/// Compute persistent homology using the standard reduction algorithm.
///
/// This implementation uses the matrix reduction algorithm to compute
/// persistence pairs in all dimensions.
///
/// # Arguments
/// * `complex` - A filtered simplicial complex
/// * `max_dimension` - Maximum dimension to compute homology for
///
/// # Returns
/// Vector of persistence pairs for all dimensions
///
/// # Algorithm
/// The algorithm reduces the boundary matrix column by column from left to right.
/// When a column reduces to zero, it creates a birth event. When a column is used
/// to reduce another column, it creates a death event.
pub fn compute_persistence(
    complex: &SimplicialComplex,
    max_dimension: usize,
) -> Result<Vec<PersistentPair>> {
    let mut pairs = Vec::new();

    // Dimension 0 (connected components) using Union-Find
    let pairs_0 = compute_persistence_dim0(complex)?;
    pairs.extend(pairs_0);

    // Higher dimensions using matrix reduction
    for dim in 1..=max_dimension.min(complex.max_dimension) {
        let pairs_d = compute_persistence_dim_reduction(complex, dim)?;
        pairs.extend(pairs_d);
    }

    Ok(pairs)
}

/// Compute 0-dimensional persistence (connected components) using Union-Find.
fn compute_persistence_dim0(complex: &SimplicialComplex) -> Result<Vec<PersistentPair>> {
    let vertices: Vec<_> = complex
        .simplices_by_dimension(0)
        .iter()
        .map(|fs| (fs.simplex.vertices[0], fs.filtration_value))
        .collect();

    let n = vertices.len();
    if n == 0 {
        return Ok(Vec::new());
    }

    let mut uf = UnionFind::new(n);
    let mut pairs = Vec::new();

    // Initialize birth times for vertices
    for (i, &(_, birth)) in vertices.iter().enumerate() {
        uf.birth_time[i] = birth;
    }

    // Process edges
    let edges: Vec<_> = complex
        .simplices_by_dimension(1)
        .iter()
        .map(|fs| {
            let v0 = fs.simplex.vertices[0];
            let v1 = fs.simplex.vertices[1];
            (v0, v1, fs.filtration_value)
        })
        .collect();

    for (v0, v1, time) in edges {
        if let Some(pair) = uf.union(v0, v1, time) {
            pairs.push(pair);
        }
    }

    // Remaining components are infinite
    let mut roots = std::collections::HashSet::new();
    for i in 0..n {
        let root = uf.find(i);
        if roots.insert(root) {
            pairs.push(PersistentPair::new(
                0,
                uf.get_birth_time(i),
                f64::INFINITY,
            ));
        }
    }

    Ok(pairs)
}

/// Compute persistence in dimension d using matrix reduction.
fn compute_persistence_dim_reduction(
    complex: &SimplicialComplex,
    dimension: usize,
) -> Result<Vec<PersistentPair>> {
    let simplices_d: Vec<&FilteredSimplex> = complex.simplices_by_dimension(dimension);
    let simplices_dm1: Vec<&FilteredSimplex> = complex.simplices_by_dimension(dimension - 1);

    if simplices_d.is_empty() {
        return Ok(Vec::new());
    }

    // Build index maps
    let mut simplex_to_idx = HashMap::new();
    for (i, fs) in simplices_dm1.iter().enumerate() {
        simplex_to_idx.insert(&fs.simplex, i);
    }

    // Build boundary matrix as sparse columns
    let mut boundary_matrix: Vec<Vec<usize>> = Vec::new();
    for fs in simplices_d.iter() {
        let mut column = Vec::new();
        for (_, face) in fs.simplex.boundary() {
            if let Some(&idx) = simplex_to_idx.get(&face) {
                column.push(idx);
            }
        }
        column.sort_unstable();
        boundary_matrix.push(column);
    }

    // Reduced matrix (we only track which columns are reduced and their low values)
    let mut low: Vec<Option<usize>> = vec![None; boundary_matrix.len()];
    let mut pairs = Vec::new();

    // Reduction algorithm
    for j in 0..boundary_matrix.len() {
        let mut column = boundary_matrix[j].clone();

        while let Some(&pivot) = column.last() {
            // Find if any previous column has the same low value
            let mut found_match = false;
            for i in 0..j {
                if low[i] == Some(pivot) {
                    // Add column i to column j (symmetric difference)
                    column = symmetric_difference(&column, &boundary_matrix[i]);
                    found_match = true;
                    break;
                }
            }

            if !found_match {
                break;
            }
        }

        if let Some(&pivot) = column.last() {
            // Column j is not zero after reduction
            low[j] = Some(pivot);

            // This creates a death event for the feature born at simplex pivot
            let birth_time = simplices_dm1[pivot].filtration_value;
            let death_time = simplices_d[j].filtration_value;

            pairs.push(PersistentPair::new(dimension - 1, birth_time, death_time));
        }

        boundary_matrix[j] = column;
    }

    // Features that never die (born but no death event)
    for (i, fs) in simplices_dm1.iter().enumerate() {
        let mut is_killed = false;
        for &lv in low.iter() {
            if lv == Some(i) {
                is_killed = true;
                break;
            }
        }

        if !is_killed && fs.simplex.dimension() == dimension - 1 {
            // Check if this simplex has zero boundary (potential generator)
            let boundary = fs.simplex.boundary();
            if !boundary.is_empty() {
                // This is a potential infinite feature
                // We need to check if it was created
                pairs.push(PersistentPair::new(
                    dimension - 1,
                    fs.filtration_value,
                    f64::INFINITY,
                ));
            }
        }
    }

    Ok(pairs)
}

/// Compute symmetric difference of two sorted vectors (XOR for sets).
fn symmetric_difference(a: &[usize], b: &[usize]) -> Vec<usize> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            result.push(a[i]);
            i += 1;
        } else if a[i] > b[j] {
            result.push(b[j]);
            j += 1;
        } else {
            // Equal elements cancel out
            i += 1;
            j += 1;
        }
    }

    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);

    result
}

/// Compute Betti numbers from persistence pairs.
///
/// Betti numbers count the number of k-dimensional holes at a given filtration value.
///
/// # Arguments
/// * `pairs` - Persistence pairs
/// * `dimension` - Which Betti number to compute (0 for components, 1 for loops, etc.)
/// * `filtration_value` - The filtration value at which to compute Betti number
///
/// # Returns
/// The d-th Betti number at the given filtration value
pub fn betti_number(pairs: &[PersistentPair], dimension: usize, filtration_value: f64) -> usize {
    pairs
        .iter()
        .filter(|p| {
            p.dimension == dimension && p.birth <= filtration_value && p.death > filtration_value
        })
        .count()
}

/// Compute the Euler characteristic from Betti numbers.
///
/// The Euler characteristic is the alternating sum of Betti numbers:
/// chi = beta_0 - beta_1 + beta_2 - beta_3 + ...
pub fn euler_characteristic(betti_numbers: &[usize]) -> i32 {
    betti_numbers
        .iter()
        .enumerate()
        .map(|(i, &b)| if i % 2 == 0 { b as i32 } else { -(b as i32) })
        .sum()
}

/// Compute persistent homology for a point cloud using Vietoris-Rips complex.
///
/// This is a convenience function that builds a Rips complex and computes persistence.
///
/// # Arguments
/// * `distance_matrix` - Distance matrix for the point cloud
/// * `max_radius` - Maximum radius for the filtration
/// * `max_dimension` - Maximum dimension to compute homology for
///
/// # Returns
/// Vector of persistence pairs
pub fn persistent_homology_rips(
    distance_matrix: &nalgebra::DMatrix<f64>,
    max_radius: f64,
    max_dimension: usize,
) -> Result<Vec<PersistentPair>> {
    let complex =
        crate::simplicial_complex::vietoris_rips_complex(distance_matrix, max_radius, max_dimension)?;
    compute_persistence(&complex, max_dimension)
}

/// Compute persistent homology using Cech complex.
pub fn persistent_homology_cech(
    distance_matrix: &nalgebra::DMatrix<f64>,
    max_radius: f64,
    max_dimension: usize,
) -> Result<Vec<PersistentPair>> {
    let complex =
        crate::simplicial_complex::cech_complex(distance_matrix, max_radius, max_dimension)?;
    compute_persistence(&complex, max_dimension)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::DMatrix;

    #[test]
    fn test_persistence_simple() {
        // Three points in a line: (0,0), (1,0), (2,0)
        let dist = DMatrix::from_row_slice(
            3,
            3,
            &[0.0, 1.0, 2.0, 1.0, 0.0, 1.0, 2.0, 1.0, 0.0],
        );

        let pairs = persistent_homology_rips(&dist, 1.5, 1).unwrap();

        // Should have 3 components born at 0
        // 2 components die when edges form at distance 1.0
        // 1 component survives forever
        let dim0_pairs: Vec<_> = pairs.iter().filter(|p| p.dimension == 0).collect();
        assert!(dim0_pairs.len() >= 2);
    }

    #[test]
    fn test_persistence_circle() {
        // Create a circle as 8 points
        let n = 8;
        let mut points = Vec::new();
        for i in 0..n {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
            points.push(angle.cos());
            points.push(angle.sin());
        }

        let point_matrix = DMatrix::from_row_slice(n, 2, &points);
        let dist = crate::distances::euclidean_distance_matrix(&point_matrix).unwrap();

        let pairs = persistent_homology_rips(&dist, 2.0, 1).unwrap();

        // Should detect a loop (1-dimensional feature)
        let dim1_pairs: Vec<_> = pairs.iter().filter(|p| p.dimension == 1).collect();
        assert!(
            !dim1_pairs.is_empty(),
            "Should detect at least one loop in the circle"
        );
    }

    #[test]
    fn test_betti_numbers() {
        let pairs = vec![
            PersistentPair::new(0, 0.0, 1.0),
            PersistentPair::new(0, 0.0, 1.5),
            PersistentPair::new(0, 0.0, f64::INFINITY),
            PersistentPair::new(1, 0.5, 2.0),
        ];

        // At filtration 0.5, should have 3 components (dimension 0)
        assert_eq!(betti_number(&pairs, 0, 0.5), 3);

        // At filtration 1.2, should have 1 component and 1 loop
        assert_eq!(betti_number(&pairs, 0, 1.2), 1);
        assert_eq!(betti_number(&pairs, 1, 1.2), 1);
    }

    #[test]
    fn test_euler_characteristic() {
        let betti = vec![1, 0, 0]; // One connected component, no holes
        assert_eq!(euler_characteristic(&betti), 1);

        let betti2 = vec![1, 1, 0]; // One component, one loop (like a circle)
        assert_eq!(euler_characteristic(&betti2), 0);

        let betti3 = vec![1, 0, 1]; // Sphere (one component, one void)
        assert_eq!(euler_characteristic(&betti3), 2);
    }
}
