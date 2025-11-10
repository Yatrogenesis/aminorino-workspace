# AMINORINO Framework

**Advanced Mind Integration: Neurological Operations & Rational Intelligence Native Optimization**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A comprehensive Rust framework for computational neuroscience, consciousness analysis, and neural-quantum bridge architectures.

## 🧠 Overview

AMINORINO provides a complete toolkit for:

- **Biophysical neural simulation** with exact Hodgkin-Huxley dynamics
- **Consciousness quantification** using Integrated Information Theory (IIT 3.0)
- **Topological analysis** of neural activity and network structure
- **Synaptic plasticity** with multiple learning rules
- **Large-scale network simulation** with millions of neurons

## 📦 Project Structure

This is a Rust workspace containing 6 crates:

```
aminorino-workspace/
├── hodgkin-huxley/      # Biophysical neuron models
├── iit/                 # Integrated Information Theory
├── tda/                 # Topological Data Analysis
├── synapse-models/      # Synaptic dynamics
├── neural-dynamics/     # Network simulation
└── aminorino/           # Meta-crate (re-exports all)
```

## 🚀 Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
aminorino = { git = "https://github.com/aminorino/aminorino" }
```

Or add individual crates:

```toml
[dependencies]
hodgkin-huxley = { git = "https://github.com/aminorino/hodgkin-huxley" }
iit = { git = "https://github.com/aminorino/iit" }
tda = { git = "https://github.com/aminorino/tda" }
synapse-models = { git = "https://github.com/aminorino/synapse-models" }
neural-dynamics = { git = "https://github.com/aminorino/neural-dynamics" }
```

### Build from Source

```bash
git clone https://github.com/aminorino/aminorino-workspace
cd aminorino-workspace
cargo build --release
cargo test --all
```

## 💡 Usage Examples

### Simulate a Neuron

```rust
use aminorino::prelude::*;

let mut neuron = HodgkinHuxleyNeuron::regular_spiking();
for _ in 0..1000 {
    neuron.integrate_rk4(15.0, 0.01).unwrap();
    if neuron.detect_spike() {
        println!("Spike detected!");
    }
}
```

### Calculate Consciousness (Φ)

```rust
use aminorino::prelude::*;

let mut system = IITSystem::new(10);
system.set_fully_connected(true).unwrap();
system.set_state(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0]).unwrap();

let result = system.calculate_phi().unwrap();
println!("Integrated Information Φ = {:.4}", result.phi);
```

### Build a Neural Network

```rust
use aminorino::prelude::*;

let mut network = NetworkBuilder::new(0.1)?
    .add_excitatory_population("E", 800)?
    .add_inhibitory_population("I", 200)?
    .connect(0, 0, ConnectionPattern::SmallWorld { k: 10, p: 0.1 },
             SynapseType::Excitatory, 0.5, 1.0)?
    .with_spike_recording()
    .build();

network.run(1000.0)?; // 1 second simulation
```

## 📚 Documentation

Each crate has comprehensive documentation:

- **[hodgkin-huxley](./hodgkin-huxley/README.md)** - Neuron models with exact biophysics
- **[iit](./iit/README.md)** - Consciousness quantification
- **[tda](./tda/README.md)** - Topological analysis
- **[synapse-models](./synapse-models/README.md)** - Synaptic dynamics
- **[neural-dynamics](./neural-dynamics/README.md)** - Network simulation

Generate local documentation:

```bash
cargo doc --workspace --no-deps --open
```

## 🔬 Features

### Hodgkin-Huxley (`hodgkin-huxley`)

- ✅ Exact HH 1952 equations with temperature dependence
- ✅ Multiple neuron types (RS, FS, IB, CH, LTS)
- ✅ 6 state variables (V, m, h, n, a, b)
- ✅ RK4 and exponential Euler integrators
- ✅ Spike detection and analysis

### IIT (`iit`)

- ✅ Integrated Information (Φ) calculation
- ✅ 5 approximation methods for different scales
- ✅ Concept identification and qualia space
- ✅ Cause-effect structure analysis
- ✅ Parallel computation support

### TDA (`tda`)

- ✅ Persistent homology (Rips, Čech complexes)
- ✅ Mapper algorithm
- ✅ Spike train analysis (Victor-Purpura, van Rossum)
- ✅ Cell assembly detection
- ✅ Bottleneck and Wasserstein distances

### Synapse Models (`synapse-models`)

- ✅ 6 neurotransmitter types
- ✅ 5 receptor types (AMPA, NMDA, GABA-A, GABA-B, mGluR)
- ✅ 7 plasticity rules (STDP, BCM, Oja, Hebbian, Homeostatic)
- ✅ Tsodyks-Markram short-term plasticity
- ✅ Calcium dynamics with CICR

### Neural Dynamics (`neural-dynamics`)

- ✅ Large-scale network simulation
- ✅ Multiple connectivity patterns (small-world, scale-free, etc.)
- ✅ Wilson-Cowan mean-field models
- ✅ Criticality analysis (avalanches, branching parameter)
- ✅ Synchronization metrics (Kuramoto order parameter)

## 🧪 Testing

Run all tests:

```bash
cargo test --workspace
```

Run tests for specific crate:

```bash
cargo test --package hodgkin-huxley
cargo test --package iit
cargo test --package tda
```

## 📊 Performance

### System Requirements

| System Size | RAM | CPU | Time (1s bio) |
|-------------|-----|-----|---------------|
| 10K neurons | 8 GB | 4 cores | ~5 min |
| 100K neurons | 32 GB | 16 cores | ~30 min |
| 1M neurons | 128 GB | 32+ cores | Cloud recommended |

### Benchmarks

```bash
cargo bench --workspace
```

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## 📖 Citation

If you use AMINORINO in your research, please cite:

```bibtex
@software{aminorino2025,
  title = {AMINORINO: Advanced Mind Integration Framework},
  author = {Molina Burgos, Francisco and Claude-AMINORINO},
  year = {2025},
  url = {https://github.com/aminorino/aminorino}
}
```

## 📄 License

Licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

## 🔗 Links

- **GitHub Organization**: https://github.com/aminorino
- **Documentation**: (Will be available on docs.rs after publication)
- **Issue Tracker**: https://github.com/aminorino/aminorino-workspace/issues

## ⚠️ Disclaimer

This framework provides tools for consciousness *analysis* and *simulation*. Actual consciousness emergence is a research question that remains open in neuroscience and philosophy.

## 🙏 Acknowledgments

Built with:
- Rust programming language
- nalgebra for linear algebra
- rayon for parallel computation
- petgraph for graph structures
- And many other excellent Rust crates

Based on research by:
- Hodgkin & Huxley (1952) - Action potential dynamics
- Tononi et al. (2004-2016) - Integrated Information Theory
- Edelsbrunner & Harer (2010) - Computational topology
- Watts & Strogatz (1998) - Small-world networks
- Many other pioneering neuroscientists

---

**Made with 🧠 and ⚡ by the AMINORINO project**
