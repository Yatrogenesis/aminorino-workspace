//! Simplicial complex data structures and operations.
//!
//! A simplicial complex is a collection of simplices (points, edges, triangles, tetrahedra, etc.)
//! that satisfy certain closure properties. This module provides data structures for representing
//! and manipulating simplicial complexes in the context of persistent homology.

use crate::error::{Result, TdaError};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

/// Represents a simplex as a sorted vector of vertex indices.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Simplex {
    pub vertices: Vec<usize>,
}

impl Simplex {
    /// Create a new simplex from a vector of vertex indices.
    ///
    /// The vertices will be sorted automatically.
    pub fn new(mut vertices: Vec<usize>) -> Self {
        vertices.sort_unstable();
        Simplex { vertices }
    }

    /// Get the dimension of the simplex.
    ///
    /// A k-simplex has dimension k (e.g., edge = 1, triangle = 2).
    pub fn dimension(&self) -> usize {
        if self.vertices.is_empty() {
            0
        } else {
            self.vertices.len() - 1
        }
    }

    /// Get all faces (boundary) of this simplex.
    ///
    /// For a k-simplex, returns all (k-1)-simplices that form its boundary.
    pub fn faces(&self) -> Vec<Simplex> {
        if self.vertices.len() <= 1 {
            return vec![];
        }

        self.vertices
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut face_vertices = self.vertices.clone();
                face_vertices.remove(i);
                Simplex::new(face_vertices)
            })
            .collect()
    }

    /// Compute the boundary of this simplex with signs for homology.
    ///
    /// Returns a vector of (sign, face) pairs where sign is +1 or -1.
    pub fn boundary(&self) -> Vec<(i32, Simplex)> {
        if self.vertices.len() <= 1 {
            return vec![];
        }

        self.vertices
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut face_vertices = self.vertices.clone();
                face_vertices.remove(i);
                let sign = if i % 2 == 0 { 1 } else { -1 };
                (sign, Simplex::new(face_vertices))
            })
            .collect()
    }
}

/// A filtered simplex with an associated filtration value.
#[derive(Debug, Clone)]
pub struct FilteredSimplex {
    pub simplex: Simplex,
    pub filtration_value: f64,
}

impl FilteredSimplex {
    pub fn new(simplex: Simplex, filtration_value: f64) -> Self {
        FilteredSimplex {
            simplex,
            filtration_value,
        }
    }
}

/// Type of filtration to use when building a complex.
#[derive(Debug, Clone, Copy)]
pub enum FiltrationType {
    /// Sublevel set filtration (standard for persistence)
    Sublevel,
    /// Superlevel set filtration
    Superlevel,
}

/// A simplicial complex with filtration values.
#[derive(Debug, Clone)]
pub struct SimplicialComplex {
    /// All simplices in the complex, sorted by filtration value
    pub simplices: Vec<FilteredSimplex>,
    /// Maximum dimension of simplices in the complex
    pub max_dimension: usize,
    /// Map from simplex to its index in the simplices vector
    simplex_index: HashMap<Simplex, usize>,
}

impl SimplicialComplex {
    /// Create a new empty simplicial complex.
    pub fn new() -> Self {
        SimplicialComplex {
            simplices: Vec::new(),
            max_dimension: 0,
            simplex_index: HashMap::new(),
        }
    }

    /// Add a simplex to the complex with a filtration value.
    ///
    /// This also adds all faces of the simplex if they don't already exist.
    pub fn add_simplex(&mut self, simplex: Simplex, filtration_value: f64) -> Result<()> {
        if simplex.vertices.is_empty() {
            return Err(TdaError::InvalidSimplex(
                "Simplex must have at least one vertex".to_string(),
            ));
        }

        // Check if simplex already exists
        if self.simplex_index.contains_key(&simplex) {
            return Ok(());
        }

        // Add all faces first
        for face in simplex.faces() {
            if !self.simplex_index.contains_key(&face) {
                let idx = self.simplices.len();
                self.simplices.push(FilteredSimplex::new(
                    face.clone(),
                    filtration_value.min(filtration_value),
                ));
                self.simplex_index.insert(face, idx);
            }
        }

        // Add the simplex itself
        let idx = self.simplices.len();
        self.max_dimension = self.max_dimension.max(simplex.dimension());
        self.simplices
            .push(FilteredSimplex::new(simplex.clone(), filtration_value));
        self.simplex_index.insert(simplex, idx);

        Ok(())
    }

    /// Sort simplices by filtration value and dimension.
    pub fn sort(&mut self) {
        self.simplices.sort_by(|a, b| {
            a.filtration_value
                .partial_cmp(&b.filtration_value)
                .unwrap()
                .then_with(|| a.simplex.dimension().cmp(&b.simplex.dimension()))
                .then_with(|| a.simplex.cmp(&b.simplex))
        });

        // Rebuild index
        self.simplex_index.clear();
        for (idx, filtered_simplex) in self.simplices.iter().enumerate() {
            self.simplex_index
                .insert(filtered_simplex.simplex.clone(), idx);
        }
    }

    /// Get the index of a simplex in the sorted list.
    pub fn get_index(&self, simplex: &Simplex) -> Option<usize> {
        self.simplex_index.get(simplex).copied()
    }

    /// Get all simplices of a given dimension.
    pub fn simplices_by_dimension(&self, dimension: usize) -> Vec<&FilteredSimplex> {
        self.simplices
            .iter()
            .filter(|fs| fs.simplex.dimension() == dimension)
            .collect()
    }

    /// Build the boundary matrix for computing homology.
    ///
    /// Returns a sparse representation as a vector of columns, where each column
    /// represents the boundary of a simplex.
    pub fn boundary_matrix(&self, dimension: usize) -> Vec<Vec<(usize, i32)>> {
        let simplices_d: Vec<_> = self
            .simplices
            .iter()
            .enumerate()
            .filter(|(_, fs)| fs.simplex.dimension() == dimension)
            .collect();

        let mut matrix = vec![Vec::new(); simplices_d.len()];

        for (col_idx, (_, filtered_simplex)) in simplices_d.iter().enumerate() {
            for (sign, face) in filtered_simplex.simplex.boundary() {
                if let Some(&row_idx) = self.simplex_index.get(&face) {
                    matrix[col_idx].push((row_idx, sign));
                }
            }
        }

        matrix
    }
}

impl Default for SimplicialComplex {
    fn default() -> Self {
        Self::new()
    }
}

/// Build a Vietoris-Rips complex from a distance matrix.
///
/// The Vietoris-Rips complex at scale r contains:
/// - All vertices (0-simplices)
/// - An edge between vertices i and j if distance(i,j) <= r
/// - A k-simplex if all its edges are present
///
/// # Arguments
/// * `distance_matrix` - Symmetric distance matrix
/// * `max_radius` - Maximum radius for the filtration
/// * `max_dimension` - Maximum dimension of simplices to include
///
/// # Returns
/// A filtered simplicial complex
pub fn vietoris_rips_complex(
    distance_matrix: &nalgebra::DMatrix<f64>,
    max_radius: f64,
    max_dimension: usize,
) -> Result<SimplicialComplex> {
    let n = distance_matrix.nrows();
    if n == 0 {
        return Err(TdaError::EmptyDataset);
    }

    let mut complex = SimplicialComplex::new();

    // Add vertices
    for i in 0..n {
        complex.add_simplex(Simplex::new(vec![i]), 0.0)?;
    }

    // Build edge list with filtration values
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance_matrix[(i, j)];
            if dist <= max_radius {
                edges.push((vec![i, j], dist));
            }
        }
    }

    // Add edges
    for (vertices, dist) in edges.iter() {
        complex.add_simplex(Simplex::new(vertices.clone()), *dist)?;
    }

    // Build higher-dimensional simplices using clique enumeration
    if max_dimension > 1 {
        // Build adjacency list
        let mut adjacency: HashMap<usize, HashSet<usize>> = HashMap::new();
        for i in 0..n {
            adjacency.insert(i, HashSet::new());
        }

        for (vertices, dist) in edges.iter() {
            if *dist <= max_radius {
                adjacency.get_mut(&vertices[0]).unwrap().insert(vertices[1]);
                adjacency.get_mut(&vertices[1]).unwrap().insert(vertices[0]);
            }
        }

        // Find cliques of increasing size
        for dim in 2..=max_dimension {
            let prev_simplices: Vec<_> = complex
                .simplices_by_dimension(dim - 1)
                .iter()
                .map(|fs| (fs.simplex.clone(), fs.filtration_value))
                .collect();

            for (simplex, filt_val) in prev_simplices {
                // Try to extend this simplex by one vertex
                let vertices = &simplex.vertices;

                // Find common neighbors of all vertices in the simplex
                let mut common_neighbors = adjacency[&vertices[0]].clone();
                for &v in vertices.iter().skip(1) {
                    common_neighbors = common_neighbors
                        .intersection(&adjacency[&v])
                        .copied()
                        .collect();
                }

                // Only consider vertices larger than the largest vertex in the simplex
                let max_v = *vertices.iter().max().unwrap();
                for &new_vertex in common_neighbors.iter() {
                    if new_vertex > max_v {
                        let mut new_vertices = vertices.clone();
                        new_vertices.push(new_vertex);

                        // Compute filtration value as max distance among all edges
                        let mut max_edge_dist = filt_val;
                        for &v in vertices.iter() {
                            let dist = distance_matrix[(v, new_vertex)];
                            max_edge_dist = max_edge_dist.max(dist);
                        }

                        if max_edge_dist <= max_radius {
                            complex.add_simplex(Simplex::new(new_vertices), max_edge_dist)?;
                        }
                    }
                }
            }
        }
    }

    complex.sort();
    Ok(complex)
}

/// Build a Cech complex from a distance matrix.
///
/// The Cech complex at scale r contains a k-simplex if the balls of radius r
/// centered at its vertices have a common intersection.
///
/// Note: This is computationally expensive and often approximated by Rips complex.
///
/// # Arguments
/// * `distance_matrix` - Symmetric distance matrix
/// * `max_radius` - Maximum radius for the filtration
/// * `max_dimension` - Maximum dimension of simplices to include
pub fn cech_complex(
    distance_matrix: &nalgebra::DMatrix<f64>,
    max_radius: f64,
    max_dimension: usize,
) -> Result<SimplicialComplex> {
    // For the Cech complex, we use the circumradius instead of max edge length
    // This is more accurate but also more expensive
    let n = distance_matrix.nrows();
    if n == 0 {
        return Err(TdaError::EmptyDataset);
    }

    let mut complex = SimplicialComplex::new();

    // Add vertices
    for i in 0..n {
        complex.add_simplex(Simplex::new(vec![i]), 0.0)?;
    }

    // For simplicity, we approximate Cech with Rips/2
    // True Cech would require circumradius computation
    let adjusted_radius = max_radius / 2.0;

    // Add edges
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance_matrix[(i, j)] / 2.0;
            if dist <= adjusted_radius {
                complex.add_simplex(Simplex::new(vec![i, j]), dist)?;
            }
        }
    }

    // Build higher-dimensional simplices (similar to Rips)
    if max_dimension > 1 {
        let mut adjacency: HashMap<usize, HashSet<usize>> = HashMap::new();
        for i in 0..n {
            adjacency.insert(i, HashSet::new());
        }

        for i in 0..n {
            for j in (i + 1)..n {
                let dist = distance_matrix[(i, j)] / 2.0;
                if dist <= adjusted_radius {
                    adjacency.get_mut(&i).unwrap().insert(j);
                    adjacency.get_mut(&j).unwrap().insert(i);
                }
            }
        }

        for dim in 2..=max_dimension {
            let prev_simplices: Vec<_> = complex
                .simplices_by_dimension(dim - 1)
                .iter()
                .map(|fs| (fs.simplex.clone(), fs.filtration_value))
                .collect();

            for (simplex, filt_val) in prev_simplices {
                let vertices = &simplex.vertices;
                let mut common_neighbors = adjacency[&vertices[0]].clone();
                for &v in vertices.iter().skip(1) {
                    common_neighbors = common_neighbors
                        .intersection(&adjacency[&v])
                        .copied()
                        .collect();
                }

                let max_v = *vertices.iter().max().unwrap();
                for &new_vertex in common_neighbors.iter() {
                    if new_vertex > max_v {
                        let mut new_vertices = vertices.clone();
                        new_vertices.push(new_vertex);

                        let mut max_edge_dist = filt_val;
                        for &v in vertices.iter() {
                            let dist = distance_matrix[(v, new_vertex)] / 2.0;
                            max_edge_dist = max_edge_dist.max(dist);
                        }

                        if max_edge_dist <= adjusted_radius {
                            complex.add_simplex(Simplex::new(new_vertices), max_edge_dist)?;
                        }
                    }
                }
            }
        }
    }

    complex.sort();
    Ok(complex)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplex_dimension() {
        let s0 = Simplex::new(vec![0]);
        assert_eq!(s0.dimension(), 0);

        let s1 = Simplex::new(vec![0, 1]);
        assert_eq!(s1.dimension(), 1);

        let s2 = Simplex::new(vec![0, 1, 2]);
        assert_eq!(s2.dimension(), 2);
    }

    #[test]
    fn test_simplex_faces() {
        let s = Simplex::new(vec![0, 1, 2]);
        let faces = s.faces();
        assert_eq!(faces.len(), 3);

        assert!(faces.contains(&Simplex::new(vec![1, 2])));
        assert!(faces.contains(&Simplex::new(vec![0, 2])));
        assert!(faces.contains(&Simplex::new(vec![0, 1])));
    }

    #[test]
    fn test_simplex_boundary() {
        let s = Simplex::new(vec![0, 1, 2]);
        let boundary = s.boundary();
        assert_eq!(boundary.len(), 3);

        // Check signs alternate
        assert_eq!(boundary[0].0, 1);
        assert_eq!(boundary[1].0, -1);
        assert_eq!(boundary[2].0, 1);
    }

    #[test]
    fn test_simplicial_complex() {
        let mut complex = SimplicialComplex::new();

        // Add a triangle
        complex
            .add_simplex(Simplex::new(vec![0, 1, 2]), 1.0)
            .unwrap();

        // Should have 3 vertices, 3 edges, and 1 triangle
        assert_eq!(complex.simplices_by_dimension(0).len(), 3);
        assert_eq!(complex.simplices_by_dimension(1).len(), 3);
        assert_eq!(complex.simplices_by_dimension(2).len(), 1);
    }

    #[test]
    fn test_vietoris_rips_complex() {
        use nalgebra::DMatrix;

        // Create a simple distance matrix for 4 points
        let dist = DMatrix::from_row_slice(
            4,
            4,
            &[
                0.0, 1.0, 1.5, 2.0, 1.0, 0.0, 1.5, 2.0, 1.5, 1.5, 0.0, 1.0, 2.0, 2.0, 1.0, 0.0,
            ],
        );

        let complex = vietoris_rips_complex(&dist, 1.5, 2).unwrap();

        // Should have 4 vertices
        assert_eq!(complex.simplices_by_dimension(0).len(), 4);

        // Should have edges with distance <= 1.5
        let edges = complex.simplices_by_dimension(1);
        assert!(edges.len() >= 4);
    }
}
