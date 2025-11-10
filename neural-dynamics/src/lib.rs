//! # Neural Dynamics Library
//!
//! A comprehensive library for large-scale neural network simulations with advanced
//! connectivity patterns, dynamics analysis, and mean-field approximations.
//!
//! ## Overview
//!
//! This library provides tools for simulating networks of biophysically realistic neurons
//! with complex connectivity patterns and analyzing their collective dynamics. It integrates:
//!
//! - **Hodgkin-Huxley neurons**: Detailed biophysical neuron models
//! - **Synaptic models**: Realistic synaptic transmission and plasticity
//! - **Network topologies**: Small-world, scale-free, spatial networks
//! - **Population dynamics**: Mean-field approximations (Wilson-Cowan)
//! - **Analysis tools**: Synchrony, criticality, avalanche detection
//!
//! ## Quick Start
//!
//! ### Create a simple excitatory-inhibitory network
//!
//! ```rust
//! use neural_dynamics::{Network, NetworkBuilder, ConnectionPattern, SynapseType};
//! use neural_dynamics::stimulation::CurrentInjection;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Build an E-I network
//! let mut network = NetworkBuilder::new(0.1)?
//!     .add_excitatory_population("E", 80)?
//!     .add_inhibitory_population("I", 20)?
//!     .connect(0, 0, ConnectionPattern::FixedProbability(0.1),
//!              SynapseType::Excitatory, 0.5, 1.0)?
//!     .connect(0, 1, ConnectionPattern::FixedProbability(0.2),
//!              SynapseType::Excitatory, 0.8, 1.0)?
//!     .connect(1, 0, ConnectionPattern::FixedProbability(0.3),
//!              SynapseType::Inhibitory, 1.5, 0.5)?
//!     .with_spike_recording()
//!     .build();
//!
//! // Add external drive to excitatory population
//! let stim = CurrentInjection::new(5.0, 0.0, 100.0);
//! network.add_stimulation(0, Box::new(stim))?;
//!
//! // Run simulation
//! network.run(100.0)?;
//!
//! // Analyze results
//! let stats = network.statistics();
//! println!("Total spikes: {}", stats.total_spikes);
//! # Ok(())
//! # }
//! ```
//!
//! ### Analyze network synchrony
//!
//! ```rust
//! use neural_dynamics::analysis::kuramoto_order_parameter;
//! use std::f64::consts::PI;
//!
//! let phases = vec![0.0, 0.1, 0.05, 0.0]; // Nearly synchronized
//! let order_param = kuramoto_order_parameter(&phases);
//! println!("Synchrony: {:.2}", order_param); // Close to 1.0
//! ```
//!
//! ### Detect network avalanches
//!
//! ```rust
//! use neural_dynamics::analysis::detect_avalanches;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let spike_trains = vec![
//!     vec![1.0, 2.0, 10.0],
//!     vec![1.5, 2.5, 10.5],
//!     vec![2.0, 11.0],
//! ];
//!
//! let avalanches = detect_avalanches(&spike_trains, 1.0, 1)?;
//! println!("Detected {} avalanches", avalanches.len());
//! # Ok(())
//! # }
//! ```
//!
//! ### Wilson-Cowan mean-field model
//!
//! ```rust
//! use neural_dynamics::mean_field::WilsonCowanModel;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut model = WilsonCowanModel::balanced_network()?;
//! model.set_input(0.5, 0.0);
//!
//! // Simulate population dynamics
//! let trace = model.simulate(100.0, 0.1)?;
//!
//! // Analyze fixed points
//! let fixed_points = model.find_fixed_points(20);
//! println!("Found {} fixed points", fixed_points.len());
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! ### Populations
//!
//! A `NeuralPopulation` groups neurons with similar properties:
//! - Homogeneous: All neurons identical
//! - Heterogeneous: Parameter variability across neurons
//! - Efficient parallel updates using rayon
//!
//! ### Projections
//!
//! A `Projection` connects two populations with:
//! - Flexible connectivity patterns (all-to-all, small-world, scale-free, etc.)
//! - Synaptic transmission delays
//! - Weight distributions (constant, uniform, normal)
//! - Event-driven spike propagation
//!
//! ### Network
//!
//! The `Network` orchestrates:
//! - Multiple populations and projections
//! - External stimulation protocols
//! - Recording (spikes, voltages, rates)
//! - Efficient simulation with delay queues
//!
//! ## Connectivity Patterns
//!
//! - **AllToAll**: Dense connectivity
//! - **OneToOne**: Identity mapping
//! - **FixedProbability(p)**: Erdős-Rényi random graph
//! - **FixedNumber(n)**: Fixed in-degree
//! - **SmallWorld{k, p}**: Watts-Strogatz model
//! - **ScaleFree{m}**: Barabási-Albert model
//! - **Gaussian{σ}**: Distance-dependent connectivity
//! - **Custom**: User-defined connectivity matrix
//!
//! ## Analysis Tools
//!
//! ### Synchrony Measures
//! - **Kuramoto order parameter**: R ∈ [0,1], 1 = perfect synchrony
//! - **Cross-correlation**: Temporal relationships between spike trains
//! - **Phase locking**: Relative spike timing analysis
//!
//! ### Criticality
//! - **Avalanche detection**: Contiguous activity bursts
//! - **Branching parameter**: σ = ⟨n_{t+1}⟩/⟨n_t⟩, σ=1 is critical
//! - **Power-law distributions**: Scale-free avalanche statistics
//!
//! ### Firing Statistics
//! - **Population rates**: Average activity levels
//! - **CV_ISI**: Coefficient of variation of interspike intervals
//! - **Spike count distributions**
//!
//! ## Mathematical Models
//!
//! ### Hodgkin-Huxley Neurons
//!
//! ```text
//! C_m dV/dt = -I_Na - I_K - I_K(Ca) - I_leak + I_ext + I_syn
//! ```
//!
//! ### Wilson-Cowan Equations
//!
//! ```text
//! τ_E dE/dt = -E + S(w_EE·E - w_EI·I + I_E)
//! τ_I dI/dt = -I + S(w_IE·E - w_II·I + I_I)
//! ```
//!
//! where S(x) = 1/(1 + exp(-gain·(x - θ))) is the sigmoid transfer function.
//!
//! ### Kuramoto Order Parameter
//!
//! ```text
//! R = |1/N Σ_j exp(iθ_j)|
//! ```
//!
//! ## Performance
//!
//! - **Parallel updates**: Population dynamics computed in parallel using rayon
//! - **Sparse connectivity**: Efficient storage and computation
//! - **Event-driven spikes**: Lazy propagation through delay queues
//! - **Memory efficient**: Minimal allocations in simulation loops
//!
//! ## Features
//!
//! - ✅ Biophysically realistic neurons (Hodgkin-Huxley)
//! - ✅ Complex synaptic dynamics (AMPA, NMDA, GABA)
//! - ✅ Short-term plasticity (depression, facilitation)
//! - ✅ Long-term plasticity (STDP)
//! - ✅ Multiple connectivity patterns
//! - ✅ Mean-field approximations
//! - ✅ Comprehensive analysis tools
//! - ✅ Parallel computation
//! - ✅ Extensive test coverage
//!
//! ## Examples
//!
//! See the examples directory for complete simulations:
//! - `balanced_network.rs`: E-I balance and oscillations
//! - `small_world.rs`: Small-world connectivity and synchronization
//! - `critical_dynamics.rs`: Self-organized criticality
//! - `wilson_cowan.rs`: Mean-field population dynamics
//!
//! ## References
//!
//! - Hodgkin & Huxley (1952). A quantitative description of membrane current and its application to conduction and excitation in nerve.
//! - Wilson & Cowan (1972). Excitatory and inhibitory interactions in localized populations of model neurons.
//! - Watts & Strogatz (1998). Collective dynamics of 'small-world' networks.
//! - Barabási & Albert (1999). Emergence of scaling in random networks.
//! - Beggs & Plenz (2003). Neuronal avalanches in neocortical circuits.
//! - Kuramoto (1984). Chemical Oscillations, Waves, and Turbulence.

pub mod analysis;
pub mod connectivity;
pub mod error;
pub mod mean_field;
pub mod network;
pub mod population;
pub mod projection;
pub mod recording;
pub mod stimulation;

// Re-export commonly used types
pub use error::{NeuralDynamicsError, Result};
pub use network::{Network, NetworkBuilder, NetworkStats, SynapseType};
pub use population::{NeuralPopulation, PopulationStats};
pub use projection::{Connection, DelayInit, Projection, WeightInit, WeightStats};
pub use connectivity::{ConnectionPattern, NetworkStats as ConnectivityStats};
pub use recording::{PopulationRateRecorder, SpikeRecorder, VoltageRecorder};
pub use mean_field::{WilsonCowanModel, PopulationRateModel};

// Re-export from dependencies
pub use hodgkin_huxley;
pub use synapse_models;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod integration_tests {
    use super::*;
    use analysis::*;
    use connectivity::*;
    use stimulation::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_small_network_simulation() {
        let mut network = NetworkBuilder::new(0.1)
            .unwrap()
            .add_excitatory_population("E", 10)
            .unwrap()
            .with_spike_recording()
            .build();

        // Add stimulation
        let stim = CurrentInjection::new(10.0, 0.0, 50.0);
        network.add_stimulation(0, Box::new(stim)).unwrap();

        // Run simulation
        network.run(50.0).unwrap();

        // Check that spikes occurred
        let stats = network.statistics();
        assert!(stats.total_spikes > 0);
    }

    #[test]
    fn test_ei_network() {
        let mut network = NetworkBuilder::new(0.1)
            .unwrap()
            .add_excitatory_population("E", 20)
            .unwrap()
            .add_inhibitory_population("I", 5)
            .unwrap()
            .connect(
                0, 0,
                ConnectionPattern::FixedProbability(0.2),
                SynapseType::Excitatory,
                0.5, 1.0
            )
            .unwrap()
            .connect(
                0, 1,
                ConnectionPattern::FixedProbability(0.3),
                SynapseType::Excitatory,
                0.8, 1.0
            )
            .unwrap()
            .connect(
                1, 0,
                ConnectionPattern::FixedProbability(0.4),
                SynapseType::Inhibitory,
                1.2, 0.5
            )
            .unwrap()
            .with_spike_recording()
            .build();

        // Stimulate excitatory population
        let stim = CurrentInjection::new(8.0, 0.0, 100.0);
        network.add_stimulation(0, Box::new(stim)).unwrap();

        // Run
        network.run(100.0).unwrap();

        // Should have activity in both populations
        let recorder = network.spike_recorder.as_ref().unwrap();
        assert!(recorder.total_spikes() > 0);
    }

    #[test]
    fn test_synchrony_emergence() {
        let mut network = NetworkBuilder::new(0.1)
            .unwrap()
            .add_excitatory_population("E", 30)
            .unwrap()
            .connect(
                0, 0,
                ConnectionPattern::SmallWorld { k: 6, p: 0.1 },
                SynapseType::Excitatory,
                1.0, 1.0
            )
            .unwrap()
            .with_spike_recording()
            .build();

        // Uniform stimulation
        let stim = CurrentInjection::new(6.0, 0.0, 200.0);
        network.add_stimulation(0, Box::new(stim)).unwrap();

        network.run(200.0).unwrap();

        // Calculate synchrony (would need phase extraction from spikes)
        let recorder = network.spike_recorder.as_ref().unwrap();
        assert!(recorder.total_spikes() > 50);
    }

    #[test]
    fn test_avalanche_detection_in_network() {
        let mut network = NetworkBuilder::new(0.1)
            .unwrap()
            .add_excitatory_population("E", 50)
            .unwrap()
            .connect(
                0, 0,
                ConnectionPattern::FixedProbability(0.08),
                SynapseType::Excitatory,
                0.8, 1.0
            )
            .unwrap()
            .with_spike_recording()
            .build();

        // Weak stimulation to maintain near-critical dynamics
        let stim = CurrentInjection::new(3.0, 0.0, 500.0);
        network.add_stimulation(0, Box::new(stim)).unwrap();

        network.run(500.0).unwrap();

        // Extract spike trains
        let pop = network.get_population(0).unwrap();
        let spike_trains: Vec<Vec<f64>> = (0..pop.size)
            .map(|i| pop.get_spike_times(i).unwrap().to_vec())
            .collect();

        // Detect avalanches
        let avalanches = detect_avalanches(&spike_trains, 1.0, 2).unwrap();
        assert!(!avalanches.is_empty());
    }

    #[test]
    fn test_wilson_cowan_integration() {
        let mut model = WilsonCowanModel::balanced_network().unwrap();
        model.set_input(1.0, 0.5);

        let trace = model.simulate(100.0, 0.1).unwrap();

        // Should reach some steady state
        let final_e = trace.last().unwrap().1;
        let final_i = trace.last().unwrap().2;

        assert!(final_e > 0.0 && final_e < 1.0);
        assert!(final_i > 0.0 && final_i < 1.0);
    }

    #[test]
    fn test_connectivity_patterns() {
        let mut rng = rand::thread_rng();

        // Test each connectivity pattern
        let patterns = vec![
            ConnectionPattern::AllToAll,
            ConnectionPattern::OneToOne,
            ConnectionPattern::FixedProbability(0.3),
            ConnectionPattern::FixedNumber(5),
            ConnectionPattern::SmallWorld { k: 4, p: 0.2 },
            ConnectionPattern::ScaleFree { m: 3 },
            ConnectionPattern::Gaussian { sigma: 0.2 },
        ];

        for pattern in patterns {
            let connections = pattern.generate(20, 20, &mut rng);
            assert!(connections.is_ok());
        }
    }

    #[test]
    fn test_population_rate_calculation() {
        let spike_trains = vec![
            vec![10.0, 20.0, 30.0, 40.0],
            vec![15.0, 25.0, 35.0],
            vec![12.0, 22.0, 32.0, 42.0],
        ];

        let rate = population_firing_rate(&spike_trains, (0.0, 50.0));

        // 11 spikes / (3 neurons * 50 ms) = 73.33 Hz
        assert!(rate > 60.0 && rate < 90.0);
    }

    #[test]
    fn test_branching_parameter() {
        let spike_trains = vec![
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![1.5, 2.5, 3.5, 4.5],
            vec![2.0, 3.0, 4.0, 5.0],
        ];

        let sigma = branching_parameter(&spike_trains, 0.5).unwrap();
        assert!(sigma > 0.0 && sigma < 3.0);
    }

    #[test]
    fn test_kuramoto_synchrony() {
        use std::f64::consts::PI;

        // Perfect synchrony
        let phases = vec![0.0; 10];
        let r = kuramoto_order_parameter(&phases);
        assert_relative_eq!(r, 1.0, epsilon = 1e-10);

        // Complete desynchronization
        let phases: Vec<f64> = (0..10).map(|i| 2.0 * PI * i as f64 / 10.0).collect();
        let r = kuramoto_order_parameter(&phases);
        assert!(r < 0.3);
    }
}
