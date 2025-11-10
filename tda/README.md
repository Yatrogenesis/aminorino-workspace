# TDA: Topological Data Analysis for Neuroscience

A comprehensive Rust library for Topological Data Analysis (TDA) with specialized tools for neuroscience applications.

## Features

### Core TDA Functionality

- **Persistent Homology**: Compute topological features across scales
  - Vietoris-Rips complex construction
  - Čech complex construction
  - Standard matrix reduction algorithm
  - Birth-death pair tracking
  - Betti number computation
  - Euler characteristic calculation

- **Persistence Diagrams**: Visualize and analyze topological features
  - Birth-death point storage with dimension tracking
  - Barcode representation
  - Bottleneck distance computation
  - Wasserstein distance (p-norm)
  - Persistence landscapes
  - Persistence entropy

- **Simplicial Complexes**: Build and manipulate topological structures
  - Simplex representation with face relations
  - Boundary operators
  - Filtration values
  - Chain complex construction
  - Efficient clique enumeration for higher-dimensional simplices

- **Mapper Algorithm**: Create graph-based visualizations of high-dimensional data
  - Flexible filter functions (PCA, coordinate projection, density, eccentricity)
  - Multiple cover strategies (uniform, adaptive)
  - Clustering methods (single-linkage, DBSCAN-like)
  - Nerve complex computation using petgraph
  - Efficient overlap detection

### Distance Metrics

- **Point Cloud Distances**:
  - Euclidean distance matrices (parallel computation with rayon)
  - Correlation distance

- **Spike Train Distances**:
  - Victor-Purpura distance (edit distance with time shifts)
  - van Rossum distance (convolution-based)
  - Parallel distance matrix computation

### Neuroscience-Specific Tools

- **Spike Train Analysis**:
  - Neural population data structures
  - Victor-Purpura and van Rossum distance matrices
  - Persistence diagrams from spike train ensembles

- **Temporal Analysis**:
  - Sliding window analysis for time-varying topology
  - Time-windowed persistence diagrams
  - Burst synchrony detection

- **Cell Assembly Detection**:
  - Clique topology for functional assemblies
  - Correlation-based connectivity graphs
  - Maximal clique enumeration

- **Functional Connectivity**:
  - Multiple connectivity methods (correlation, Victor-Purpura, van Rossum)
  - Topological feature extraction for machine learning
  - Burst synchrony analysis

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tda = "0.1.0"
nalgebra = "0.33"
```

## Examples

### Basic Persistent Homology

```rust
use nalgebra::DMatrix;
use tda::prelude::*;

// Create a circle point cloud
let n = 20;
let mut points = Vec::new();
for i in 0..n {
    let angle = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
    points.push(angle.cos());
    points.push(angle.sin());
}
let point_matrix = DMatrix::from_row_slice(n, 2, &points);

// Compute distance matrix
let dist = euclidean_distance_matrix(&point_matrix)?;

// Compute persistent homology
let pairs = persistent_homology_rips(&dist, 2.0, 1)?;

// Create persistence diagram
let diagram = PersistenceDiagram::from_pairs(&pairs, Some(1));

// Analyze features
for (birth, death) in diagram.points_by_dimension(1) {
    println!("Loop: birth={:.3}, death={:.3}, persistence={:.3}",
             birth, death, death - birth);
}
```

### Spike Train Analysis

```rust
use tda::distances::SpikeTrain;
use tda::neural_tda::NeuralPopulation;

// Create spike trains for multiple neurons
let train1 = SpikeTrain::new(vec![1.0, 2.5, 4.0, 5.5, 7.0])?;
let train2 = SpikeTrain::new(vec![1.1, 2.6, 4.1, 5.6, 7.1])?;
let train3 = SpikeTrain::new(vec![10.0, 11.0, 12.0, 13.0])?;

// Create neural population
let population = NeuralPopulation::new(vec![train1, train2, train3])?;

// Compute persistence diagram using Victor-Purpura distance
let diagram = population.persistence_diagram_vp(1.0, 5.0, 1)?;

println!("Detected {} topological features", diagram.len());

// Extract topological features for ML
let features = topological_features(&population, 1.0, 5.0, 1)?;
println!("Feature vector: {:?}", features);
```

### Cell Assembly Detection

```rust
use tda::neural_tda::{NeuralPopulation, detect_cell_assemblies};

// Create population with correlated neurons
let population = NeuralPopulation::new(spike_trains)?;

// Detect assemblies using clique topology
let assemblies = detect_cell_assemblies(
    &population,
    0.7,  // correlation threshold
    3     // minimum assembly size
)?;

for (i, assembly) in assemblies.iter().enumerate() {
    println!("Assembly {}: {} neurons, strength={:.3}",
             i, assembly.neurons.len(), assembly.strength);
}
```

### Sliding Window Analysis

```rust
use tda::neural_tda::SlidingWindowAnalysis;

// Configure sliding window
let window = SlidingWindowAnalysis::new(
    1.0,   // window size
    0.5,   // step size
    0.0,   // start time
    10.0   // end time
)?;

// Analyze population over time
let results = window.analyze_population(&population, 1.0, 5.0, 1)?;

for (time, diagram) in results {
    println!("Time {:.2}: {} features", time, diagram.len());
}
```

### Mapper Algorithm

```rust
use tda::mapper::{MapperBuilder, filters, CoverStrategy, ClusteringMethod};

let data = DMatrix::from_row_slice(100, 10, &data_array);
let dist = euclidean_distance_matrix(&data)?;

// Build Mapper graph
let graph = MapperBuilder::new()
    .filter(filters::eccentricity())
    .cover(CoverStrategy::Uniform {
        num_intervals: 10,
        overlap_percent: 0.5,
    })
    .clustering(ClusteringMethod::SingleLinkage { threshold: 0.5 })
    .build(&data, &dist)?;

println!("Mapper graph: {} nodes, {} edges",
         graph.node_count(), graph.edge_count());

// Access node information
for node in graph.node_weights() {
    println!("Cluster: {} points, representative={:?}",
             node.size, node.representative_value);
}
```

### Functional Connectivity

```rust
use tda::neural_tda::functional_connectivity;
use std::collections::HashMap;

// Compute functional connectivity matrix
let mut params = HashMap::new();
params.insert("q".to_string(), 1.0);

let conn_matrix = functional_connectivity(
    &population,
    "victor-purpura",
    &params
)?;

println!("Connectivity matrix shape: {}x{}",
         conn_matrix.nrows(), conn_matrix.ncols());
```

## Mathematical Background

### Persistent Homology

Persistent homology computes topological features (connected components, loops, voids) of data across multiple scales. For a point cloud:

1. Build a filtration (nested sequence of simplicial complexes)
2. Track when features appear (birth) and disappear (death)
3. Long-lived features represent significant topology

**Betti Numbers**: β₀ counts connected components, β₁ counts loops, β₂ counts voids, etc.

### Vietoris-Rips Complex

At scale r, the Rips complex contains:
- All data points as vertices
- An edge between points i,j if d(i,j) ≤ r
- A k-simplex if all its edges exist

### Victor-Purpura Distance

Spike train metric based on edit distance:
- Cost 1 to insert/delete a spike
- Cost q|t₁-t₂| to shift a spike from time t₁ to t₂
- Parameter q sets the timescale (1/q)

### Mapper Algorithm

Creates a topological summary:
1. Apply filter function f: X → ℝᵏ
2. Cover range of f with overlapping sets
3. Cluster points in each cover element
4. Build nerve: nodes are clusters, edges connect overlapping clusters

## Performance

- **Parallel Distance Matrices**: Uses rayon for parallel computation
- **Sparse Representations**: Efficient storage for boundary matrices
- **Clique Enumeration**: Optimized for Rips complex construction
- **Thread-Safe**: All major operations support concurrent execution

## Architecture

```
tda/
├── src/
│   ├── lib.rs                    # Main API and documentation
│   ├── error.rs                  # Error types
│   ├── distances.rs              # Distance metrics
│   ├── simplicial_complex.rs     # Simplicial complex structures
│   ├── persistent_homology.rs    # Homology computation
│   ├── persistence_diagram.rs    # Diagram operations and distances
│   ├── mapper.rs                 # Mapper algorithm
│   └── neural_tda.rs            # Neuroscience-specific tools
├── Cargo.toml
└── README.md
```

## Module Overview

- **error**: Error types using thiserror
- **distances**: Euclidean, correlation, Victor-Purpura, van Rossum
- **simplicial_complex**: Simplex, filtered complex, Rips/Čech construction
- **persistent_homology**: Union-Find (dim 0), matrix reduction (dim ≥1)
- **persistence_diagram**: Birth-death diagrams, bottleneck/Wasserstein distances
- **mapper**: Filter functions, cover strategies, clustering, nerve construction
- **neural_tda**: Spike trains, cell assemblies, connectivity, temporal analysis

## Testing

The library includes comprehensive tests:

```bash
cargo test
```

Tests cover:
- Geometric examples (circles, spheres)
- Spike train distances
- Cell assembly detection
- Mapper on synthetic data
- Persistence diagram distances
- Integration tests

## References

### TDA Theory
- Edelsbrunner, H., & Harer, J. (2010). Computational Topology: An Introduction
- Carlsson, G. (2009). Topology and data. Bulletin of the AMS
- Ghrist, R. (2008). Barcodes: The persistent topology of data

### Neuroscience Applications
- Curto, C. (2017). What can topology tell us about the neural code?
- Giusti, C., et al. (2015). Clique topology reveals intrinsic geometric structure in neural correlations
- Reimann, M. W., et al. (2017). Cliques of neurons bound into cavities provide a missing link between structure and function

### Spike Train Metrics
- Victor, J. D., & Purpura, K. P. (1997). Metric-space analysis of spike trains
- van Rossum, M. C. W. (2001). A novel spike distance

### Mapper Algorithm
- Singh, G., Mémoli, F., & Carlsson, G. E. (2007). Topological methods for the analysis of high dimensional data sets

## License

MIT OR Apache-2.0

## Contributing

This library is part of the CORTEXIA neuroscience workspace. Contributions are welcome!
