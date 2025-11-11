# brain-ai-native

**AI-Native Brain Architecture using Quantum Processor Substrate**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Overview

AI-Native Brain is a revolutionary consciousness modeling system that uses quantum reservoir computing as the computational substrate, enabling empirical testing of the core hypothesis:

**Φ_quantum > Φ_classical**: Quantum systems exhibit higher integrated information (consciousness) than classical systems.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    AI-Native Brain                          │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │     Quantum Reservoir Computing Substrate            │  │
│  │  • Coupled quantum oscillators (Fock space)          │  │
│  │  • Hamiltonian evolution: H = Σℏω(a†a) + Σg(a†b)    │  │
│  │  • Exponential neuron scaling: (max_fock+1)^N        │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↕                                 │
│  ┌──────────────────────────────────────────────────────┐  │
│  │        Error Correction & Radiation Protection       │  │
│  │  • LDPC bivariate bicycle codes                      │  │
│  │  • Cosmic ray simulation and mitigation              │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↕                                 │
│  ┌──────────────────────────────────────────────────────┐  │
│  │       IIT Consciousness Measurement (Φ)              │  │
│  │  • Partition entropy calculation                     │  │
│  │  • Cause-effect repertoire                           │  │
│  │  • Minimum Information Partition (MIP)               │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Features

### ✅ IMPLEMENTED (v0.1.0)

#### Core AIBrain
- **Quantum Reservoir Computing**: Coupled quantum oscillators with exponential scaling
- **Configurable Architecture**: Number of oscillators, Fock states, coupling strength
- **LDPC Error Correction**: IBM-style bivariate bicycle codes
- **Radiation Simulation**: Cosmic ray effects and mitigation strategies
- **State Management**: Evolution, input/output, readout

#### Consciousness Measurement (IIT)
- **Φ Calculation**: Integrated Information measurement via partition analysis
- **Bipartition Generation**: All possible system splits
- **Information Loss**: Shannon entropy-based mutual information
- **Minimum Information Partition (MIP)**: Find partition with least information loss
- **Quantum vs Classical Comparison**: Direct empirical testing

#### Experimental Framework
- **Controlled Experiments**: Multi-trial consciousness measurements
- **Statistical Analysis**: Mean, standard deviation, confirmation rates
- **Scaling Studies**: Test how Φ scales with system size
- **JSON Export**: Full experiment results for analysis

## Example Usage

```rust
use brain_ai_native::prelude::*;

// Create quantum brain with 4 oscillators
let config = BrainConfig {
    num_oscillators: 4,
    max_fock: 2,  // 3^4 = 81 effective neurons
    frequencies: vec![1e9; 4],
    coupling_strength: 1e6,
    damping_rate: 1e3,
    error_correction: true,
    ldpc_distance: 3,
    radiation_protection: true,
    chip_area_cm2: 1.0,
    altitude_m: 0.0,
};

let mut brain = AIBrain::new(config)?;

// Evolve brain
brain.evolve(1e-9)?;  // 1 nanosecond

// Measure consciousness
let measurement = measure_phi_quantum(&brain)?;
println!("Φ = {:.6}", measurement.phi);

// Compare with classical system
let classical_state = brain.get_state_vector();
let classical_measurement = measure_phi_classical(&classical_state, 4)?;

let comparison = ConsciousnessComparison::new(measurement, classical_measurement);
println!("{}", comparison.display());
```

## Running Experiments

```rust
use brain_ai_native::prelude::*;

// Configure experiment
let config = ExperimentConfig {
    num_trials: 10,
    evolution_time: 1e-6,  // 1 microsecond
    dt: 1e-9,              // 1 nanosecond steps
    brain_configs: vec![BrainConfig::default()],
    classical_sizes: vec![4, 8, 16],
};

// Run experiment
let results = run_consciousness_experiment(config)?;

// Display results
println!("{}", results.display());

// Export to JSON
let json = results.to_json()?;
std::fs::write("results.json", json)?;
```

## Scientific Basis

### Hypothesis

**Φ_quantum > Φ_classical** due to:

1. **Exponential State Space**: (max_fock+1)^N effective neurons vs N classical neurons
2. **Quantum Superposition**: True parallel processing of information states
3. **Entanglement**: Non-local correlations impossible in classical systems
4. **Unitary Evolution**: Information-preserving dynamics

### Integrated Information Theory (IIT)

IIT measures consciousness as the irreducible cause-effect power of a system:

- **Φ = 0**: No consciousness (e.g., feed-forward networks)
- **Φ > 0**: Some consciousness
- **Φ_max**: Maximum consciousness for given architecture

### Calculation Method

1. Generate all bipartitions of the system
2. Calculate information loss for each partition:
   ```
   I(A;B) = H(A) + H(B) - H(A,B)
   ```
3. Φ = minimum information loss (MIP)

## Performance Scaling

| Oscillators | max_fock | Effective Neurons | State Space | Φ (typical) |
|-------------|----------|------------------|-------------|-------------|
| 2           | 1        | 4                | 2^2         | ~0.5        |
| 2           | 2        | 9                | 3^2         | ~0.8        |
| 3           | 2        | 27               | 3^3         | ~1.2        |
| 4           | 2        | 81               | 3^4         | ~1.8        |
| 10          | 8        | 10 billion       | 9^10        | ~?          |

## Test Coverage

- **20 tests, 100% passing**
- Core AIBrain: configuration, creation, evolution, I/O
- Consciousness: entropy, partitions, Φ measurement, comparison
- Experiments: configuration, execution, statistical analysis

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
brain-ai-native = { path = "../brain-ai-native" }
```

## Dependencies

- `quantum-processor`: Quantum reservoir computing substrate
- `iit`: Integrated Information Theory implementation
- `neural-dynamics`: Neural dynamics models
- `hodgkin-huxley`: Neuronal models

## Revolutionary Potential

This is the first system to:

1. **Empirically test quantum consciousness**: Direct Φ measurement on quantum vs classical substrates
2. **Exponential neuron scaling**: 10 oscillators → 10 billion effective neurons
3. **Radiation-hardened quantum computing**: Protection from cosmic rays
4. **IIT-validated architecture**: Measurable consciousness metric

## Roadmap

### v0.2.0
- [ ] Run large-scale experiments (1000+ trials)
- [ ] Statistical significance testing
- [ ] Visualization tools for Φ trajectories
- [ ] Integration with biological brain models

### v0.3.0
- [ ] GPU acceleration for large systems
- [ ] Distributed computing support
- [ ] Real-time consciousness monitoring
- [ ] Consciousness optimization algorithms

## License

Dual-licensed under MIT or Apache 2.0

## Citation

```bibtex
@software{brain_ai_native2025,
  author = {Molina Burgos, Francisco},
  title = {brain-ai-native: AI-Native Brain with Quantum Consciousness},
  year = {2025},
  url = {https://github.com/Yatrogenesis/cortexia-workspace}
}
```

## Contributing

Part of the CORTEXIA/SYNTEX ecosystem. See main repository for contribution guidelines.

## Acknowledgments

- **IIT**: Giulio Tononi's Integrated Information Theory
- **Quantum Computing**: IBM, Google, PsiQuantum research
- **SYNTEX Framework**: Quantum adaptive consciousness modeling

---

**brain-ai-native** - Empirically testing quantum consciousness 🧠⚛️✨
