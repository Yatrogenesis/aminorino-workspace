//! Network-level synaptic dynamics.
//!
//! This module provides structures for managing synaptic networks including:
//! - Chemical synapses
//! - Gap junctions (electrical synapses)
//! - Ephaptic coupling
//! - Neuromodulation
//! - Spike propagation with delays

use crate::error::{Result, SynapseError};
use crate::synapse::Synapse;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

/// Synaptic connection in a network.
#[derive(Debug, Clone)]
pub struct Connection {
    /// Presynaptic neuron ID.
    pub pre_neuron: usize,

    /// Postsynaptic neuron ID.
    pub post_neuron: usize,

    /// Synapse model.
    pub synapse: Synapse,
}

/// Gap junction (electrical synapse).
///
/// Provides bidirectional, instantaneous electrical coupling between neurons.
#[derive(Debug, Clone)]
pub struct GapJunction {
    /// First neuron ID.
    pub neuron1: usize,

    /// Second neuron ID.
    pub neuron2: usize,

    /// Coupling conductance (nS).
    pub conductance: f64,

    /// Whether the junction is rectifying (one-way).
    pub rectifying: bool,
}

impl GapJunction {
    /// Create a new gap junction.
    pub fn new(neuron1: usize, neuron2: usize, conductance: f64) -> Result<Self> {
        if conductance < 0.0 {
            return Err(SynapseError::InvalidWeight(conductance, 0.0, f64::INFINITY));
        }

        Ok(Self {
            neuron1,
            neuron2,
            conductance,
            rectifying: false,
        })
    }

    /// Calculate current from neuron1 to neuron2.
    pub fn current(&self, v1: f64, v2: f64) -> f64 {
        let i12 = self.conductance * (v2 - v1);

        if self.rectifying && i12 < 0.0 {
            0.0
        } else {
            i12
        }
    }
}

/// Ephaptic coupling - electric field effects between nearby neurons.
///
/// Models extracellular potential effects on neighboring neurons.
#[derive(Debug, Clone)]
pub struct EphapticCoupling {
    /// Source neuron ID.
    pub source_neuron: usize,

    /// Target neuron ID.
    pub target_neuron: usize,

    /// Coupling strength (mV/Hz).
    pub strength: f64,

    /// Distance between neurons (μm).
    pub distance: f64,

    /// Time constant for field effects (ms).
    pub tau: f64,
}

impl EphapticCoupling {
    /// Create new ephaptic coupling.
    pub fn new(source: usize, target: usize, distance: f64) -> Self {
        // Strength decays with distance
        let strength = 0.01 / (1.0 + distance / 100.0);

        Self {
            source_neuron: source,
            target_neuron: target,
            strength,
            distance,
            tau: 10.0, // 10 ms time constant
        }
    }

    /// Calculate ephaptic effect on target neuron.
    ///
    /// # Arguments
    /// * `source_rate` - Firing rate of source neuron (Hz)
    pub fn effect(&self, source_rate: f64) -> f64 {
        self.strength * source_rate
    }
}

/// Neuromodulator state.
#[derive(Debug, Clone)]
pub struct NeuromodulatorState {
    /// Dopamine concentration (μM).
    pub dopamine: f64,

    /// Serotonin concentration (μM).
    pub serotonin: f64,

    /// Acetylcholine concentration (μM).
    pub acetylcholine: f64,

    /// Norepinephrine concentration (μM).
    pub norepinephrine: f64,

    /// Time constants for decay (ms).
    pub tau_decay: f64,
}

impl Default for NeuromodulatorState {
    fn default() -> Self {
        Self {
            dopamine: 0.0,
            serotonin: 0.0,
            acetylcholine: 0.0,
            norepinephrine: 0.0,
            tau_decay: 1000.0, // Slow decay
        }
    }
}

impl NeuromodulatorState {
    /// Create new neuromodulator state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update neuromodulator concentrations.
    ///
    /// # Arguments
    /// * `dt` - Time step (ms)
    pub fn update(&mut self, dt: f64) {
        // Exponential decay
        let decay = (-dt / self.tau_decay).exp();
        self.dopamine *= decay;
        self.serotonin *= decay;
        self.acetylcholine *= decay;
        self.norepinephrine *= decay;
    }

    /// Release dopamine.
    pub fn release_dopamine(&mut self, amount: f64) {
        self.dopamine += amount;
    }

    /// Release serotonin.
    pub fn release_serotonin(&mut self, amount: f64) {
        self.serotonin += amount;
    }

    /// Release acetylcholine.
    pub fn release_acetylcholine(&mut self, amount: f64) {
        self.acetylcholine += amount;
    }

    /// Release norepinephrine.
    pub fn release_norepinephrine(&mut self, amount: f64) {
        self.norepinephrine += amount;
    }

    /// Get total neuromodulatory effect on synaptic weight.
    pub fn weight_modulation(&self) -> f64 {
        // Dopamine enhances learning
        let da_effect = 1.0 + 0.5 * self.dopamine.min(1.0);

        // Serotonin slightly suppresses
        let sero_effect = 1.0 - 0.2 * self.serotonin.min(1.0);

        // ACh enhances
        let ach_effect = 1.0 + 0.3 * self.acetylcholine.min(1.0);

        // NE enhances
        let ne_effect = 1.0 + 0.4 * self.norepinephrine.min(1.0);

        da_effect * sero_effect * ach_effect * ne_effect
    }
}

/// Delayed spike event.
#[derive(Debug, Clone, Copy)]
struct DelayedSpike {
    /// Target neuron ID.
    target_neuron: usize,

    /// Connection index.
    connection_idx: usize,

    /// Arrival time (ms).
    arrival_time: f64,
}

/// Synaptic network.
///
/// Manages a network of neurons connected by synapses.
pub struct SynapticNetwork {
    /// Number of neurons in the network.
    pub n_neurons: usize,

    /// Chemical synaptic connections.
    pub connections: Vec<Connection>,

    /// Gap junctions.
    pub gap_junctions: Vec<GapJunction>,

    /// Ephaptic couplings.
    pub ephaptic_couplings: Vec<EphapticCoupling>,

    /// Neuromodulator state.
    pub neuromodulators: NeuromodulatorState,

    /// Current time (ms).
    current_time: f64,

    /// Queue of delayed spikes.
    spike_queue: VecDeque<DelayedSpike>,

    /// Adjacency list: neuron -> connections.
    adjacency: HashMap<usize, Vec<usize>>,

    /// Neuron voltages (mV).
    neuron_voltages: Vec<f64>,
}

impl SynapticNetwork {
    /// Create a new synaptic network.
    ///
    /// # Arguments
    /// * `n_neurons` - Number of neurons in the network
    pub fn new(n_neurons: usize) -> Self {
        Self {
            n_neurons,
            connections: Vec::new(),
            gap_junctions: Vec::new(),
            ephaptic_couplings: Vec::new(),
            neuromodulators: NeuromodulatorState::new(),
            current_time: 0.0,
            spike_queue: VecDeque::new(),
            adjacency: HashMap::new(),
            neuron_voltages: vec![-65.0; n_neurons],
        }
    }

    /// Add a chemical synapse connection.
    pub fn add_connection(&mut self, pre: usize, post: usize, synapse: Synapse) -> Result<()> {
        if pre >= self.n_neurons {
            return Err(SynapseError::NeuronNotFound(pre));
        }
        if post >= self.n_neurons {
            return Err(SynapseError::NeuronNotFound(post));
        }

        let conn_idx = self.connections.len();
        self.connections.push(Connection {
            pre_neuron: pre,
            post_neuron: post,
            synapse,
        });

        // Update adjacency list
        self.adjacency.entry(pre).or_insert_with(Vec::new).push(conn_idx);

        Ok(())
    }

    /// Add a gap junction.
    pub fn add_gap_junction(&mut self, neuron1: usize, neuron2: usize, conductance: f64) -> Result<()> {
        if neuron1 >= self.n_neurons || neuron2 >= self.n_neurons {
            return Err(SynapseError::NeuronNotFound(neuron1.max(neuron2)));
        }

        self.gap_junctions.push(GapJunction::new(neuron1, neuron2, conductance)?);
        Ok(())
    }

    /// Add ephaptic coupling.
    pub fn add_ephaptic_coupling(&mut self, source: usize, target: usize, distance: f64) -> Result<()> {
        if source >= self.n_neurons || target >= self.n_neurons {
            return Err(SynapseError::NeuronNotFound(source.max(target)));
        }

        self.ephaptic_couplings.push(EphapticCoupling::new(source, target, distance));
        Ok(())
    }

    /// Process spike from a neuron.
    ///
    /// # Arguments
    /// * `neuron_id` - ID of spiking neuron
    pub fn spike(&mut self, neuron_id: usize) -> Result<()> {
        if neuron_id >= self.n_neurons {
            return Err(SynapseError::NeuronNotFound(neuron_id));
        }

        // Find all connections from this neuron
        if let Some(conn_indices) = self.adjacency.get(&neuron_id) {
            for &conn_idx in conn_indices {
                let conn = &mut self.connections[conn_idx];
                let delay = conn.synapse.delay;

                // Trigger presynaptic spike
                conn.synapse.presynaptic_spike(self.current_time)?;

                // Schedule delayed spike arrival
                self.spike_queue.push_back(DelayedSpike {
                    target_neuron: conn.post_neuron,
                    connection_idx: conn_idx,
                    arrival_time: self.current_time + delay,
                });
            }
        }

        Ok(())
    }

    /// Update network dynamics.
    ///
    /// # Arguments
    /// * `neuron_voltages` - Current voltage of each neuron (mV)
    /// * `dt` - Time step (ms)
    pub fn update(&mut self, neuron_voltages: &[f64], dt: f64) -> Result<()> {
        if neuron_voltages.len() != self.n_neurons {
            return Err(SynapseError::InvalidNetwork(
                format!("Expected {} voltages, got {}", self.n_neurons, neuron_voltages.len())
            ));
        }

        self.neuron_voltages.copy_from_slice(neuron_voltages);
        self.current_time += dt;

        // Process delayed spikes that have arrived
        while let Some(spike) = self.spike_queue.front() {
            if spike.arrival_time <= self.current_time {
                let _spike = self.spike_queue.pop_front().unwrap();
                // Spike has arrived - already processed in synapse
            } else {
                break;
            }
        }

        // Update all synapses
        for conn in &mut self.connections {
            let post_voltage = self.neuron_voltages[conn.post_neuron];
            conn.synapse.update(self.current_time, post_voltage, dt)?;
        }

        // Update neuromodulators
        self.neuromodulators.update(dt);

        Ok(())
    }

    /// Get synaptic current to a neuron.
    ///
    /// # Arguments
    /// * `neuron_id` - Target neuron ID
    ///
    /// # Returns
    /// Total synaptic current (pA)
    pub fn get_synaptic_current(&self, neuron_id: usize) -> Result<f64> {
        if neuron_id >= self.n_neurons {
            return Err(SynapseError::NeuronNotFound(neuron_id));
        }

        let voltage = self.neuron_voltages[neuron_id];
        let mut total_current = 0.0;

        // Chemical synapses
        for conn in &self.connections {
            if conn.post_neuron == neuron_id {
                let modulation = self.neuromodulators.weight_modulation();
                total_current += conn.synapse.current(voltage) * modulation;
            }
        }

        // Gap junctions
        for gap in &self.gap_junctions {
            if gap.neuron1 == neuron_id {
                let v_other = self.neuron_voltages[gap.neuron2];
                total_current += gap.current(voltage, v_other);
            } else if gap.neuron2 == neuron_id {
                let v_other = self.neuron_voltages[gap.neuron1];
                total_current -= gap.current(v_other, voltage);
            }
        }

        Ok(total_current)
    }

    /// Get all synaptic connections to a neuron.
    pub fn get_inputs(&self, neuron_id: usize) -> Vec<&Connection> {
        self.connections
            .iter()
            .filter(|c| c.post_neuron == neuron_id)
            .collect()
    }

    /// Get all synaptic connections from a neuron.
    pub fn get_outputs(&self, neuron_id: usize) -> Vec<&Connection> {
        self.connections
            .iter()
            .filter(|c| c.pre_neuron == neuron_id)
            .collect()
    }

    /// Get connectivity statistics.
    pub fn connectivity_stats(&self) -> NetworkStats {
        let mut in_degrees = vec![0; self.n_neurons];
        let mut out_degrees = vec![0; self.n_neurons];

        for conn in &self.connections {
            out_degrees[conn.pre_neuron] += 1;
            in_degrees[conn.post_neuron] += 1;
        }

        NetworkStats {
            n_neurons: self.n_neurons,
            n_connections: self.connections.len(),
            n_gap_junctions: self.gap_junctions.len(),
            n_ephaptic: self.ephaptic_couplings.len(),
            avg_in_degree: in_degrees.iter().sum::<usize>() as f64 / self.n_neurons as f64,
            avg_out_degree: out_degrees.iter().sum::<usize>() as f64 / self.n_neurons as f64,
        }
    }

    /// Reset all synapses in the network.
    pub fn reset(&mut self) {
        for conn in &mut self.connections {
            conn.synapse.reset();
        }
        self.spike_queue.clear();
        self.current_time = 0.0;
        self.neuron_voltages.fill(-65.0);
    }
}

/// Network connectivity statistics.
#[derive(Debug, Clone)]
pub struct NetworkStats {
    /// Number of neurons.
    pub n_neurons: usize,

    /// Number of chemical synapses.
    pub n_connections: usize,

    /// Number of gap junctions.
    pub n_gap_junctions: usize,

    /// Number of ephaptic couplings.
    pub n_ephaptic: usize,

    /// Average in-degree.
    pub avg_in_degree: f64,

    /// Average out-degree.
    pub avg_out_degree: f64,
}

/// Thread-safe wrapper for synaptic network.
pub type SharedNetwork = Arc<Mutex<SynapticNetwork>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_creation() {
        let net = SynapticNetwork::new(10);
        assert_eq!(net.n_neurons, 10);
        assert_eq!(net.connections.len(), 0);
    }

    #[test]
    fn test_add_connection() {
        let mut net = SynapticNetwork::new(3);
        let syn = Synapse::excitatory(1.0, 1.0).unwrap();

        net.add_connection(0, 1, syn).unwrap();
        assert_eq!(net.connections.len(), 1);
    }

    #[test]
    fn test_add_gap_junction() {
        let mut net = SynapticNetwork::new(3);
        net.add_gap_junction(0, 1, 0.5).unwrap();

        assert_eq!(net.gap_junctions.len(), 1);
        assert_eq!(net.gap_junctions[0].conductance, 0.5);
    }

    #[test]
    fn test_spike_propagation() {
        let mut net = SynapticNetwork::new(2);
        let syn = Synapse::excitatory(1.0, 1.0).unwrap();
        net.add_connection(0, 1, syn).unwrap();

        // Spike from neuron 0
        net.spike(0).unwrap();

        // Should have scheduled spike
        assert_eq!(net.spike_queue.len(), 1);
    }

    #[test]
    fn test_network_update() {
        let mut net = SynapticNetwork::new(2);
        let syn = Synapse::excitatory(1.0, 1.0).unwrap();
        net.add_connection(0, 1, syn).unwrap();

        let voltages = vec![-65.0, -65.0];
        net.update(&voltages, 0.1).unwrap();

        assert_eq!(net.current_time, 0.1);
    }

    #[test]
    fn test_neuromodulator_state() {
        let mut nm = NeuromodulatorState::new();

        nm.release_dopamine(0.5);
        assert_eq!(nm.dopamine, 0.5);

        // Should decay
        nm.update(100.0);
        assert!(nm.dopamine < 0.5);
    }

    #[test]
    fn test_gap_junction_current() {
        let gap = GapJunction::new(0, 1, 1.0).unwrap();

        let i = gap.current(-70.0, -60.0);
        // Current from lower to higher voltage
        assert!(i > 0.0);
    }

    #[test]
    fn test_connectivity_stats() {
        let mut net = SynapticNetwork::new(3);

        let syn1 = Synapse::excitatory(1.0, 1.0).unwrap();
        let syn2 = Synapse::excitatory(1.0, 1.0).unwrap();

        net.add_connection(0, 1, syn1).unwrap();
        net.add_connection(1, 2, syn2).unwrap();

        let stats = net.connectivity_stats();
        assert_eq!(stats.n_connections, 2);
        assert_eq!(stats.n_neurons, 3);
    }

    #[test]
    fn test_ephaptic_coupling() {
        let eph = EphapticCoupling::new(0, 1, 50.0);

        let effect = eph.effect(10.0); // 10 Hz
        assert!(effect > 0.0);
    }
}
