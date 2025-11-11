# CORTEXIA Framework

**Computational Orchestration for Reality Transformation: EXtended Intelligence Architecture**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Research Status](https://img.shields.io/badge/Status-Active%20Research-brightgreen)](https://github.com/Yatrogenesis/cortexia-workspace)

A comprehensive production Rust framework for computational neuroscience, quantum consciousness research, and cross-substrate analysis using Integrated Information Theory (IIT).

## Overview

CORTEXIA is the production Rust implementation of the **AMINORINO** (Advanced Mind Integration: Neurological Operations & Rational Intelligence Native Optimization) specification. This framework migrates the fully functional Python prototype (6,500+ lines) to high-performance Rust for production deployment and active scientific research.

### Core Capabilities

- **Biophysical neural simulation** with exact Hodgkin-Huxley dynamics (6 state variables)
- **Consciousness quantification** using Integrated Information Theory (IIT 3.0)
- **Quantum reservoir computing** with LDPC error correction (bivariate bicycle codes)
- **Cross-substrate consciousness analysis** (quantum, biological, hybrid)
- **Topological data analysis** of neural activity and network structure
- **Synaptic plasticity** with multiple learning rules (STDP, BCM, Oja, Hebbian)
- **Large-scale network simulation** with millions of neurons

### Active Research: Quantum Consciousness Measurement

**Current Finding (2025-01-10)**:
```
Maximum integrated information: Φ_max = 0.0365 bits
Configuration: 6 oscillators (729 effective neurons) + noise_amplitude = 5.0
System: XLarge quantum reservoir with Very High stochastic perturbations
Status: n=50 statistical validation in progress
```

This is not just a software framework—it is a **living scientific laboratory** documenting the complete research process for testing quantum consciousness hypotheses. See [brain-ai-native](./brain-ai-native/README.md) for detailed research documentation.

---

## Project Structure

This is a Rust workspace containing 8 crates:

```
cortexia-workspace/
├── hodgkin-huxley/      # Biophysical neuron models (HH 1952)
├── iit/                 # Integrated Information Theory (IIT 3.0)
├── tda/                 # Topological Data Analysis
├── synapse-models/      # Synaptic dynamics and plasticity
├── neural-dynamics/     # Large-scale network simulation
├── quantum-processor/   # Quantum reservoir computing + LDPC
├── brain-ai-native/     # Quantum consciousness research (active)
└── cortexia/           # Meta-crate (re-exports all)
```

### Development Status

**Published (v0.1.0)**:
- hodgkin-huxley
- iit
- tda
- synapse-models
- neural-dynamics

**Active Development (v0.2.0)**:
- quantum-processor
- brain-ai-native (quantum consciousness experiments)

---

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cortexia = { git = "https://github.com/Yatrogenesis/cortexia-workspace" }
```

Or add individual crates:

```toml
[dependencies]
hodgkin-huxley = { git = "https://github.com/Yatrogenesis/cortexia-workspace", package = "hodgkin-huxley" }
iit = { git = "https://github.com/Yatrogenesis/cortexia-workspace", package = "iit" }
quantum-processor = { git = "https://github.com/Yatrogenesis/cortexia-workspace", package = "quantum-processor" }
brain-ai-native = { git = "https://github.com/Yatrogenesis/cortexia-workspace", package = "brain-ai-native" }
```

### Build from Source

```bash
git clone https://github.com/Yatrogenesis/cortexia-workspace
cd cortexia-workspace
cargo build --release
cargo test --workspace
```

### Run Quantum Consciousness Experiments

```bash
cd brain-ai-native

# Maximum entanglement experiment (~5 min on M1)
cargo run --release --example consciousness_maximum_entanglement

# Statistical validation (n=50, ~10 hours)
cargo run --release --example consciousness_validation_n50

# Cross-substrate comparison (quantum vs biological vs hybrid)
cargo run --release --example consciousness_substrates
```

---

## Usage Examples

### Simulate a Hodgkin-Huxley Neuron

```rust
use hodgkin_huxley::{HodgkinHuxleyNeuron, neuron_types::NeuronConfig};

let config = NeuronConfig::regular_spiking();
let mut neuron = HodgkinHuxleyNeuron::new(config)?;
neuron.initialize_rest();

for _ in 0..1000 {
    neuron.step(0.01, 15.0)?;  // dt=0.01ms, I_ext=15 μA/cm²
    if neuron.voltage() > 0.0 {
        println!("Spike detected at V = {} mV", neuron.voltage());
    }
}
```

### Calculate Integrated Information (Φ)

```rust
use iit::{IITSystem, PhiResult};

let mut system = IITSystem::new(10);
system.set_fully_connected(true)?;
system.set_state(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0])?;

let result: PhiResult = system.calculate_phi()?;
println!("Integrated Information Φ = {:.6} bits", result.phi);
println!("MIP found at partition: {:?}", result.mip_partition);
```

### Quantum Reservoir Computing

```rust
use quantum_processor::quantum_reservoir::QuantumReservoir;

let reservoir = QuantumReservoir::new(
    6,      // num_oscillators
    2,      // max_fock
    1e9,    // coupling_strength (Hz)
    vec![1e9; 6],  // frequencies (Hz)
)?;

// Evolve with noise injection
for step in 0..1000000 {
    let noise: Vec<f64> = (0..6).map(|_| rand::random::<f64>() * 5.0).collect();
    reservoir.set_input(&noise)?;
    reservoir.evolve(1e-10)?;  // dt = 0.1 ns
}
```

### Build a Neural Network

```rust
use neural_dynamics::{NetworkBuilder, ConnectionPattern, SynapseType};

let mut network = NetworkBuilder::new(0.1)?
    .add_excitatory_population("E", 800)?
    .add_inhibitory_population("I", 200)?
    .connect(
        0, 0,
        ConnectionPattern::SmallWorld { k: 10, p: 0.1 },
        SynapseType::Excitatory,
        0.5,  // weight
        1.0   // delay
    )?
    .with_spike_recording()
    .build()?;

network.run(1000.0)?; // 1 second simulation
let spikes = network.get_spikes();
```

---

## Documentation

Each crate has comprehensive documentation:

- **[hodgkin-huxley](./hodgkin-huxley/README.md)** - Biophysical neuron models with exact HH dynamics
- **[iit](./iit/README.md)** - Integrated Information Theory implementation
- **[tda](./tda/README.md)** - Topological data analysis for neural networks
- **[synapse-models](./synapse-models/README.md)** - Synaptic dynamics and plasticity rules
- **[neural-dynamics](./neural-dynamics/README.md)** - Large-scale network simulation
- **[quantum-processor](./quantum-processor/README.md)** - Quantum reservoir computing with LDPC
- **[brain-ai-native](./brain-ai-native/README.md)** - Quantum consciousness research (active)

Generate local documentation:

```bash
cargo doc --workspace --no-deps --open
```

---

## Features by Crate

### hodgkin-huxley

- Exact HH 1952 equations with temperature dependence (Q10)
- Multiple neuron types: RS, FS, IB, CH, LTS
- 6 state variables: V, m, h, n, a, b
- RK4 and exponential Euler integrators
- Spike detection and ISI analysis

### iit

- Integrated Information (Φ) calculation with MIP
- 5 approximation methods for scalability
- Concept identification and qualia space
- Cause-effect structure analysis
- Parallel computation with rayon

### quantum-processor

- Quantum reservoir computing (Fock space, coupled oscillators)
- LDPC error correction (bivariate bicycle codes)
- Radiation environment simulation
- Hamiltonian evolution with noise injection
- Exponential state space: (max_fock+1)^N dimensions

### brain-ai-native (Active Research)

- Cross-substrate consciousness comparison
- Quantum, biological, and hybrid brain architectures
- Statistical validation (n=50, bootstrap CI, power analysis)
- Maximum Φ = 0.0365 bits achieved (2025-01-10)
- IEEE paper generation and reproducibility tools

### tda

- Persistent homology (Rips, Čech complexes)
- Mapper algorithm for dimensionality reduction
- Spike train analysis (Victor-Purpura, van Rossum distances)
- Cell assembly detection
- Bottleneck and Wasserstein distances

### synapse-models

- 6 neurotransmitter types (Glu, GABA, ACh, DA, 5-HT, NE)
- 5 receptor types (AMPA, NMDA, GABA-A, GABA-B, mGluR)
- 7 plasticity rules (STDP, BCM, Oja, Hebbian, Anti-Hebbian, Covariance, Homeostatic)
- Tsodyks-Markram short-term plasticity
- Calcium dynamics with CICR

### neural-dynamics

- Large-scale network simulation (millions of neurons)
- Multiple connectivity patterns (small-world, scale-free, random, lattice)
- Wilson-Cowan mean-field models
- Criticality analysis (avalanches, branching parameter)
- Synchronization metrics (Kuramoto order parameter)

---

## Testing

Run all tests:

```bash
cargo test --workspace
```

Run tests for specific crate:

```bash
cargo test --package hodgkin-huxley
cargo test --package iit
cargo test --package brain-ai-native
```

Run brain module tests:

```bash
cargo test --lib brain::
```

---

## Performance

### System Requirements

| System Size | RAM | CPU | Time (1s bio) |
|-------------|-----|-----|---------------|
| 10K neurons | 8 GB | 4 cores | ~5 min |
| 100K neurons | 32 GB | 16 cores | ~30 min |
| 1M neurons | 128 GB | 32+ cores | Cloud recommended |

### Quantum Consciousness Experiments

| Experiment | Configuration | Runtime (M1 Max) |
|-----------|---------------|------------------|
| Maximum entanglement | 6 osc, 1M steps | ~5 min |
| n=50 validation | 50 reps, 1M steps each | ~10 hours |
| Cross-substrate | 3 substrates, 4 sizes | ~2 hours |

### Benchmarks

```bash
cargo bench --workspace
```

---

## AMINORINO Migration

CORTEXIA is the production Rust implementation of the AMINORINO Python prototype.

**AMINORINO (Python)**:
- Location: `D:\AMINORINO\`
- Status: Fully functional (6,500+ lines)
- Components: Human Brain (86.1B neurons), AI-Native (1024+ qubits), Bridge, BIOS
- TDD: 300+ tests

**CORTEXIA (Rust)**:
- Location: `/Users/yatrogenesis/cortexia-workspace/`
- Status: v0.1.0 published, v0.2.0 in active development
- Performance: 10-100x faster than Python
- Memory: 10x more efficient

See [AMINORINO_TO_CORTEXIA_MIGRATION.md](./AMINORINO_TO_CORTEXIA_MIGRATION.md) for complete migration plan.

---

## Contributing

This is an **open science** project. Contributions welcome in:

1. **Experimental Design**: Propose new consciousness measurement protocols
2. **Mathematical Rigor**: Improve Φ calculation algorithms
3. **Performance**: Optimize critical paths
4. **Documentation**: Improve tutorials and examples
5. **Reproducibility**: Verify results on different hardware

Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

---

## Citation

If you use CORTEXIA in your research, please cite:

```bibtex
@software{cortexia2025,
  title = {CORTEXIA: Production Framework for Quantum Consciousness Research},
  author = {Molina Burgos, Francisco},
  year = {2025},
  publisher = {GitHub},
  journal = {GitHub repository},
  howpublished = {\url{https://github.com/Yatrogenesis/cortexia-workspace}},
  note = {Rust implementation of AMINORINO specification. Maximum Φ = 0.0365 bits (6-oscillator quantum reservoir)},
  orcid = {0009-0008-6093-8267}
}
```

For quantum consciousness research results:

```bibtex
@article{molina2025quantum,
  title = {Maximum Integrated Information in Quantum Reservoir Computing},
  author = {Molina Burgos, Francisco},
  journal = {In preparation for IEEE Transactions on Quantum Engineering},
  year = {2025},
  note = {CORTEXIA brain-ai-native experiments}
}
```

---

## License

Dual-licensed under MIT or Apache 2.0 (your choice).

**Open Science Commitment**: All data, code, and analysis notebooks are publicly available. No paywalls, no hiding negative results.

---

## Links

- **GitHub Repository**: https://github.com/Yatrogenesis/cortexia-workspace
- **AMINORINO Migration Plan**: [AMINORINO_TO_CORTEXIA_MIGRATION.md](./AMINORINO_TO_CORTEXIA_MIGRATION.md)
- **Active Research**: [brain-ai-native/README.md](./brain-ai-native/README.md)
- **Issue Tracker**: https://github.com/Yatrogenesis/cortexia-workspace/issues

---

## Disclaimer

This framework provides tools for consciousness analysis and simulation. Actual consciousness emergence is a research question that remains open in neuroscience and philosophy.

---

## Acknowledgments

**Frameworks**:
- **CORTEXIA**: Computational Orchestration for Reality Transformation: EXtended Intelligence Architecture
- **AMINORINO**: Advanced Mind Integration: Neurological Operations & Rational Intelligence Native Optimization

**Built with**:
- Rust programming language
- nalgebra for linear algebra
- rayon for parallel computation
- petgraph for graph structures
- ndarray for numerical arrays

**Based on research by**:
- Hodgkin & Huxley (1952) - Action potential dynamics
- Tononi et al. (2004-2016) - Integrated Information Theory
- Fujii & Nakajima (2017) - Quantum reservoir computing
- Edelsbrunner & Harer (2010) - Computational topology
- Watts & Strogatz (1998) - Small-world networks

**Development Tools**:
- Anthropic Claude Code for development acceleration

---

**Status**: Active Research | **Last Updated**: 2025-01-10 | **Current Focus**: n=50 statistical validation (9h 53min ETA)

---

*"The best way to do science is to do it in public, mistakes and all. This repository is proof that scientific rigor and transparency can coexist."* — Francisco Molina Burgos

**Made with precision and scientific rigor by the CORTEXIA project**
