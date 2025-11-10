# IIT - Integrated Information Theory 3.0

A complete Rust implementation of Integrated Information Theory (IIT) 3.0, a mathematical framework for quantifying consciousness and integrated information in neural and computational systems.

## Overview

IIT quantifies consciousness as **Φ (Phi)** - integrated information that measures how much a system is irreducible to its parts. This library provides:

- **Complete IIT 3.0 implementation** following Oizumi, Albantakis & Tononi (2014)
- **Multiple approximation methods** for systems of different sizes
- **Parallel computation** using rayon for performance
- **Comprehensive concept identification** and cause-effect structure analysis
- **Flexible API** with builder pattern and type-safe operations

## Features

### Core Functionality

- ✅ **Φ (Phi) Calculation**: Multiple methods from exact to approximate
- ✅ **Concept Identification**: Find all concepts with irreducible cause-effect power
- ✅ **MIP Search**: Minimum Information Partition with parallel algorithms
- ✅ **Cause-Effect Repertoires**: Complete repertoire computation
- ✅ **MICE Analysis**: Maximally Irreducible Cause-Effect for mechanisms
- ✅ **Qualia Space**: Analyze the distribution and structure of concepts

### Approximation Methods

1. **Exact**: Exhaustive search (≤15 elements)
2. **Geometric**: Balduzzi & Tononi (2008) approximation
3. **Spectral**: Eigenvalue-based approximation
4. **Mean Field**: Statistical physics approximation for large systems
5. **Tau (τ)**: Fast connectivity-based measure

### Performance

- **Small systems** (≤10 elements): Exact calculation in milliseconds
- **Medium systems** (10-50 elements): Geometric/spectral approximations
- **Large systems** (>50 elements): Mean field or tau approximations
- **Parallel computation**: Automatic parallelization with rayon

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
iit = "0.1.0"
```

## Quick Start

```rust
use iit::*;

// Create a 3-neuron system
let mut system = IITSystem::new(3);

// Set up all-to-all connectivity
for i in 0..3 {
    for j in 0..3 {
        if i != j {
            system.set_connection(i, j, true).unwrap();
        }
    }
}

// Set system state
system.set_state(vec![1, 0, 1]).unwrap();

// Calculate Φ
let result = system.calculate_phi().unwrap();
println!("Φ = {}", result.phi);
```

## Examples

### Using the Builder Pattern

```rust
use iit::*;

let system = IITSystemBuilder::new(4)
    .state(vec![1, 1, 0, 1])
    .fully_connected()
    .approximation(ApproximationMethod::Geometric)
    .parallel(true)
    .build()
    .unwrap();

let result = system.calculate_phi().unwrap();
```

### Concept Identification

```rust
use iit::*;

let mut system = fully_connected_system(3);
system.set_state(vec![1, 0, 1]).unwrap();

let ces = system.identify_concepts().unwrap();
println!("Found {} concepts", ces.n_concepts());

// Get top concepts by Φ
let top = ces.core(5);
for concept in top {
    println!("Mechanism {:?}: Φ = {:.4}", concept.mechanism, concept.phi);
}
```

### Comparing Approximation Methods

```rust
use iit::*;

let methods = vec![
    ApproximationMethod::Geometric,
    ApproximationMethod::Spectral,
    ApproximationMethod::MeanField,
    ApproximationMethod::Tau,
];

let mut system = fully_connected_system(10);
system.set_state(vec![1; 10]).unwrap();

for method in methods {
    let mut config = PhiConfig::default();
    config.approximation = method;
    system.set_config(config);

    let result = system.calculate_phi().unwrap();
    println!("{:?}: Φ = {:.4}", method, result.phi);
}
```

### Qualia Space Analysis

```rust
use iit::*;

let mut system = fully_connected_system(3);
system.set_state(vec![1, 1, 0]).unwrap();

let qualia = system.analyze_qualia_space().unwrap();
println!("Concepts: {}", qualia.n_concepts);
println!("Mean Φ: {:.4}", qualia.mean_phi);
println!("Max Φ: {:.4}", qualia.max_phi);
```

## Theory Background

### Integrated Information Theory

IIT proposes that consciousness corresponds to integrated information. A system is conscious to the extent that it:

1. **Exists intrinsically**: Has cause-effect power over itself
2. **Is structured**: Has specific cause-effect structure (concepts)
3. **Is integrated**: Cannot be reduced to independent parts
4. **Is definite**: Has specific borders and composition

### Mathematical Framework

**Φ (Phi)** is defined as the distance between:
- The cause-effect structure of the whole system
- The cause-effect structure under the Minimum Information Partition (MIP)

For a mechanism M in state s:
- **Cause repertoire**: P(purview_past | M=s)
- **Effect repertoire**: P(purview_future | M=s)
- **φ**: Distance from unconstrained distribution

### Key Publications

- Tononi, G. (2004). An information integration theory of consciousness. *BMC Neuroscience*, 5(1), 42.
- Balduzzi, D., & Tononi, G. (2008). Integrated information in discrete dynamical systems. *PLOS Computational Biology*, 4(6), e1000091.
- Oizumi, M., Albantakis, L., & Tononi, G. (2014). From the phenomenology to the mechanisms of consciousness: Integrated Information Theory 3.0. *PLOS Computational Biology*, 10(5), e1003588.

## Architecture

### Module Structure

```
iit/
├── src/
│   ├── lib.rs           # Main API and IITSystem
│   ├── phi.rs           # Φ calculation methods
│   ├── partition.rs     # MIP search and partition enumeration
│   ├── repertoire.rs    # Cause and effect repertoires
│   ├── causality.rs     # Cause-effect structure
│   ├── concepts.rs      # Concept identification
│   ├── emd.rs          # Earth Mover's Distance
│   └── error.rs        # Error types
├── tests/              # Integration tests
├── benches/            # Performance benchmarks
└── examples/           # Usage examples
```

### Core Types

- **`IITSystem`**: Main system representation
- **`PhiResult`**: Results of Φ calculation
- **`Concept`**: A mechanism with cause-effect power
- **`CauseEffectStructure`**: Constellation of concepts
- **`Repertoire`**: Probability distribution over states
- **`MICE`**: Maximally irreducible cause-effect

## Performance Considerations

### System Size Guidelines

- **≤10 elements**: Use exact calculation
- **10-20 elements**: Use exact or geometric approximation
- **20-100 elements**: Use geometric or spectral approximation
- **>100 elements**: Use mean field or tau approximation

### Optimization Tips

1. **Enable parallel computation**: `config.parallel = true`
2. **Choose appropriate approximation**: Balance accuracy vs speed
3. **Filter concepts**: Use `min_phi` threshold to reduce computation
4. **Cache results**: The library includes built-in caching

## Testing

Run tests:

```bash
cargo test
```

Run benchmarks:

```bash
cargo bench
```

Run examples:

```bash
cargo run --example basic_usage
```

## Limitations and Future Work

### Current Limitations

- Binary states only (continuous states planned)
- Simplified TPM indexing (full state-by-node format planned)
- EMD uses L1 approximation (full EMD solver planned)
- Large systems require approximations (inherent to IIT complexity)

### Planned Features

- [ ] Continuous state support
- [ ] GPU acceleration for large systems
- [ ] Full EMD implementation with linear programming
- [ ] IIT 4.0 updates
- [ ] Visualization tools
- [ ] Network generation utilities

## Contributing

Contributions are welcome! Areas of interest:

- Performance optimizations
- Additional approximation methods
- Visualization tools
- Documentation improvements
- Test cases from IIT literature

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Citation

If you use this library in academic work, please cite:

```bibtex
@software{iit_rust,
  author = {Francisco Molina Burgos and Claude-AMINORINO},
  title = {IIT: Integrated Information Theory 3.0 in Rust},
  year = {2024},
  url = {https://github.com/aminorino/iit}
}
```

## References

1. Oizumi, M., Albantakis, L., & Tononi, G. (2014). From the phenomenology to the mechanisms of consciousness: Integrated Information Theory 3.0. *PLOS Computational Biology*, 10(5), e1003588.

2. Balduzzi, D., & Tononi, G. (2008). Integrated information in discrete dynamical systems: Motivation and theoretical framework. *PLOS Computational Biology*, 4(6), e1000091.

3. Tononi, G. (2004). An information integration theory of consciousness. *BMC Neuroscience*, 5(1), 42.

4. Tononi, G., Boly, M., Massimini, M., & Koch, C. (2016). Integrated information theory: from consciousness to its physical substrate. *Nature Reviews Neuroscience*, 17(7), 450-461.

## Acknowledgments

This implementation is based on the theoretical work of Giulio Tononi and collaborators at the University of Wisconsin-Madison. We thank the IIT community for their open discussion of the theory and its implementation challenges.
