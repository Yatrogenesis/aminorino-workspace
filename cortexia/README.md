# CORTEXIA

**Computational Orchestration for Reality Transformation: EXtended Intelligence Architecture**

A complete Rust framework for computational neuroscience and consciousness research.

[![Crates.io](https://img.shields.io/crates/v/cortexia.svg)](https://crates.io/crates/cortexia)
[![Documentation](https://docs.rs/cortexia/badge.svg)](https://docs.rs/cortexia)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Overview

CORTEXIA is a meta-framework that unifies six production-grade Rust libraries for neuroscience research:

- **hodgkin-huxley** - Biophysical neuron models
- **iit** - Integrated Information Theory 3.0
- **tda** - Topological Data Analysis for neural networks
- **synapse-models** - Synaptic dynamics and plasticity
- **neural-dynamics** - Large-scale network simulation

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
cortexia = "0.1.0"
```

## Example

```rust
use cortexia::prelude::*;

fn main() {
    // Create a Hodgkin-Huxley neuron
    let mut neuron = HodgkinHuxleyNeuron::new();

    // Calculate integrated information (Phi)
    let mut iit_system = IITSystem::new(3);
    iit_system.set_state(vec![1, 0, 1]).unwrap();
    let phi_result = iit_system.calculate_phi().unwrap();
    println!("Φ = {}", phi_result.phi);

    // Build a neural network
    let network = NetworkBuilder::new()
        .populations(2, 100)
        .build();
}
```

## Features

### Biophysical Accuracy
- Exact Hodgkin-Huxley equations (6-state model)
- AMPA, NMDA, GABA receptor kinetics
- Neurotransmitter systems (DA, 5-HT, ACh, NE)

### Consciousness Research
- IIT 3.0 with 5 Phi approximation methods
- Cause-effect repertoires
- Concept identification
- Qualia space analysis

### Topological Analysis
- Persistent homology (Rips & Čech)
- Mapper algorithm
- Spike train analysis
- Cell assembly detection

### Synaptic Dynamics
- STDP, BCM, Oja plasticity rules
- Triplet and voltage-dependent plasticity
- Short-term dynamics
- Vesicle pool modeling

### Network Simulation
- Multiple connectivity patterns (random, small-world, scale-free)
- Wilson-Cowan and Kuramoto dynamics
- Oscillation and synchronization analysis
- Parallel computation support

## Documentation

Full documentation available at [docs.rs/cortexia](https://docs.rs/cortexia)

## Individual Crates

Each component is also available as a standalone crate:

- [`hodgkin-huxley`](https://crates.io/crates/hodgkin-huxley)
- [`iit`](https://crates.io/crates/iit)
- [`tda`](https://crates.io/crates/tda)
- [`synapse-models`](https://crates.io/crates/synapse-models)
- [`neural-dynamics`](https://crates.io/crates/neural-dynamics)

## Performance

- Parallelized computations with `rayon`
- Efficient linear algebra with `nalgebra` and `ndarray`
- Optimized graph algorithms with `petgraph`
- Zero-cost abstractions

## Applications

- Computational neuroscience research
- Consciousness quantification (IIT)
- Brain-computer interfaces
- Cognitive modeling
- AI safety research
- Neural coding analysis

## License

Dual-licensed under MIT or Apache 2.0

## Citation

```bibtex
@software{cortexia2025,
  author = {Molina Burgos, Francisco},
  title = {CORTEXIA: Computational Orchestration for Reality Transformation},
  year = {2025},
  url = {https://github.com/Yatrogenesis/cortexia-workspace}
}
```

## Contributing

Contributions welcome! Please see the main repository at [github.com/Yatrogenesis/cortexia-workspace](https://github.com/Yatrogenesis/cortexia-workspace)

## Acknowledgments

Developed with Claude Code by Anthropic.

---

**CORTEXIA** - Bridging neuroscience, consciousness, and computation. 🧠⚡
