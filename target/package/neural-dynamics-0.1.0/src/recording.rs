//! Data recording and analysis utilities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Records voltage traces from neurons.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoltageRecorder {
    /// Time points
    pub times: Vec<f64>,
    /// Voltage traces for each recorded neuron
    pub traces: HashMap<(usize, usize), Vec<f64>>, // (pop_idx, neuron_idx) -> trace
    /// Sampling interval (ms)
    pub dt: f64,
}

impl VoltageRecorder {
    pub fn new(dt: f64) -> Self {
        Self {
            times: Vec::new(),
            traces: HashMap::new(),
            dt,
        }
    }

    /// Record voltage from a neuron.
    pub fn record(&mut self, pop_idx: usize, neuron_idx: usize, time: f64, voltage: f64) {
        let key = (pop_idx, neuron_idx);
        self.traces.entry(key).or_insert_with(Vec::new).push(voltage);

        // Update time vector (assumes all neurons recorded at same times)
        if self.traces.len() == 1 {
            self.times.push(time);
        }
    }

    /// Clear all recordings.
    pub fn clear(&mut self) {
        self.times.clear();
        self.traces.clear();
    }

    /// Get trace for a specific neuron.
    pub fn get_trace(&self, pop_idx: usize, neuron_idx: usize) -> Option<&[f64]> {
        self.traces.get(&(pop_idx, neuron_idx)).map(|v| v.as_slice())
    }
}

/// Records spike times from neurons.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpikeRecorder {
    /// Spike times for each neuron: (pop_idx, neuron_idx) -> spike_times
    pub spike_trains: HashMap<(usize, usize), Vec<f64>>,
}

impl SpikeRecorder {
    pub fn new() -> Self {
        Self {
            spike_trains: HashMap::new(),
        }
    }

    /// Record a spike.
    pub fn record_spike(&mut self, pop_idx: usize, neuron_idx: usize, time: f64) {
        let key = (pop_idx, neuron_idx);
        self.spike_trains.entry(key).or_insert_with(Vec::new).push(time);
    }

    /// Get spike times for a neuron.
    pub fn get_spikes(&self, pop_idx: usize, neuron_idx: usize) -> Option<&[f64]> {
        self.spike_trains.get(&(pop_idx, neuron_idx)).map(|v| v.as_slice())
    }

    /// Clear all recordings.
    pub fn clear(&mut self) {
        self.spike_trains.clear();
    }

    /// Get total number of spikes.
    pub fn total_spikes(&self) -> usize {
        self.spike_trains.values().map(|v| v.len()).sum()
    }

    /// Generate raster plot data.
    pub fn raster_data(&self) -> Vec<(f64, usize, usize)> {
        let mut data = Vec::new();
        for (&(pop_idx, neuron_idx), spikes) in &self.spike_trains {
            for &time in spikes {
                data.push((time, pop_idx, neuron_idx));
            }
        }
        data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        data
    }
}

impl Default for SpikeRecorder {
    fn default() -> Self {
        Self::new()
    }
}

/// Records population-level firing rates.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PopulationRateRecorder {
    /// Time points
    pub times: Vec<f64>,
    /// Firing rates for each population (Hz)
    pub rates: HashMap<usize, Vec<f64>>, // pop_idx -> rates
    /// Time window for rate calculation (ms)
    pub window: f64,
}

impl PopulationRateRecorder {
    pub fn new(window: f64) -> Self {
        Self {
            times: Vec::new(),
            rates: HashMap::new(),
            window,
        }
    }

    /// Record instantaneous rate.
    pub fn record(&mut self, pop_idx: usize, time: f64, rate: f64) {
        self.rates.entry(pop_idx).or_insert_with(Vec::new).push(rate);

        if self.rates.len() == 1 {
            self.times.push(time);
        }
    }

    /// Clear all recordings.
    pub fn clear(&mut self) {
        self.times.clear();
        self.rates.clear();
    }

    /// Get rate trace for a population.
    pub fn get_rates(&self, pop_idx: usize) -> Option<&[f64]> {
        self.rates.get(&pop_idx).map(|v| v.as_slice())
    }
}

/// Records synaptic currents.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SynapticCurrentRecorder {
    /// Time points
    pub times: Vec<f64>,
    /// Synaptic currents for each neuron
    pub currents: HashMap<(usize, usize), Vec<f64>>, // (pop_idx, neuron_idx) -> currents
}

impl SynapticCurrentRecorder {
    pub fn new() -> Self {
        Self {
            times: Vec::new(),
            currents: HashMap::new(),
        }
    }

    /// Record current.
    pub fn record(&mut self, pop_idx: usize, neuron_idx: usize, time: f64, current: f64) {
        let key = (pop_idx, neuron_idx);
        self.currents.entry(key).or_insert_with(Vec::new).push(current);

        if self.currents.len() == 1 {
            self.times.push(time);
        }
    }

    /// Clear all recordings.
    pub fn clear(&mut self) {
        self.times.clear();
        self.currents.clear();
    }

    /// Get current trace for a neuron.
    pub fn get_currents(&self, pop_idx: usize, neuron_idx: usize) -> Option<&[f64]> {
        self.currents.get(&(pop_idx, neuron_idx)).map(|v| v.as_slice())
    }
}

impl Default for SynapticCurrentRecorder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voltage_recorder() {
        let mut recorder = VoltageRecorder::new(0.1);

        recorder.record(0, 0, 0.0, -65.0);
        recorder.record(0, 0, 0.1, -64.5);
        recorder.record(0, 1, 0.0, -66.0);
        recorder.record(0, 1, 0.1, -65.5);

        assert_eq!(recorder.times.len(), 2);
        assert_eq!(recorder.get_trace(0, 0).unwrap().len(), 2);
        assert_eq!(recorder.get_trace(0, 0).unwrap()[0], -65.0);
    }

    #[test]
    fn test_spike_recorder() {
        let mut recorder = SpikeRecorder::new();

        recorder.record_spike(0, 0, 10.0);
        recorder.record_spike(0, 0, 20.0);
        recorder.record_spike(0, 1, 15.0);

        assert_eq!(recorder.get_spikes(0, 0).unwrap().len(), 2);
        assert_eq!(recorder.get_spikes(0, 1).unwrap().len(), 1);
        assert_eq!(recorder.total_spikes(), 3);
    }

    #[test]
    fn test_raster_data() {
        let mut recorder = SpikeRecorder::new();

        recorder.record_spike(0, 0, 10.0);
        recorder.record_spike(0, 1, 5.0);
        recorder.record_spike(1, 0, 8.0);

        let raster = recorder.raster_data();
        assert_eq!(raster.len(), 3);

        // Should be sorted by time
        assert_eq!(raster[0].0, 5.0);
        assert_eq!(raster[1].0, 8.0);
        assert_eq!(raster[2].0, 10.0);
    }

    #[test]
    fn test_population_rate_recorder() {
        let mut recorder = PopulationRateRecorder::new(10.0);

        recorder.record(0, 0.0, 50.0);
        recorder.record(0, 1.0, 55.0);
        recorder.record(1, 0.0, 30.0);

        assert_eq!(recorder.times.len(), 2);
        assert_eq!(recorder.get_rates(0).unwrap().len(), 2);
        assert_eq!(recorder.get_rates(1).unwrap().len(), 1);
    }

    #[test]
    fn test_synaptic_current_recorder() {
        let mut recorder = SynapticCurrentRecorder::new();

        recorder.record(0, 0, 0.0, 1.5);
        recorder.record(0, 0, 0.1, 2.0);

        assert_eq!(recorder.times.len(), 2);
        assert_eq!(recorder.get_currents(0, 0).unwrap().len(), 2);
        assert_eq!(recorder.get_currents(0, 0).unwrap()[1], 2.0);
    }
}
