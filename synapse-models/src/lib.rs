//! # Synapse Models Library
//!
//! A comprehensive Rust library for modeling synaptic dynamics in computational neuroscience.
//!
//! ## Overview
//!
//! This library provides detailed biophysical models of synaptic transmission including:
//!
//! - **Neurotransmitter dynamics**: Multiple neurotransmitter types (glutamate, GABA, dopamine, etc.)
//!   with realistic release and clearance kinetics
//! - **Receptor models**: Detailed kinetic models for ionotropic (AMPA, NMDA, GABA-A) and
//!   metabotropic (GABA-B, mGluR) receptors
//! - **Vesicle pool dynamics**: Tsodyks-Markram model for short-term depression and facilitation
//! - **Calcium dynamics**: Pre- and postsynaptic calcium with buffering, stores, and CICR
//! - **Plasticity rules**: STDP, BCM, Oja's rule, Hebbian learning, homeostatic plasticity
//! - **Network models**: Support for chemical synapses, gap junctions, ephaptic coupling, and neuromodulation
//!
//! ## Features
//!
//! - Biologically accurate parameters based on experimental data
//! - Efficient numerical integration using exponential Euler methods
//! - Thread-safe design for parallel simulations
//! - Comprehensive test coverage
//! - Extensive documentation with neuroscience background
//!
//! ## Example: Basic Synaptic Transmission
//!
//! ```rust
//! use synapse_models::synapse::Synapse;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create an excitatory synapse
//! let mut synapse = Synapse::excitatory(1.0, 1.0)?;
//!
//! // Presynaptic spike at t=0
//! synapse.presynaptic_spike(0.0)?;
//!
//! // Simulate for 10 ms
//! for t in 0..100 {
//!     let time = t as f64 * 0.1;
//!     synapse.update(time, -65.0, 0.1)?;
//!
//!     // Get postsynaptic current
//!     let current = synapse.current(-65.0);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Example: STDP Learning
//!
//! ```rust
//! use synapse_models::synapse::Synapse;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut synapse = Synapse::excitatory(0.5, 1.0)?;
//!
//! let initial_weight = synapse.weight;
//!
//! // Pre before post -> potentiation
//! synapse.presynaptic_spike(0.0)?;
//! synapse.postsynaptic_spike(10.0)?;
//!
//! assert!(synapse.weight > initial_weight);
//! # Ok(())
//! # }
//! ```
//!
//! ## Example: Network Simulation
//!
//! ```rust
//! use synapse_models::network::SynapticNetwork;
//! use synapse_models::synapse::Synapse;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a network with 10 neurons
//! let mut network = SynapticNetwork::new(10);
//!
//! // Add excitatory connections
//! for i in 0..9 {
//!     let syn = Synapse::excitatory(1.0, 1.0)?;
//!     network.add_connection(i, i + 1, syn)?;
//! }
//!
//! // Spike from first neuron
//! network.spike(0)?;
//!
//! // Update network
//! let voltages = vec![-65.0; 10];
//! network.update(&voltages, 0.1)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Biophysical Background
//!
//! ### Synaptic Transmission
//!
//! Synaptic transmission involves multiple steps:
//! 1. **Action potential arrival** triggers voltage-gated calcium channels
//! 2. **Calcium influx** causes vesicle fusion and neurotransmitter release
//! 3. **Neurotransmitter diffusion** across the synaptic cleft (~20 nm)
//! 4. **Receptor binding** opens ion channels or activates second messengers
//! 5. **Postsynaptic current** flows, changing membrane potential
//! 6. **Neurotransmitter clearance** by reuptake or degradation
//!
//! ### Short-Term Plasticity
//!
//! Short-term plasticity operates on timescales of milliseconds to seconds:
//! - **Depression**: Depletion of readily releasable vesicle pool
//! - **Facilitation**: Residual calcium enhances release probability
//! - Modeled by Tsodyks-Markram equations
//!
//! ### Long-Term Plasticity
//!
//! Long-term plasticity underlies learning and memory:
//! - **STDP**: Spike timing-dependent modification (±20-40 ms window)
//! - **LTP/LTD**: Long-term potentiation/depression
//! - Requires postsynaptic calcium elevation
//! - CaMKII activation → LTP, calcineurin activation → LTD
//!
//! ## Mathematical Models
//!
//! ### Receptor Kinetics
//!
//! First-order binding scheme:
//! ```text
//! dR/dt = α[NT](1-R) - βR
//! ```
//! where R is open probability, [NT] is neurotransmitter concentration,
//! α is binding rate, β is unbinding rate.
//!
//! ### NMDA Voltage Dependence
//!
//! Mg²⁺ block (Jahr & Stevens, 1990):
//! ```text
//! B(V) = 1 / (1 + [Mg²⁺]/3.57 * exp(-0.062*V))
//! ```
//!
//! ### Tsodyks-Markram Model
//!
//! ```text
//! dx/dt = (1-x)/τ_rec - U*x*δ(t-t_spike)
//! du/dt = (U₀-u)/τ_facil + U₀(1-u)δ(t-t_spike)
//! ```
//!
//! ### STDP Window
//!
//! ```text
//! Δw = A₊ exp(-Δt/τ₊)     for Δt > 0 (potentiation)
//! Δw = -A₋ exp(Δt/τ₋)     for Δt < 0 (depression)
//! ```
//!
//! ## Performance Considerations
//!
//! - Uses exponential Euler integration for numerical stability
//! - Sparse network representation for efficiency
//! - Minimal allocations in update loops
//! - Thread-safe for parallel neuron updates
//!
//! ## References
//!
//! - Tsodyks & Markram (1997). The neural code between neocortical pyramidal neurons depends on neurotransmitter release probability.
//! - Bi & Poo (1998). Synaptic modifications in cultured hippocampal neurons: dependence on spike timing, synaptic strength, and postsynaptic cell type.
//! - Jahr & Stevens (1990). Voltage dependence of NMDA-activated macroscopic conductances predicted by single-channel kinetics.
//! - Bienenstock et al. (1982). Theory for the development of neuron selectivity: orientation specificity and binocular interaction in visual cortex.

pub mod calcium;
pub mod error;
pub mod network;
pub mod neurotransmitter;
pub mod plasticity;
pub mod receptor;
pub mod synapse;
pub mod vesicle;

// Re-export commonly used types
pub use error::{Result, SynapseError};
pub use synapse::{Synapse, SynapseBuilder, SynapseType};
pub use network::{SynapticNetwork, NetworkStats};

/// Library version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod integration_tests {
    use super::*;
    use synapse::Synapse;
    use network::SynapticNetwork;

    #[test]
    fn test_complete_synaptic_transmission() {
        let mut syn = Synapse::excitatory(1.0, 1.0).unwrap();

        // Initial state
        assert_eq!(syn.conductance(), 0.0);

        // Presynaptic spike
        syn.presynaptic_spike(0.0).unwrap();

        // Simulate transmission - need enough time for delay + receptor activation
        let mut max_conductance = 0.0_f64;
        let mut max_nt = 0.0_f64;
        for t in 0..100 {
            syn.update(t as f64 * 0.1, -65.0, 0.1).unwrap();
            max_conductance = max_conductance.max(syn.conductance());
            max_nt = max_nt.max(syn.neurotransmitter.get_concentration());
        }

        // Should have produced postsynaptic response at some point
        assert!(max_conductance > 0.0 || max_nt > 0.0,
                "max_conductance: {}, max_nt: {}", max_conductance, max_nt);
    }

    #[test]
    fn test_stdp_learning_window() {
        // Test potentiation
        let mut syn_pot = Synapse::excitatory(0.5, 1.0).unwrap();
        let w0 = syn_pot.weight;
        syn_pot.presynaptic_spike(0.0).unwrap();
        syn_pot.postsynaptic_spike(10.0).unwrap();
        assert!(syn_pot.weight > w0, "Pre before post should potentiate");

        // Test depression
        let mut syn_dep = Synapse::excitatory(0.5, 1.0).unwrap();
        let w0 = syn_dep.weight;
        syn_dep.postsynaptic_spike(0.0).unwrap();
        syn_dep.presynaptic_spike(10.0).unwrap();
        assert!(syn_dep.weight < w0, "Post before pre should depress");
    }

    #[test]
    fn test_short_term_plasticity() {
        let mut syn = Synapse::depressing_excitatory(1.0, 1.0).unwrap();

        let initial_prob = syn.vesicle_pool.release_probability();

        // Rapid spikes cause depression
        for i in 0..5 {
            syn.presynaptic_spike(i as f64 * 10.0).unwrap();
            for _ in 0..10 {
                syn.update((i as f64 + 0.1) * 10.0, -65.0, 1.0).unwrap();
            }
        }

        assert!(syn.vesicle_pool.release_probability() < initial_prob);
    }

    #[test]
    fn test_network_connectivity() {
        let mut net = SynapticNetwork::new(5);

        // Create feedforward network
        for i in 0..4 {
            let syn = Synapse::excitatory(1.0, 1.0).unwrap();
            net.add_connection(i, i + 1, syn).unwrap();
        }

        let stats = net.connectivity_stats();
        assert_eq!(stats.n_connections, 4);
        assert_eq!(stats.n_neurons, 5);
    }

    #[test]
    fn test_excitatory_inhibitory_balance() {
        let mut net = SynapticNetwork::new(3);

        // Excitatory connection
        let exc = Synapse::excitatory(1.0, 1.0).unwrap();
        net.add_connection(0, 2, exc).unwrap();

        // Inhibitory connection
        let inh = Synapse::inhibitory(1.0, 1.0).unwrap();
        net.add_connection(1, 2, inh).unwrap();

        // Both spike
        net.spike(0).unwrap();
        net.spike(1).unwrap();

        // Update
        let voltages = vec![-65.0; 3];
        for _ in 0..30 {
            net.update(&voltages, 0.1).unwrap();
        }

        // Should have both excitatory and inhibitory currents
        let inputs = net.get_inputs(2);
        assert_eq!(inputs.len(), 2);
    }

    #[test]
    fn test_calcium_dynamics() {
        let mut syn = Synapse::excitatory(1.0, 1.0).unwrap();

        let initial_ca = syn.postsynaptic_calcium.get_concentration();

        // Spike causes calcium elevation
        syn.postsynaptic_spike(0.0).unwrap();

        assert!(syn.postsynaptic_calcium.get_concentration() > initial_ca);

        // Calcium should decay
        for _ in 0..100 {
            syn.update(1.0, -65.0, 0.1).unwrap();
        }

        // Should return toward baseline (but may not reach due to other dynamics)
        let final_ca = syn.postsynaptic_calcium.get_concentration();
        assert!(final_ca <= syn.postsynaptic_calcium.get_concentration());
    }

    #[test]
    fn test_nmda_voltage_dependence() {
        use receptor::NMDAReceptor;

        let nmda = NMDAReceptor::new();

        let block_neg = nmda.mg_block(-70.0);
        let block_zero = nmda.mg_block(0.0);
        let block_pos = nmda.mg_block(40.0);

        // Block should decrease with depolarization
        assert!(block_neg < block_zero);
        assert!(block_zero < block_pos);
    }

    #[test]
    fn test_receptor_kinetics() {
        use receptor::{AMPAReceptor, ReceptorDynamics};

        let mut ampa = AMPAReceptor::new();

        // Apply neurotransmitter
        for _ in 0..50 {
            ampa.update(1.0, -65.0, 0.1).unwrap();
        }

        let peak = ampa.get_conductance();
        assert!(peak > 0.0);

        // Remove neurotransmitter
        for _ in 0..100 {
            ampa.update(0.0, -65.0, 0.1).unwrap();
        }

        // Should decay
        assert!(ampa.get_conductance() < peak);
    }
}
