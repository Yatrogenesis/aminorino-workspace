//! Topological Data Analysis (TDA) Library for Neuroscience
//!
//! This library provides comprehensive tools for topological data analysis with a focus
//! on neuroscience applications. It includes:
//!
//! - **Persistent Homology**: Compute topological features (connected components, loops, voids)
//!   and track their evolution across scales
//! - **Persistence Diagrams**: Visualize and compare topological features using birth-death diagrams
//! - **Mapper Algorithm**: Create graph-based visualizations of high-dimensional data
//! - **Simplicial Complexes**: Build and manipulate Vietoris-Rips, Čech, and other complexes
//! - **Distance Metrics**: Euclidean, correlation, and spike train distances (Victor-Purpura, van Rossum)
//! - **Neural TDA**: Specialized tools for spike train analysis, cell assembly detection, and
//!   functional connectivity
//!
//! # Theory Background
//!
//! ## Persistent Homology
//!
//! Persistent homology is a method from algebraic topology that computes topological features
//! of a space at different scales. For a point cloud in Euclidean space, we build a nested
//! sequence of simplicial complexes (a filtration) and track when topological features
//! (connected components, loops, voids) appear and disappear.
//!
//! The key insight is that important topological features persist across many scales,
//! while noise creates features that appear and disappear quickly.
//!
//! ## Vietoris-Rips Complex
//!
//! Given a point cloud and a distance threshold r, the Vietoris-Rips complex includes:
//! - All points as 0-simplices (vertices)
//! - An edge between points i and j if distance(i,j) ≤ r
//! - A k-simplex if all its edges are present
//!
//! This creates a simplicial complex that captures the "shape" of the data at scale r.
//!
//! ## Mapper Algorithm
//!
//! Mapper is a method for topological visualization of high-dimensional data:
//! 1. Apply a filter function to project data to lower dimensions
//! 2. Cover the filter space with overlapping intervals
//! 3. Cluster points within each interval
//! 4. Build a graph where nodes are clusters and edges connect overlapping clusters
//!
//! This creates a graph that captures the global structure of the data.
//!
//! # Neuroscience Applications
//!
//! ## Spike Train Analysis
//!
//! Neural activity is often recorded as spike trains - sequences of action potential times.
//! TDA can analyze spike train similarity and detect patterns:
//! - Victor-Purpura distance: Measures cost of transforming one spike train to another
//! - van Rossum distance: Based on convolution with exponential kernels
//! - Persistence diagrams: Reveal topological structure in spike train ensembles
//!
//! ## Cell Assembly Detection
//!
//! Neurons that fire together form functional assemblies. TDA detects these using
//! clique topology - groups of strongly connected neurons in the functional connectivity graph.
//!
//! ## Functional Connectivity
//!
//! TDA can compute functional connectivity matrices from spike trains and analyze
//! their topological structure to understand network organization.
//!
//! # Examples
//!
//! ## Basic Persistent Homology
//!
//! ```rust
//! use nalgebra::DMatrix;
//! use tda::persistent_homology::persistent_homology_rips;
//! use tda::distances::euclidean_distance_matrix;
//!
//! // Create a circle point cloud
//! let n = 20;
//! let mut points = Vec::new();
//! for i in 0..n {
//!     let angle = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
//!     points.push(angle.cos());
//!     points.push(angle.sin());
//! }
//! let point_matrix = DMatrix::from_row_slice(n, 2, &points);
//!
//! // Compute distance matrix
//! let dist = euclidean_distance_matrix(&point_matrix).unwrap();
//!
//! // Compute persistent homology
//! let pairs = persistent_homology_rips(&dist, 2.0, 1).unwrap();
//!
//! // Should detect a loop (1-dimensional feature)
//! let loops: Vec<_> = pairs.iter().filter(|p| p.dimension == 1).collect();
//! assert!(!loops.is_empty());
//! ```
//!
//! ## Spike Train Analysis
//!
//! ```rust
//! use tda::distances::SpikeTrain;
//! use tda::neural_tda::NeuralPopulation;
//!
//! // Create spike trains
//! let train1 = SpikeTrain::new(vec![1.0, 2.0, 3.0, 5.0, 8.0]).unwrap();
//! let train2 = SpikeTrain::new(vec![1.1, 2.1, 3.1, 5.2, 8.1]).unwrap();
//! let train3 = SpikeTrain::new(vec![10.0, 11.0, 12.0]).unwrap();
//!
//! // Create neural population
//! let population = NeuralPopulation::new(vec![train1, train2, train3]).unwrap();
//!
//! // Compute persistence diagram
//! let diagram = population.persistence_diagram_vp(1.0, 5.0, 1).unwrap();
//! println!("Detected {} topological features", diagram.len());
//! ```
//!
//! ## Mapper Algorithm
//!
//! ```rust
//! use nalgebra::DMatrix;
//! use tda::mapper::{MapperBuilder, CoverStrategy, ClusteringMethod, filters};
//! use tda::distances::euclidean_distance_matrix;
//!
//! // Create some data
//! let data = DMatrix::from_row_slice(10, 2, &[
//!     0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 3.0, 0.0, 4.0, 0.0,
//!     5.0, 1.0, 6.0, 1.0, 7.0, 1.0, 8.0, 1.0, 9.0, 1.0,
//! ]);
//!
//! let dist = euclidean_distance_matrix(&data).unwrap();
//!
//! // Build Mapper graph
//! let graph = MapperBuilder::new()
//!     .filter(filters::coordinate_projection(0))
//!     .cover(CoverStrategy::Uniform {
//!         num_intervals: 3,
//!         overlap_percent: 0.3,
//!     })
//!     .clustering(ClusteringMethod::SingleLinkage { threshold: 2.0 })
//!     .build(&data, &dist)
//!     .unwrap();
//!
//! println!("Mapper graph has {} nodes", graph.node_count());
//! ```

// Re-export main components
pub mod distances;
pub mod error;
pub mod mapper;
pub mod neural_tda;
pub mod persistence_diagram;
pub mod persistent_homology;
pub mod simplicial_complex;

// Re-export commonly used types
pub use error::{Result, TdaError};

pub use distances::{
    euclidean_distance_matrix, correlation_distance_matrix, SpikeTrain,
    victor_purpura_distance, van_rossum_distance,
};

pub use simplicial_complex::{
    Simplex, SimplicialComplex, FilteredSimplex, FiltrationType,
    vietoris_rips_complex, cech_complex,
};

pub use persistent_homology::{
    PersistentPair, compute_persistence, persistent_homology_rips,
    persistent_homology_cech, betti_number, euler_characteristic,
};

pub use persistence_diagram::{
    PersistenceDiagram, bottleneck_distance, wasserstein_distance,
    persistence_landscape, persistence_entropy, persistent_betti_curve,
};

pub use mapper::{
    MapperBuilder, MapperGraph, MapperNode, ClusteringMethod, CoverStrategy,
    filters,
};

pub use neural_tda::{
    NeuralPopulation, SlidingWindowAnalysis, CellAssembly,
    detect_cell_assemblies, functional_connectivity, topological_features,
    burst_synchrony_analysis,
};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::distances::{euclidean_distance_matrix, SpikeTrain};
    pub use crate::error::{Result, TdaError};
    pub use crate::mapper::{MapperBuilder, filters, ClusteringMethod, CoverStrategy};
    pub use crate::neural_tda::{NeuralPopulation, detect_cell_assemblies};
    pub use crate::persistence_diagram::{PersistenceDiagram, bottleneck_distance};
    pub use crate::persistent_homology::{persistent_homology_rips, PersistentPair};
    pub use crate::simplicial_complex::vietoris_rips_complex;
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::DMatrix;

    #[test]
    fn test_library_integration() {
        // Test that all components work together
        let points = DMatrix::from_row_slice(4, 2, &[
            0.0, 0.0,
            1.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
        ]);

        let dist = euclidean_distance_matrix(&points).unwrap();
        let pairs = persistent_homology_rips(&dist, 2.0, 1).unwrap();
        let diagram = PersistenceDiagram::from_pairs(&pairs, None);

        assert!(diagram.len() > 0);
    }

    #[test]
    fn test_circle_detection() {
        // Create a circle and verify we detect the loop
        let n = 16;
        let mut points = Vec::new();
        for i in 0..n {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
            points.push(angle.cos());
            points.push(angle.sin());
        }

        let point_matrix = DMatrix::from_row_slice(n, 2, &points);
        let dist = euclidean_distance_matrix(&point_matrix).unwrap();
        let pairs = persistent_homology_rips(&dist, 1.5, 1).unwrap();

        // Should detect at least one 1-dimensional feature (the loop)
        let loops: Vec<_> = pairs.iter().filter(|p| p.dimension == 1).collect();
        assert!(!loops.is_empty(), "Should detect loop in circle");
    }

    #[test]
    fn test_neural_tda_integration() {
        // Test neuroscience-specific functionality
        let train1 = SpikeTrain::new(vec![1.0, 2.0, 3.0]).unwrap();
        let train2 = SpikeTrain::new(vec![1.1, 2.1, 3.1]).unwrap();

        let pop = NeuralPopulation::new(vec![train1, train2]).unwrap();
        assert_eq!(pop.size(), 2);

        let dist = pop.victor_purpura_matrix(1.0).unwrap();
        assert_eq!(dist.nrows(), 2);
        assert_eq!(dist.ncols(), 2);
    }

    #[test]
    fn test_mapper_integration() {
        let data = DMatrix::from_row_slice(6, 2, &[
            0.0, 0.0, 1.0, 0.0, 2.0, 0.0,
            3.0, 1.0, 4.0, 1.0, 5.0, 1.0,
        ]);

        let dist = euclidean_distance_matrix(&data).unwrap();

        let graph = MapperBuilder::new()
            .filter(filters::coordinate_projection(0))
            .cover(CoverStrategy::Uniform {
                num_intervals: 2,
                overlap_percent: 0.3,
            })
            .clustering(ClusteringMethod::SingleLinkage { threshold: 1.5 })
            .build(&data, &dist)
            .unwrap();

        assert!(graph.node_count() > 0);
    }

    #[test]
    fn test_betti_numbers() {
        // Create a simple complex and verify Betti numbers
        let points = DMatrix::from_row_slice(3, 2, &[
            0.0, 0.0,
            1.0, 0.0,
            0.0, 1.0,
        ]);

        let dist = euclidean_distance_matrix(&points).unwrap();
        let pairs = persistent_homology_rips(&dist, 0.5, 1).unwrap();

        // At small scale, should have 3 connected components
        let beta_0 = betti_number(&pairs, 0, 0.1);
        assert_eq!(beta_0, 3);
    }

    #[test]
    fn test_persistence_diagram_distances() {
        let mut diagram1 = PersistenceDiagram::new();
        diagram1.add_point(0.0, 1.0, 0);
        diagram1.add_point(0.5, 1.5, 1);

        let mut diagram2 = PersistenceDiagram::new();
        diagram2.add_point(0.0, 1.0, 0);
        diagram2.add_point(0.5, 1.5, 1);

        let bottleneck = bottleneck_distance(&diagram1, &diagram2, None).unwrap();
        assert!(bottleneck < 1e-6);

        let wasserstein = wasserstein_distance(&diagram1, &diagram2, 2.0, None).unwrap();
        assert!(wasserstein < 1e-6);
    }
}
