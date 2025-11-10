//! Inter-population connectivity (projections).
//!
//! This module defines how populations connect to each other through synaptic projections.

use crate::connectivity::ConnectionPattern;
use crate::error::Result;
use rand::Rng;
use rand_distr::{Distribution, Normal, Uniform};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use synapse_models::synapse::Synapse;

/// A projection from a source population to a target population.
#[derive(Clone)]
pub struct Projection {
    /// Source population index
    pub source_pop: usize,
    /// Target population index
    pub target_pop: usize,
    /// Connection matrix (sparse representation)
    /// Each element is (source_idx, target_idx, synapse)
    connections: Vec<Connection>,
    /// Delay queue for spike propagation
    delay_queue: VecDeque<DelayedSpike>,
    /// Maximum delay in ms
    max_delay: f64,
}

/// A single synaptic connection with delay.
#[derive(Clone)]
pub struct Connection {
    /// Source neuron index in source population
    pub source: usize,
    /// Target neuron index in target population
    pub target: usize,
    /// Synaptic model
    pub synapse: Synapse,
    /// Transmission delay (ms)
    pub delay: f64,
    /// Connection weight (scaling factor)
    pub weight: f64,
}

/// A spike waiting to be delivered.
#[derive(Clone, Debug)]
struct DelayedSpike {
    /// Time when spike should be delivered
    delivery_time: f64,
    /// Source neuron index
    #[allow(dead_code)]
    source: usize,
    /// Target neuron index
    #[allow(dead_code)]
    target: usize,
    /// Connection index in connections vector
    connection_idx: usize,
}

impl Projection {
    /// Create a new projection with specified connection pattern.
    ///
    /// # Arguments
    ///
    /// * `source_pop` - Source population index
    /// * `target_pop` - Target population index
    /// * `source_size` - Number of neurons in source population
    /// * `target_size` - Number of neurons in target population
    /// * `pattern` - Connection pattern
    /// * `synapse_template` - Template synapse (will be cloned for each connection)
    /// * `weight_init` - Weight initialization scheme
    /// * `delay_init` - Delay initialization scheme
    /// * `rng` - Random number generator
    pub fn new<R: Rng>(
        source_pop: usize,
        target_pop: usize,
        source_size: usize,
        target_size: usize,
        pattern: &ConnectionPattern,
        synapse_template: &Synapse,
        weight_init: WeightInit,
        delay_init: DelayInit,
        rng: &mut R,
    ) -> Result<Self> {
        // Generate connection list based on pattern
        let connection_pairs = pattern.generate(source_size, target_size, rng)?;

        let mut connections = Vec::with_capacity(connection_pairs.len());
        let mut max_delay: f64 = 0.0;

        for (source, target) in connection_pairs {
            let synapse = synapse_template.clone();
            let weight = weight_init.sample(rng);
            let delay = delay_init.sample(rng);
            max_delay = max_delay.max(delay);

            connections.push(Connection {
                source,
                target,
                synapse,
                delay,
                weight,
            });
        }

        Ok(Self {
            source_pop,
            target_pop,
            connections,
            delay_queue: VecDeque::new(),
            max_delay,
        })
    }

    /// Create an all-to-all projection.
    pub fn all_to_all<R: Rng>(
        source_pop: usize,
        target_pop: usize,
        source_size: usize,
        target_size: usize,
        synapse_template: &Synapse,
        weight: f64,
        delay: f64,
        rng: &mut R,
    ) -> Result<Self> {
        Self::new(
            source_pop,
            target_pop,
            source_size,
            target_size,
            &ConnectionPattern::AllToAll,
            synapse_template,
            WeightInit::Constant(weight),
            DelayInit::Constant(delay),
            rng,
        )
    }

    /// Create a one-to-one projection.
    pub fn one_to_one<R: Rng>(
        source_pop: usize,
        target_pop: usize,
        size: usize,
        synapse_template: &Synapse,
        weight: f64,
        delay: f64,
        rng: &mut R,
    ) -> Result<Self> {
        Self::new(
            source_pop,
            target_pop,
            size,
            size,
            &ConnectionPattern::OneToOne,
            synapse_template,
            WeightInit::Constant(weight),
            DelayInit::Constant(delay),
            rng,
        )
    }

    /// Create a random fixed-probability projection.
    pub fn fixed_probability<R: Rng>(
        source_pop: usize,
        target_pop: usize,
        source_size: usize,
        target_size: usize,
        probability: f64,
        synapse_template: &Synapse,
        weight: f64,
        delay: f64,
        rng: &mut R,
    ) -> Result<Self> {
        Self::new(
            source_pop,
            target_pop,
            source_size,
            target_size,
            &ConnectionPattern::FixedProbability(probability),
            synapse_template,
            WeightInit::Constant(weight),
            DelayInit::Constant(delay),
            rng,
        )
    }

    /// Register a spike from source population.
    pub fn register_spike(&mut self, source_idx: usize, current_time: f64) {
        // Find all connections from this source neuron
        for (conn_idx, conn) in self.connections.iter().enumerate() {
            if conn.source == source_idx {
                self.delay_queue.push_back(DelayedSpike {
                    delivery_time: current_time + conn.delay,
                    source: source_idx,
                    target: conn.target,
                    connection_idx: conn_idx,
                });
            }
        }
    }

    /// Process delayed spikes and update synapses.
    ///
    /// Returns list of (target_idx, synaptic_current) pairs.
    pub fn process_spikes(&mut self, current_time: f64, target_voltages: &[f64], dt: f64) -> Result<Vec<(usize, f64)>> {
        let mut target_currents: Vec<f64> = vec![0.0; target_voltages.len()];

        // Deliver spikes whose time has come
        while let Some(spike) = self.delay_queue.front() {
            if spike.delivery_time <= current_time {
                let spike = self.delay_queue.pop_front().unwrap();
                // Trigger presynaptic spike in synapse
                let conn = &mut self.connections[spike.connection_idx];
                conn.synapse.presynaptic_spike(current_time)?;
            } else {
                break; // Queue is sorted by time
            }
        }

        // Update all synapses and collect currents
        for conn in self.connections.iter_mut() {
            let target_voltage = target_voltages[conn.target];
            conn.synapse.update(current_time, target_voltage, dt)?;

            let current = conn.synapse.current(target_voltage) * conn.weight;
            target_currents[conn.target] += current;
        }

        // Convert to sparse representation
        let result: Vec<(usize, f64)> = target_currents
            .iter()
            .enumerate()
            .filter(|(_, &current)| current.abs() > 1e-12)
            .map(|(idx, &current)| (idx, current))
            .collect();

        Ok(result)
    }

    /// Get number of connections.
    pub fn num_connections(&self) -> usize {
        self.connections.len()
    }

    /// Get maximum delay.
    pub fn max_delay(&self) -> f64 {
        self.max_delay
    }

    /// Get all connections.
    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    /// Apply STDP learning rule to all synapses.
    pub fn apply_stdp(&mut self, source_spike_times: &[Vec<f64>], target_spike_times: &[Vec<f64>]) -> Result<()> {
        for conn in self.connections.iter_mut() {
            let source_spikes = &source_spike_times[conn.source];
            let target_spikes = &target_spike_times[conn.target];

            // Apply STDP based on spike timing
            for &t_pre in source_spikes {
                for &t_post in target_spikes {
                    let dt = t_post - t_pre;
                    // STDP is handled by the synapse itself
                    // We just notify it of spike pairs
                    if dt.abs() < 40.0 { // STDP window typically ±20-40 ms
                        conn.synapse.presynaptic_spike(t_pre)?;
                        conn.synapse.postsynaptic_spike(t_post)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Scale all weights by a factor.
    pub fn scale_weights(&mut self, factor: f64) {
        for conn in self.connections.iter_mut() {
            conn.weight *= factor;
        }
    }

    /// Get weight statistics.
    pub fn weight_statistics(&self) -> WeightStats {
        if self.connections.is_empty() {
            return WeightStats {
                mean: 0.0,
                std: 0.0,
                min: 0.0,
                max: 0.0,
                count: 0,
            };
        }

        let weights: Vec<f64> = self.connections.iter().map(|c| c.weight).collect();
        let mean = weights.iter().sum::<f64>() / weights.len() as f64;
        let variance = weights.iter()
            .map(|w| (w - mean).powi(2))
            .sum::<f64>() / weights.len() as f64;
        let std = variance.sqrt();

        let min = weights.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = weights.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        WeightStats {
            mean,
            std,
            min,
            max,
            count: weights.len(),
        }
    }
}

/// Weight initialization schemes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WeightInit {
    /// Constant weight for all connections
    Constant(f64),
    /// Uniform distribution [min, max]
    Uniform { min: f64, max: f64 },
    /// Normal distribution
    Normal { mean: f64, std: f64 },
}

impl WeightInit {
    fn sample<R: Rng>(&self, rng: &mut R) -> f64 {
        match self {
            WeightInit::Constant(w) => *w,
            WeightInit::Uniform { min, max } => {
                Uniform::new(*min, *max).sample(rng)
            }
            WeightInit::Normal { mean, std } => {
                Normal::new(*mean, *std).unwrap().sample(rng).max(0.0)
            }
        }
    }
}

/// Delay initialization schemes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DelayInit {
    /// Constant delay for all connections
    Constant(f64),
    /// Uniform distribution [min, max]
    Uniform { min: f64, max: f64 },
    /// Distance-dependent delay
    DistanceDependent { speed: f64 },
}

impl DelayInit {
    fn sample<R: Rng>(&self, rng: &mut R) -> f64 {
        match self {
            DelayInit::Constant(d) => *d,
            DelayInit::Uniform { min, max } => {
                Uniform::new(*min, *max).sample(rng)
            }
            DelayInit::DistanceDependent { speed: _ } => {
                // For now, return constant (distance calculation requires spatial info)
                1.0
            }
        }
    }
}

/// Weight statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightStats {
    pub mean: f64,
    pub std: f64,
    pub min: f64,
    pub max: f64,
    pub count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_to_all_projection() {
        let mut rng = rand::thread_rng();
        let synapse = Synapse::excitatory(1.0, 1.0).unwrap();

        let proj = Projection::all_to_all(0, 1, 3, 2, &synapse, 1.0, 0.5, &mut rng).unwrap();

        assert_eq!(proj.num_connections(), 6); // 3 * 2
        assert_eq!(proj.source_pop, 0);
        assert_eq!(proj.target_pop, 1);
    }

    #[test]
    fn test_one_to_one_projection() {
        let mut rng = rand::thread_rng();
        let synapse = Synapse::excitatory(1.0, 1.0).unwrap();

        let proj = Projection::one_to_one(0, 1, 5, &synapse, 1.0, 0.5, &mut rng).unwrap();

        assert_eq!(proj.num_connections(), 5);
    }

    #[test]
    fn test_fixed_probability_projection() {
        let mut rng = rand::thread_rng();
        let synapse = Synapse::excitatory(1.0, 1.0).unwrap();

        let proj = Projection::fixed_probability(
            0, 1, 10, 10, 0.5, &synapse, 1.0, 0.5, &mut rng
        ).unwrap();

        // Probabilistic, but should have roughly 50 connections (10*10*0.5)
        let n_conn = proj.num_connections();
        assert!(n_conn > 20 && n_conn < 80);
    }

    #[test]
    fn test_spike_registration() {
        let mut rng = rand::thread_rng();
        let synapse = Synapse::excitatory(1.0, 1.0).unwrap();

        let mut proj = Projection::one_to_one(0, 1, 3, &synapse, 1.0, 1.0, &mut rng).unwrap();

        proj.register_spike(0, 0.0);
        assert_eq!(proj.delay_queue.len(), 1);

        proj.register_spike(1, 1.0);
        assert_eq!(proj.delay_queue.len(), 2);
    }

    #[test]
    fn test_spike_processing_with_delay() {
        let mut rng = rand::thread_rng();
        let synapse = Synapse::excitatory(1.0, 1.0).unwrap();

        let mut proj = Projection::one_to_one(0, 1, 2, &synapse, 1.0, 2.0, &mut rng).unwrap();

        // Spike at t=0
        proj.register_spike(0, 0.0);

        // Process at t=1.0 (before delay)
        let voltages = vec![-65.0; 2];
        let currents = proj.process_spikes(1.0, &voltages, 0.1).unwrap();
        assert!(currents.is_empty()); // Delay not yet elapsed

        // Process at t=2.5 (after delay)
        let currents = proj.process_spikes(2.5, &voltages, 0.1).unwrap();
        // Spike should have been delivered
        assert_eq!(proj.delay_queue.len(), 0);
    }

    #[test]
    fn test_weight_scaling() {
        let mut rng = rand::thread_rng();
        let synapse = Synapse::excitatory(1.0, 1.0).unwrap();

        let mut proj = Projection::all_to_all(0, 1, 2, 2, &synapse, 2.0, 0.5, &mut rng).unwrap();

        proj.scale_weights(0.5);

        for conn in proj.connections() {
            assert!((conn.weight - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_weight_statistics() {
        let mut rng = rand::thread_rng();
        let synapse = Synapse::excitatory(1.0, 1.0).unwrap();

        let proj = Projection::new(
            0, 1, 5, 5,
            &ConnectionPattern::AllToAll,
            &synapse,
            WeightInit::Normal { mean: 1.0, std: 0.1 },
            DelayInit::Constant(1.0),
            &mut rng,
        ).unwrap();

        let stats = proj.weight_statistics();
        assert_eq!(stats.count, 25);
        assert!(stats.mean > 0.0);
    }
}
