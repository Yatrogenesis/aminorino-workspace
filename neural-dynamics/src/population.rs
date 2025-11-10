//! Neural population management.
//!
//! This module provides structures for managing groups of neurons with similar properties.

use crate::error::{NeuralDynamicsError, Result};
use hodgkin_huxley::{HodgkinHuxleyNeuron, neuron_types::NeuronConfig};
use rand::Rng;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Neural population with homogeneous or heterogeneous neuron parameters.
#[derive(Clone)]
pub struct NeuralPopulation {
    /// Population identifier
    pub id: String,
    /// Number of neurons in population
    pub size: usize,
    /// Neurons in this population
    neurons: Vec<HodgkinHuxleyNeuron>,
    /// External currents for each neuron (µA/cm²)
    external_currents: Vec<f64>,
    /// Synaptic currents from network (µA/cm²)
    synaptic_currents: Vec<f64>,
    /// Spike times for each neuron
    spike_times: Vec<Vec<f64>>,
    /// Spike threshold for detection (mV)
    spike_threshold: f64,
    /// Last spike detection state
    last_above_threshold: Vec<bool>,
    /// Voltage recording (optional)
    voltage_trace: Option<Vec<Vec<f64>>>,
    /// Time points for recordings
    recorded_times: Vec<f64>,
}

impl NeuralPopulation {
    /// Create a homogeneous population with identical neurons.
    ///
    /// # Arguments
    ///
    /// * `id` - Population identifier
    /// * `size` - Number of neurons
    /// * `config` - Neuron configuration (same for all neurons)
    pub fn new_homogeneous(id: impl Into<String>, size: usize, config: NeuronConfig) -> Result<Self> {
        if size == 0 {
            return Err(NeuralDynamicsError::EmptyPopulation);
        }

        let neurons: Result<Vec<HodgkinHuxleyNeuron>> = (0..size)
            .map(|_| HodgkinHuxleyNeuron::new(config.clone()).map_err(|e| e.into()))
            .collect();
        let mut neurons: Vec<HodgkinHuxleyNeuron> = neurons?;

        // Initialize all neurons at rest
        for neuron in neurons.iter_mut() {
            neuron.initialize_rest();
        }

        Ok(Self {
            id: id.into(),
            size,
            neurons,
            external_currents: vec![0.0; size],
            synaptic_currents: vec![0.0; size],
            spike_times: vec![Vec::new(); size],
            spike_threshold: -20.0,
            last_above_threshold: vec![false; size],
            voltage_trace: None,
            recorded_times: Vec::new(),
        })
    }

    /// Create a heterogeneous population with varied parameters.
    ///
    /// # Arguments
    ///
    /// * `id` - Population identifier
    /// * `size` - Number of neurons
    /// * `base_config` - Base neuron configuration
    /// * `variability` - Parameter variability (0.0 = homogeneous, 0.2 = 20% std dev)
    /// * `rng` - Random number generator
    pub fn new_heterogeneous<R: Rng>(
        id: impl Into<String>,
        size: usize,
        base_config: NeuronConfig,
        variability: f64,
        rng: &mut R,
    ) -> Result<Self> {
        if size == 0 {
            return Err(NeuralDynamicsError::EmptyPopulation);
        }

        if variability < 0.0 {
            return Err(NeuralDynamicsError::InvalidParameter {
                parameter: "variability".to_string(),
                value: variability,
                reason: "must be non-negative".to_string(),
            });
        }

        let mut neurons = Vec::with_capacity(size);

        for _ in 0..size {
            let mut config = base_config.clone();

            // Add variability to conductances
            if variability > 0.0 {
                let g_na_dist = Normal::new(config.na_channel.g_max, config.na_channel.g_max * variability)
                    .map_err(|e| NeuralDynamicsError::InvalidParameter {
                        parameter: "g_na variability".to_string(),
                        value: variability,
                        reason: e.to_string(),
                    })?;
                config.na_channel.g_max = g_na_dist.sample(rng).max(0.0);

                let g_k_dist = Normal::new(config.k_channel.g_max, config.k_channel.g_max * variability)
                    .map_err(|e| NeuralDynamicsError::InvalidParameter {
                        parameter: "g_k variability".to_string(),
                        value: variability,
                        reason: e.to_string(),
                    })?;
                config.k_channel.g_max = g_k_dist.sample(rng).max(0.0);
            }

            let mut neuron = HodgkinHuxleyNeuron::new(config)?;
            neuron.initialize_rest();
            neurons.push(neuron);
        }

        Ok(Self {
            id: id.into(),
            size,
            neurons,
            external_currents: vec![0.0; size],
            synaptic_currents: vec![0.0; size],
            spike_times: vec![Vec::new(); size],
            spike_threshold: -20.0,
            last_above_threshold: vec![false; size],
            voltage_trace: None,
            recorded_times: Vec::new(),
        })
    }

    /// Create an excitatory population (regular spiking).
    pub fn excitatory(id: impl Into<String>, size: usize) -> Result<Self> {
        Self::new_homogeneous(id, size, NeuronConfig::regular_spiking())
    }

    /// Create an inhibitory population (fast spiking).
    pub fn inhibitory(id: impl Into<String>, size: usize) -> Result<Self> {
        Self::new_homogeneous(id, size, NeuronConfig::fast_spiking())
    }

    /// Enable voltage recording for all neurons.
    pub fn enable_recording(&mut self) {
        self.voltage_trace = Some(vec![Vec::new(); self.size]);
        self.recorded_times.clear();
    }

    /// Disable voltage recording.
    pub fn disable_recording(&mut self) {
        self.voltage_trace = None;
        self.recorded_times.clear();
    }

    /// Set external current for a specific neuron.
    pub fn set_external_current(&mut self, index: usize, current: f64) -> Result<()> {
        if index >= self.size {
            return Err(NeuralDynamicsError::InvalidNeuronIndex {
                index,
                max: self.size - 1,
            });
        }
        self.external_currents[index] = current;
        Ok(())
    }

    /// Set external current for all neurons.
    pub fn set_external_currents(&mut self, currents: &[f64]) -> Result<()> {
        if currents.len() != self.size {
            return Err(NeuralDynamicsError::SizeMismatch {
                expected: self.size,
                actual: currents.len(),
            });
        }
        self.external_currents.copy_from_slice(currents);
        Ok(())
    }

    /// Add to external current for a specific neuron.
    pub fn add_external_current(&mut self, index: usize, current: f64) -> Result<()> {
        if index >= self.size {
            return Err(NeuralDynamicsError::InvalidNeuronIndex {
                index,
                max: self.size - 1,
            });
        }
        self.external_currents[index] += current;
        Ok(())
    }

    /// Set synaptic current for a specific neuron (called by network).
    pub fn set_synaptic_current(&mut self, index: usize, current: f64) -> Result<()> {
        if index >= self.size {
            return Err(NeuralDynamicsError::InvalidNeuronIndex {
                index,
                max: self.size - 1,
            });
        }
        self.synaptic_currents[index] = current;
        Ok(())
    }

    /// Reset synaptic currents (typically called before network updates).
    pub fn reset_synaptic_currents(&mut self) {
        self.synaptic_currents.fill(0.0);
    }

    /// Get voltage of a specific neuron.
    pub fn get_voltage(&self, index: usize) -> Result<f64> {
        if index >= self.size {
            return Err(NeuralDynamicsError::InvalidNeuronIndex {
                index,
                max: self.size - 1,
            });
        }
        Ok(self.neurons[index].voltage())
    }

    /// Get voltages of all neurons.
    pub fn get_voltages(&self) -> Vec<f64> {
        self.neurons.iter().map(|n| n.voltage()).collect()
    }

    /// Get spike times for a specific neuron.
    pub fn get_spike_times(&self, index: usize) -> Result<&[f64]> {
        if index >= self.size {
            return Err(NeuralDynamicsError::InvalidNeuronIndex {
                index,
                max: self.size - 1,
            });
        }
        Ok(&self.spike_times[index])
    }

    /// Get all spike times.
    pub fn get_all_spike_times(&self) -> &[Vec<f64>] {
        &self.spike_times
    }

    /// Update population for one time step (sequential).
    pub fn update(&mut self, dt: f64, current_time: f64) -> Result<()> {
        for i in 0..self.size {
            let total_current = self.external_currents[i] + self.synaptic_currents[i];
            self.neurons[i].step(dt, total_current)?;

            // Detect spikes
            let v = self.neurons[i].voltage();
            if !self.last_above_threshold[i] && v > self.spike_threshold {
                self.spike_times[i].push(current_time);
                self.last_above_threshold[i] = true;
            } else if self.last_above_threshold[i] && v <= self.spike_threshold {
                self.last_above_threshold[i] = false;
            }
        }

        // Record voltages if enabled
        if let Some(ref mut traces) = self.voltage_trace {
            for (i, neuron) in self.neurons.iter().enumerate() {
                traces[i].push(neuron.voltage());
            }
            self.recorded_times.push(current_time);
        }

        Ok(())
    }

    /// Update population for one time step (parallel).
    pub fn update_parallel(&mut self, dt: f64, current_time: f64) -> Result<()> {
        // Parallel update of neuron dynamics
        let neurons_mutex = Arc::new(Mutex::new(&mut self.neurons));
        let results: Vec<Result<f64>> = (0..self.size)
            .into_par_iter()
            .map(|i| {
                let total_current = self.external_currents[i] + self.synaptic_currents[i];
                let mut neurons = neurons_mutex.lock().unwrap();
                neurons[i].step(dt, total_current)?;
                Ok(neurons[i].voltage())
            })
            .collect();

        // Check for errors and collect voltages
        let voltages: Result<Vec<_>> = results.into_iter().collect();
        let voltages = voltages?;

        // Sequential spike detection (typically fast)
        for (i, &v) in voltages.iter().enumerate() {
            if !self.last_above_threshold[i] && v > self.spike_threshold {
                self.spike_times[i].push(current_time);
                self.last_above_threshold[i] = true;
            } else if self.last_above_threshold[i] && v <= self.spike_threshold {
                self.last_above_threshold[i] = false;
            }
        }

        // Record voltages if enabled
        if let Some(ref mut traces) = self.voltage_trace {
            for (i, &v) in voltages.iter().enumerate() {
                traces[i].push(v);
            }
            self.recorded_times.push(current_time);
        }

        Ok(())
    }

    /// Get recorded voltage traces.
    pub fn get_voltage_traces(&self) -> Option<(&[Vec<f64>], &[f64])> {
        self.voltage_trace.as_ref().map(|traces| (traces.as_slice(), self.recorded_times.as_slice()))
    }

    /// Clear all recordings and spike times.
    pub fn clear_history(&mut self) {
        self.spike_times.iter_mut().for_each(|v| v.clear());
        if let Some(ref mut traces) = self.voltage_trace {
            traces.iter_mut().for_each(|v| v.clear());
        }
        self.recorded_times.clear();
    }

    /// Reset all neurons to resting state.
    pub fn reset(&mut self) {
        for neuron in self.neurons.iter_mut() {
            neuron.initialize_rest();
        }
        self.external_currents.fill(0.0);
        self.synaptic_currents.fill(0.0);
        self.last_above_threshold.fill(false);
        self.clear_history();
    }

    /// Calculate population statistics.
    pub fn statistics(&self, time_window: Option<(f64, f64)>) -> PopulationStats {
        let voltages = self.get_voltages();
        let mean_voltage = voltages.iter().sum::<f64>() / self.size as f64;
        let voltage_std = (voltages.iter()
            .map(|v| (v - mean_voltage).powi(2))
            .sum::<f64>() / self.size as f64)
            .sqrt();

        // Count spikes in time window
        let (total_spikes, active_neurons) = if let Some((t_start, t_end)) = time_window {
            let mut total = 0;
            let mut active = 0;
            for spike_train in &self.spike_times {
                let count = spike_train.iter().filter(|&&t| t >= t_start && t < t_end).count();
                if count > 0 {
                    active += 1;
                    total += count;
                }
            }
            (total, active)
        } else {
            let total: usize = self.spike_times.iter().map(|v| v.len()).sum();
            let active = self.spike_times.iter().filter(|v| !v.is_empty()).count();
            (total, active)
        };

        PopulationStats {
            size: self.size,
            mean_voltage,
            voltage_std,
            total_spikes,
            active_neurons,
            firing_rate: 0.0, // Calculated if time window provided
        }
    }

    /// Calculate instantaneous population firing rate.
    pub fn instantaneous_rate(&self, time: f64, window: f64) -> f64 {
        let t_start = time - window / 2.0;
        let t_end = time + window / 2.0;

        let spike_count: usize = self.spike_times
            .iter()
            .map(|spikes| spikes.iter().filter(|&&t| t >= t_start && t < t_end).count())
            .sum();

        (spike_count as f64 / (self.size as f64 * window)) * 1000.0 // Hz
    }
}

/// Population statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationStats {
    pub size: usize,
    pub mean_voltage: f64,
    pub voltage_std: f64,
    pub total_spikes: usize,
    pub active_neurons: usize,
    pub firing_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_create_homogeneous_population() {
        let pop = NeuralPopulation::excitatory("E", 10).unwrap();
        assert_eq!(pop.size, 10);
        assert_eq!(pop.id, "E");
        assert_eq!(pop.neurons.len(), 10);
    }

    #[test]
    fn test_create_heterogeneous_population() {
        let mut rng = rand::thread_rng();
        let pop = NeuralPopulation::new_heterogeneous(
            "E",
            20,
            NeuronConfig::regular_spiking(),
            0.2,
            &mut rng,
        )
        .unwrap();
        assert_eq!(pop.size, 20);
    }

    #[test]
    fn test_empty_population_fails() {
        let result = NeuralPopulation::excitatory("E", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_external_current() {
        let mut pop = NeuralPopulation::excitatory("E", 5).unwrap();
        pop.set_external_current(2, 10.0).unwrap();
        assert_eq!(pop.external_currents[2], 10.0);

        // Out of bounds
        assert!(pop.set_external_current(10, 5.0).is_err());
    }

    #[test]
    fn test_population_update() {
        let mut pop = NeuralPopulation::excitatory("E", 3).unwrap();
        pop.set_external_current(0, 20.0).unwrap();

        for i in 0..1000 {
            pop.update(0.01, i as f64 * 0.01).unwrap();
        }

        // Neuron 0 should have spiked with strong current
        assert!(!pop.spike_times[0].is_empty());
        // Other neurons should be quiet
        assert!(pop.spike_times[1].is_empty());
    }

    #[test]
    fn test_voltage_recording() {
        let mut pop = NeuralPopulation::excitatory("E", 2).unwrap();
        pop.enable_recording();

        for i in 0..10 {
            pop.update(0.1, i as f64 * 0.1).unwrap();
        }

        let (traces, times) = pop.get_voltage_traces().unwrap();
        assert_eq!(traces.len(), 2);
        assert_eq!(times.len(), 10);
        assert_eq!(traces[0].len(), 10);
    }

    #[test]
    fn test_population_reset() {
        let mut pop = NeuralPopulation::excitatory("E", 3).unwrap();
        pop.set_external_current(0, 10.0).unwrap();
        pop.update(0.01, 0.01).unwrap();

        pop.reset();
        assert_eq!(pop.external_currents[0], 0.0);
        assert!(pop.spike_times[0].is_empty());
    }

    #[test]
    fn test_population_statistics() {
        let mut pop = NeuralPopulation::excitatory("E", 5).unwrap();
        let stats = pop.statistics(None);
        assert_eq!(stats.size, 5);
        assert!(stats.mean_voltage < 0.0); // Should be at rest
    }

    #[test]
    fn test_instantaneous_rate() {
        let mut pop = NeuralPopulation::excitatory("E", 10).unwrap();

        // Inject currents to cause spikes
        for i in 0..10 {
            pop.set_external_current(i, 15.0).unwrap();
        }

        for i in 0..5000 {
            pop.update(0.01, i as f64 * 0.01).unwrap();
        }

        let rate = pop.instantaneous_rate(25.0, 10.0);
        assert!(rate > 0.0); // Should have some activity
    }
}
