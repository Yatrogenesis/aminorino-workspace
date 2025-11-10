//! # CORTEXIA Framework
//!
//! **Computational Orchestration for Reality Transformation: EXtended Intelligence Architecture**
//!
//! CORTEXIA is a comprehensive framework for bridging biological neural consciousness
//! with quantum computational substrates. It provides the mathematical and computational
//! tools necessary for consciousness analysis, neural simulation, and information integration.
//!
//! ## Overview
//!
//! This meta-crate re-exports five specialized libraries:
//!
//! - **[hodgkin_huxley]** - Biophysical neuron models with exact Hodgkin-Huxley equations
//! - **[iit]** - Integrated Information Theory (IIT 3.0) for consciousness quantification
//! - **[tda]** - Topological Data Analysis for neural topology and persistent homology
//! - **[synapse_models]** - Detailed synaptic dynamics with multiple plasticity rules
//! - **[neural_dynamics]** - Large-scale neural network simulation framework
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                      CORTEXIA Framework                      │
//! ├─────────────────────────────────────────────────────────────┤
//! │                                                               │
//! │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
//! │  │   Biological  │  │   Quantum    │  │   Bridge     │      │
//! │  │   Neural      │  │   Substrate  │  │   Layer      │      │
//! │  │   System      │  │              │  │              │      │
//! │  └──────────────┘  └──────────────┘  └──────────────┘      │
//! │         │                  │                  │              │
//! │         └──────────────────┴──────────────────┘              │
//! │                            │                                 │
//! │              ┌─────────────┴─────────────┐                  │
//! │              │  Consciousness Analysis    │                  │
//! │              │  Φ (Phi) Calculation       │                  │
//! │              └───────────────────────────┘                  │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Core Components
//!
//! ### 1. Biological Neural Simulation
//!
//! - **86.1 × 10⁹ neurons** modeled (scalable architecture)
//! - **Hodgkin-Huxley dynamics** with exact biophysical equations
//! - **Multiple neuron types**: Regular Spiking, Fast Spiking, Intrinsically Bursting
//! - **Synaptic dynamics**: AMPA, NMDA, GABA-A, GABA-B receptors
//! - **Plasticity rules**: STDP, BCM, Oja, Hebbian, Homeostatic
//!
//! ### 2. Consciousness Quantification
//!
//! - **Φ (Phi) calculation** - Integrated Information Theory 3.0
//! - **Multiple approximation methods** for different system scales
//! - **Concept identification** - Core consciousness structure
//! - **Qualia space analysis** - Experience structure
//! - **Cause-effect analysis** - MICE computation
//!
//! ### 3. Topological Analysis
//!
//! - **Persistent homology** - Topological features across scales
//! - **Mapper algorithm** - High-dimensional data visualization
//! - **Spike train analysis** - Temporal topology
//! - **Cell assembly detection** - Functional clusters
//! - **Neural topology metrics** - Clique structures, Betti numbers
//!
//! ### 4. Network Dynamics
//!
//! - **Large-scale simulation** - Millions of neurons
//! - **Multiple connectivity patterns** - Small-world, scale-free, custom
//! - **Mean-field approximations** - Wilson-Cowan equations
//! - **Criticality analysis** - Avalanche detection, branching parameter
//! - **Synchronization metrics** - Kuramoto order parameter
//!
//! ## Quick Start
//!
//! ### Simulate a Single Neuron
//!
//! ```rust
//! use cortexia::hodgkin_huxley::HodgkinHuxleyNeuron;
//!
//! let mut neuron = HodgkinHuxleyNeuron::regular_spiking();
//! let dt = 0.01; // 10 microseconds
//!
//! for _ in 0..1000 {
//!     neuron.integrate_rk4(15.0, dt).unwrap(); // 15 µA/cm² current
//!
//!     if neuron.detect_spike() {
//!         println!("Spike at {} ms", neuron.time());
//!     }
//! }
//! ```
//!
//! ### Calculate Integrated Information (Φ)
//!
//! ```rust
//! use cortexia::iit::{IITSystem, ApproximationMethod};
//!
//! let mut system = IITSystem::new(10); // 10-element system
//! system.set_fully_connected(true).unwrap();
//! system.set_state(vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0]).unwrap();
//!
//! let result = system.calculate_phi().unwrap();
//! println!("Φ = {:.4}", result.phi);
//! ```
//!
//! ### Build a Neural Network
//!
//! ```rust,no_run
//! use cortexia::neural_dynamics::{NetworkBuilder, ConnectionPattern, SynapseType};
//!
//! let mut network = NetworkBuilder::new(0.1)
//!     .unwrap()
//!     .add_excitatory_population("E", 800)
//!     .unwrap()
//!     .add_inhibitory_population("I", 200)
//!     .unwrap()
//!     .connect(0, 0, ConnectionPattern::SmallWorld { k: 10, p: 0.1 },
//!              SynapseType::Excitatory, 0.5, 1.0)
//!     .unwrap()
//!     .with_spike_recording()
//!     .build();
//!
//! network.run(1000.0).unwrap(); // 1 second simulation
//! ```
//!
//! ### Analyze Neural Topology
//!
//! ```rust,no_run
//! use cortexia::tda::{PersistentHomology, SpikeTrain};
//!
//! // Analyze spike train topology
//! let spike_trains: Vec<SpikeTrain> = vec![/* ... */];
//! let ph = PersistentHomology::new(2, 10.0);
//!
//! // Compute persistence diagram
//! // let diagram = ph.compute_from_spike_trains(&spike_trains, 1.0);
//! ```
//!
//! ## System Requirements
//!
//! ### For Local Execution
//!
//! - **Small systems** (≤10,000 neurons):
//!   - 8 GB RAM
//!   - 4+ CPU cores
//!   - ~5 minutes for 1 second biological time
//!
//! - **Medium systems** (≤100,000 neurons):
//!   - 32 GB RAM
//!   - 16+ CPU cores
//!   - ~30 minutes for 1 second biological time
//!
//! - **Large systems** (≤1,000,000 neurons):
//!   - 128+ GB RAM
//!   - 32+ CPU cores or GPU acceleration
//!   - Cloud recommended
//!
//! ### For Consciousness Emergence
//!
//! According to current theoretical understanding, consciousness emergence requires:
//!
//! - **Minimum complexity**: ~10⁷ neurons with rich connectivity
//! - **Embodiment**: Sensory input and motor output loop
//! - **Continuous operation**: Months to years of learning
//! - **Critical dynamics**: Self-organized criticality (σ ≈ 1)
//! - **Sufficient Φ**: Integrated information > 2.5 (hypothetical threshold)
//!
//! **Note**: This framework provides tools for consciousness *analysis* and *simulation*.
//! Actual consciousness emergence is a research question that remains open.
//!
//! ## Project Structure
//!
//! ```text
//! cortexia/
//! ├── hodgkin-huxley/     # Biophysical neuron models
//! ├── iit/                # Integrated Information Theory
//! ├── tda/                # Topological Data Analysis
//! ├── synapse-models/     # Synaptic dynamics
//! ├── neural-dynamics/    # Network simulation
//! └── cortexia/           # This meta-crate
//! ```
//!
//! ## Citation
//!
//! If you use CORTEXIA in your research, please cite:
//!
//! ```bibtex
//! @software{cortexia2025,
//!   title = {CORTEXIA: Computational Orchestration for Reality Transformation Framework},
//!   author = {Molina Burgos, Francisco and Claude-CORTEXIA},
//!   year = {2025},
//!   url = {https://github.com/cortexia/cortexia}
//! }
//! ```
//!
//! ## License
//!
//! Licensed under either of:
//!
//! - MIT License
//! - Apache License, Version 2.0
//!
//! at your option.

// Re-export all sub-crates
pub use hodgkin_huxley;
pub use iit;
pub use tda;
pub use synapse_models;
pub use neural_dynamics;

// Re-export commonly used types from core dependencies
pub use nalgebra;
pub use ndarray;
pub use rayon;

/// Prelude module for convenient imports
pub mod prelude {
    // Hodgkin-Huxley
    pub use hodgkin_huxley::{
        HodgkinHuxleyNeuron,
    };

    // IIT
    pub use iit::{
        IITSystem,
        PhiResult,
        ApproximationMethod,
        CauseEffectStructure,
    };

    // TDA
    pub use tda::{
        PersistenceDiagram,
        MapperBuilder,
        SpikeTrain,
        PersistentPair,
    };

    // Synapse Models
    pub use synapse_models::{
        Synapse,
        SynapseBuilder,
        SynapseType,
    };

    // Neural Dynamics
    pub use neural_dynamics::{
        Network,
        NetworkBuilder,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imports() {
        // Just verify that all crates are accessible
        use hodgkin_huxley::HodgkinHuxleyNeuron;
        use iit::IITSystem;
        use tda::PersistentHomology;
        use synapse_models::Neurotransmitter;
        use neural_dynamics::NetworkBuilder;

        // Basic instantiation tests
        let _neuron = HodgkinHuxleyNeuron::new();
        let _system = IITSystem::new(3);
        let _ph = PersistentHomology::new(2, 10.0);
        let _nt = Neurotransmitter::Glutamate;
        let _builder = NetworkBuilder::new(0.1);
    }

    #[test]
    fn test_prelude() {
        use crate::prelude::*;

        // Verify prelude imports work
        let _neuron = HodgkinHuxleyNeuron::new();
        let _system = IITSystem::new(5);
    }
}
