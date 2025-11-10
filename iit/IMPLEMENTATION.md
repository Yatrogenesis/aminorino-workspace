# IIT Library Implementation Details

This document provides technical details about the implementation of the Integrated Information Theory (IIT) 3.0 library in Rust.

## Architecture Overview

### Module Organization

```
iit/
├── src/
│   ├── lib.rs           # Main API, IITSystem, builders
│   ├── error.rs         # Error types and Result alias
│   ├── partition.rs     # Partition enumeration and MIP search
│   ├── repertoire.rs    # Cause/effect repertoire computation
│   ├── emd.rs          # Earth Mover's Distance calculations
│   ├── phi.rs          # Φ calculation with approximations
│   ├── causality.rs    # Cause-effect structure analysis
│   └── concepts.rs     # Concept identification
├── tests/              # Integration tests
├── benches/            # Performance benchmarks
└── examples/           # Usage examples
```

## Core Components

### 1. Error Handling (`error.rs`)

**Design**: Comprehensive error types using `thiserror` for all operations.

**Key Features**:
- Typed errors for different failure modes
- Contextual error messages
- Result type alias for convenience

**Error Types**:
- `InvalidTPM`: Malformed transition probability matrix
- `InvalidConnectivity`: Invalid connectivity matrix
- `InvalidState`: State specification errors
- `DimensionMismatch`: Matrix dimension errors
- `NumericalInstability`: Numerical computation issues
- `SystemTooLarge`: System exceeds computational limits

### 2. Partition Enumeration (`partition.rs`)

**Design**: Efficient partition generation with parallel MIP search.

**Key Structures**:
- `Partition`: Bipartition with cut type
- `PartitionInfo`: Partition with Φ value
- `BipartitionIterator`: Iterator over all bipartitions

**Algorithms**:
- Binary representation for efficient enumeration
- Parallel search using rayon for MIP finding
- Heuristic search for large systems (>15 elements)
- Caching mechanism for intermediate results

**Complexity**:
- Exact: O(2^n) partitions for n elements
- Heuristic: O(k) where k is sample size

### 3. Repertoire Computation (`repertoire.rs`)

**Design**: Probability distribution computation with caching.

**Key Structures**:
- `Repertoire`: Multi-dimensional probability distribution
- `Direction`: Cause or Effect
- `RepertoireCache`: LRU cache for computed repertoires

**Features**:
- Uniform (unconstrained) repertoire generation
- Entropy calculation
- Marginalization over subsets
- Validation of probability distributions

**Implementation Notes**:
- Uses ndarray for efficient multi-dimensional arrays
- Supports both cause and effect directions
- Automatic normalization

### 4. Earth Mover's Distance (`emd.rs`)

**Design**: Distance metrics for probability distributions.

**Implemented Metrics**:
- L1 distance (total variation)
- Kullback-Leibler divergence
- Jensen-Shannon divergence
- Greedy EMD approximation
- Hamming distance for discrete states

**Notes**:
- L1 used as EMD approximation for efficiency
- Full EMD requires linear programming solver (future work)
- KL and JS divergence for alternative measures

### 5. Φ Calculation (`phi.rs`)

**Design**: Multiple approximation methods for different system sizes.

**Approximation Methods**:

#### Exact (≤15 elements)
- Exhaustive partition search
- Complete repertoire computation
- EMD between whole and partitioned systems

#### Geometric (Balduzzi & Tononi 2008)
```rust
φ_geometric = Σ √(in_degree(i) * out_degree(i)) / n
```
- Fast approximation based on connectivity
- Good for qualitative analysis

#### Spectral
```rust
φ_spectral = λ_max - λ_min
```
- Based on eigenvalue spectrum
- Uses spectral gap as proxy for integration

#### Mean Field
```rust
φ_mf = n * mean_activation * coupling * (1 - mean_activation)
```
- Statistical physics approximation
- Suitable for large systems

#### Tau (τ)
```rust
τ = bidirectional_connections / total_possible_connections
```
- Fastest approximation
- Based purely on connectivity structure

**Configuration**:
```rust
pub struct PhiConfig {
    pub max_exact_size: usize,        // Threshold for exact vs approximate
    pub approximation: ApproximationMethod,
    pub cut_type: CutType,            // Uni- or bidirectional cuts
    pub parallel: bool,               // Enable parallelization
    pub cache_size: usize,            // Cache size for results
}
```

### 6. Causality Analysis (`causality.rs`)

**Design**: Cause-effect structure computation.

**Key Structures**:
- `CauseEffect`: Mechanism's cause-effect pair
- `MIC`: Maximally Irreducible Cause
- `MIE`: Maximally Irreducible Effect
- `MICE`: Combined MIC and MIE

**Algorithms**:
- Exhaustive purview search for small systems
- Parallel computation of φ_cause and φ_effect
- EMD-based distance from unconstrained repertoire

**Complexity**:
- O(2^n) purviews for n elements
- Parallelized across purviews

### 7. Concept Identification (`concepts.rs`)

**Design**: Full concept identification with filtering.

**Key Structures**:
- `Concept`: Mechanism with MICE
- `CauseEffectStructure`: Constellation of concepts
- `ConceptConfig`: Configuration for identification
- `QualiaSpace`: Statistical analysis of concepts

**Features**:
- Threshold-based filtering
- Size-based filtering
- Core concept extraction
- Statistical analysis (mean, std, distribution)

**Performance**:
- Parallel mechanism evaluation
- Early filtering by min_phi
- Optional max_mechanism_size limit

### 8. Main API (`lib.rs`)

**Design**: High-level API with builder pattern.

**Core Types**:
```rust
pub struct IITSystem {
    n_elements: usize,
    state: Vec<usize>,
    tpm: ArrayD<f64>,
    connectivity: Vec<Vec<bool>>,
    config: PhiConfig,
}
```

**Builder Pattern**:
```rust
IITSystemBuilder::new(n)
    .state(vec![...])
    .fully_connected()
    .approximation(method)
    .parallel(true)
    .build()
```

**Helper Functions**:
- `fully_connected_system(n)`: All-to-all connectivity
- `feedforward_system(n)`: Chain connectivity

## Performance Optimizations

### 1. Parallelization

**Using rayon**:
- Partition search parallelized
- Concept identification parallelized
- Purview search parallelized

**Thread Safety**:
- Arc/Mutex for shared state
- Send/Sync bounds on closures
- No global mutable state

### 2. Caching

**Repertoire Cache**:
- LRU eviction policy
- Keyed by (mechanism, state, purview, direction)
- Configurable size

**Partition Cache**:
- HashMap-based storage
- Simple eviction when full

### 3. Early Termination

**Filtering**:
- min_phi threshold to skip low-Φ concepts
- max_mechanism_size to limit search space

**Heuristics**:
- Connectivity-based pruning
- State-based optimization

## Mathematical Details

### State Representation

**Binary States**:
- States represented as Vec<usize> with values 0 or 1
- State indices computed via binary encoding
- Efficient bit manipulation

### TPM Format

**Multi-dimensional Array**:
- Shape: [2, 2, ..., 2] (2*n dimensions)
- Indexed by current state (n dims) + future state (n dims)
- Probability at index [s1, s2, ..., sn, s1', s2', ..., sn']

### Connectivity Matrix

**Boolean Matrix**:
- connectivity[i][j] = true if i→j connection exists
- Asymmetric (directed graph)
- No self-connections assumed

## Testing Strategy

### Unit Tests

**Each Module**:
- Basic functionality tests
- Edge case handling
- Error condition testing

**Coverage**:
- Error type creation
- Partition validity
- Repertoire computation
- Distance metrics
- Approximation methods

### Integration Tests

**Test Cases**:
- 3-neuron majority gate (IIT classic)
- Disconnected systems (Φ = 0)
- Single elements (Φ = 0)
- Feedforward vs recurrent
- All approximation methods
- Concept filtering
- State space exploration

**Known Test Cases**:
- Validated against IIT literature examples
- Comparison with other implementations (where available)

### Benchmarks

**Performance Metrics**:
- Φ calculation time vs system size
- Approximation method comparison
- Concept identification scaling
- Partition enumeration speed

## Limitations and Future Work

### Current Limitations

1. **Binary States Only**
   - No continuous states
   - Future: Gaussian approximation for continuous

2. **Simplified TPM**
   - Limited indexing scheme
   - Future: Full state-by-node format

3. **EMD Approximation**
   - L1 instead of true EMD
   - Future: Linear programming solver integration

4. **Memory Usage**
   - Full TPM storage for all systems
   - Future: Sparse representation for large systems

### Planned Enhancements

1. **Continuous States**
   - Multivariate Gaussian repertoires
   - Differential entropy calculations

2. **GPU Acceleration**
   - CUDA kernels for partition search
   - Parallel repertoire computation

3. **Advanced Approximations**
   - Variational methods
   - Monte Carlo sampling
   - Neural network approximators

4. **IIT 4.0**
   - Updates to latest IIT version
   - Compositional structure

5. **Visualization**
   - Concept space visualization
   - Qualia structure plots
   - State space diagrams

## References

### IIT Theory

1. Oizumi, M., Albantakis, L., & Tononi, G. (2014). From the phenomenology to the mechanisms of consciousness: Integrated Information Theory 3.0. PLOS Computational Biology, 10(5), e1003588.

2. Balduzzi, D., & Tononi, G. (2008). Integrated information in discrete dynamical systems: Motivation and theoretical framework. PLOS Computational Biology, 4(6), e1000091.

### Implementation Techniques

1. Rayon parallel processing: https://github.com/rayon-rs/rayon
2. ndarray multi-dimensional arrays: https://github.com/rust-ndarray/ndarray
3. nalgebra linear algebra: https://github.com/dimforge/nalgebra

## Contributing Guidelines

### Code Style

- Follow Rust standard style (rustfmt)
- Document all public APIs
- Include examples in documentation
- Write unit tests for new features

### Performance

- Benchmark new approximation methods
- Profile before optimizing
- Maintain parallel safety
- Consider memory usage

### Documentation

- Update README for user-facing changes
- Update IMPLEMENTATION.md for internals
- Include theory background in module docs
- Provide examples for complex features
