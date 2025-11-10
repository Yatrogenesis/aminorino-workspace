# Synapse Models Library

A comprehensive Rust library for modeling synaptic dynamics in computational neuroscience. This library provides detailed biophysical models of synaptic transmission, plasticity, and network dynamics.

## Features

### Neurotransmitter Dynamics
- **Multiple neurotransmitter types**: Glutamate, GABA, Dopamine, Serotonin, Acetylcholine, Norepinephrine
- **Realistic kinetics**: Exponential decay with biologically accurate time constants
- **Diffusion and clearance**: Based on experimental measurements

### Receptor Models
- **Ionotropic receptors**: AMPA, NMDA, GABA-A with detailed kinetic schemes
- **Metabotropic receptors**: GABA-B, mGluR with G-protein coupling
- **NMDA voltage dependence**: Mg²⁺ block with Jahr & Stevens (1990) model
- **Calcium permeability**: NMDA-mediated Ca²⁺ influx

### Vesicle Pool Dynamics
- **Tsodyks-Markram model**: Short-term depression and facilitation
- **Multiple pools**: Ready releasable, reserve, and recycling pools
- **Calcium-dependent release**: Hill equation with cooperativity
- **Quantal release**: Binomial statistics with multi-vesicular release

### Calcium Dynamics
- **Pre/postsynaptic compartments**: Independent calcium dynamics
- **Buffering**: Fast and slow buffers with realistic binding kinetics
- **Calcium stores**: ER uptake/release with SERCA pumps
- **CICR**: Calcium-induced calcium release via RyR
- **IP3 signaling**: IP3 receptor-mediated release

### Plasticity Rules
- **STDP**: Spike-timing dependent plasticity with asymmetric window
- **BCM rule**: Bienenstock-Cooper-Munro with sliding threshold
- **Oja's rule**: Normalized Hebbian learning
- **Hebbian/Anti-Hebbian**: Classic learning rules
- **Homeostatic plasticity**: Activity-dependent scaling
- **Meta-plasticity**: Plasticity of plasticity

### Network Models
- **Chemical synapses**: Full models with delays and dynamics
- **Gap junctions**: Electrical synapses with bidirectional coupling
- **Ephaptic coupling**: Electric field effects
- **Neuromodulation**: Dopamine, serotonin, ACh, norepinephrine
- **Spike propagation**: Event-driven with realistic delays

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
synapse-models = "0.1.0"
```

## Quick Start

### Basic Synaptic Transmission

```rust
use synapse_models::Synapse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an excitatory synapse
    let mut synapse = Synapse::excitatory(1.0, 1.0)?;

    // Presynaptic spike
    synapse.presynaptic_spike(0.0)?;

    // Simulate for 10 ms
    for t in 0..100 {
        let time = t as f64 * 0.1;
        synapse.update(time, -65.0, 0.1)?;

        println!("t={:.1} ms, g={:.4} nS", time, synapse.conductance());
    }

    Ok(())
}
```

### STDP Learning

```rust
use synapse_models::Synapse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut synapse = Synapse::excitatory(0.5, 1.0)?;
    let initial_weight = synapse.weight;

    // Pre before post (10 ms) → potentiation
    synapse.presynaptic_spike(0.0)?;
    synapse.postsynaptic_spike(10.0)?;

    println!("Weight change: {:.6}", synapse.weight - initial_weight);
    // Output: Weight change: 0.006065 (potentiation)

    Ok(())
}
```

### Network Simulation

```rust
use synapse_models::{SynapticNetwork, Synapse};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a network with 10 neurons
    let mut network = SynapticNetwork::new(10);

    // Add connections
    for i in 0..9 {
        let syn = Synapse::excitatory(1.0, 2.0)?;
        network.add_connection(i, i + 1, syn)?;
    }

    // Spike from first neuron
    network.spike(0)?;

    // Simulate
    let voltages = vec![-65.0; 10];
    for _ in 0..100 {
        network.update(&voltages, 0.1)?;
    }

    // Get synaptic current to neuron 5
    let current = network.get_synaptic_current(5)?;
    println!("Current to neuron 5: {:.2} pA", current);

    Ok(())
}
```

## Biophysical Parameters

All parameters are based on experimental data:

### AMPA Receptors
- Rise time: 0.2 ms
- Decay time: 2 ms
- Reversal potential: 0 mV

### NMDA Receptors
- Rise time: 2 ms
- Decay time: 100 ms
- Reversal potential: 0 mV
- Mg²⁺ block: Jahr & Stevens (1990)

### GABA-A Receptors
- Rise time: 0.5 ms
- Decay time: 5 ms
- Reversal potential: -70 mV

### GABA-B Receptors
- Rise time: 50 ms
- Decay time: 200 ms
- Reversal potential: -90 mV

### Vesicle Pool (Tsodyks-Markram)
- **Depressing**: U = 0.6, τ_rec = 130 ms
- **Facilitating**: U = 0.15, τ_rec = 670 ms, τ_facil = 17 ms

### STDP
- Potentiation window: τ+ = 20 ms, A+ = 0.01
- Depression window: τ- = 20 ms, A- = 0.01

## Mathematical Models

### Receptor Kinetics
First-order binding:
```
dR/dt = α[NT](1-R) - βR
```
where R is open probability, [NT] is neurotransmitter concentration.

### NMDA Mg²⁺ Block
Jahr & Stevens (1990):
```
B(V) = 1 / (1 + [Mg²⁺]/3.57 * exp(-0.062*V))
```

### Tsodyks-Markram Model
```
dx/dt = (1-x)/τ_rec - U*x*δ(t-t_spike)
du/dt = (U₀-u)/τ_facil + U₀(1-u)δ(t-t_spike)
```

### STDP Window
```
Δw = A+ * exp(-Δt/τ+)     for Δt > 0 (potentiation)
Δw = -A- * exp(Δt/τ-)     for Δt < 0 (depression)
```

## Examples

Run the examples:

```bash
# Basic synapse simulation
cargo run --example basic_synapse

# Network simulation
cargo run --example network_simulation
```

## Testing

Run the comprehensive test suite:

```bash
cargo test --package synapse-models
```

All 68 tests cover:
- Neurotransmitter dynamics
- Receptor kinetics and voltage dependence
- Vesicle pool dynamics
- Calcium dynamics and CICR
- All plasticity rules
- Synapse integration
- Network connectivity and propagation

## Performance

- **Numerical integration**: Exponential Euler for stability
- **Sparse networks**: Efficient adjacency list representation
- **Thread-safe**: Safe for parallel neuron updates
- **Minimal allocations**: Optimized for real-time simulation

## Architecture

```
synapse-models/
├── src/
│   ├── lib.rs              # Main library with documentation
│   ├── error.rs            # Error types
│   ├── neurotransmitter.rs # Neurotransmitter dynamics
│   ├── receptor.rs         # Receptor models
│   ├── vesicle.rs          # Vesicle pool dynamics
│   ├── calcium.rs          # Calcium dynamics
│   ├── plasticity.rs       # Learning rules
│   ├── synapse.rs          # Complete synapse model
│   └── network.rs          # Network-level dynamics
├── examples/               # Example programs
└── tests/                  # Integration tests
```

## References

### Key Papers
1. **Tsodyks & Markram (1997)** - Short-term synaptic plasticity model
   - *Proc Natl Acad Sci USA* 94(2):719-723

2. **Bi & Poo (1998)** - STDP experimental characterization
   - *J Neurosci* 18(24):10464-10472

3. **Jahr & Stevens (1990)** - NMDA Mg²⁺ block
   - *J Neurosci* 10(6):1830-1837

4. **Bienenstock et al. (1982)** - BCM theory
   - *J Neurosci* 2(1):32-48

### Textbooks
- Dayan & Abbott (2001) - *Theoretical Neuroscience*
- Gerstner et al. (2014) - *Neuronal Dynamics*
- Koch (1999) - *Biophysics of Computation*

## Contributing

This library is part of the CORTEXIA computational neuroscience toolkit. Contributions are welcome!

## License

MIT OR Apache-2.0

## Authors

- Francisco Molina Burgos
- Claude-CORTEXIA

## Citation

If you use this library in your research, please cite:

```bibtex
@software{synapse_models,
  title = {Synapse Models: A Comprehensive Rust Library for Synaptic Dynamics},
  author = {Molina Burgos, Francisco and Claude-CORTEXIA},
  year = {2024},
  url = {https://github.com/cortexia/synapse-models}
}
```

## Acknowledgments

This library implements models from numerous experimental and theoretical studies in neuroscience. We thank the researchers whose work made this possible.
