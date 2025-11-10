//! Neural network architecture and simulation control.

use crate::connectivity::ConnectionPattern;
use crate::error::{NeuralDynamicsError, Result};
use crate::population::NeuralPopulation;
use crate::projection::{DelayInit, Projection, WeightInit};
use crate::recording::{PopulationRateRecorder, SpikeRecorder, VoltageRecorder};
use crate::stimulation::Stimulation;
use serde::{Deserialize, Serialize};
use synapse_models::synapse::Synapse;

/// Neural network with multiple populations and projections.
pub struct Network {
    /// Neural populations
    populations: Vec<NeuralPopulation>,
    /// Inter-population projections
    projections: Vec<Projection>,
    /// Current simulation time (ms)
    current_time: f64,
    /// Time step (ms)
    dt: f64,
    /// External stimulation protocols
    stimulations: Vec<(usize, Box<dyn Stimulation>)>, // (pop_idx, stimulation)
    /// Recorders
    pub spike_recorder: Option<SpikeRecorder>,
    pub voltage_recorder: Option<VoltageRecorder>,
    pub rate_recorder: Option<PopulationRateRecorder>,
}

impl Network {
    /// Create a new empty network.
    pub fn new(dt: f64) -> Result<Self> {
        if dt <= 0.0 {
            return Err(NeuralDynamicsError::InvalidParameter {
                parameter: "dt".to_string(),
                value: dt,
                reason: "must be positive".to_string(),
            });
        }

        Ok(Self {
            populations: Vec::new(),
            projections: Vec::new(),
            current_time: 0.0,
            dt,
            stimulations: Vec::new(),
            spike_recorder: None,
            voltage_recorder: None,
            rate_recorder: None,
        })
    }

    /// Add a population to the network.
    pub fn add_population(&mut self, population: NeuralPopulation) -> usize {
        let idx = self.populations.len();
        self.populations.push(population);
        idx
    }

    /// Add a projection between populations.
    pub fn add_projection(&mut self, projection: Projection) -> usize {
        let idx = self.projections.len();
        self.projections.push(projection);
        idx
    }

    /// Add external stimulation to a population.
    pub fn add_stimulation(&mut self, pop_idx: usize, stim: Box<dyn Stimulation>) -> Result<()> {
        if pop_idx >= self.populations.len() {
            return Err(NeuralDynamicsError::InvalidPopulationIndex {
                index: pop_idx,
                max: self.populations.len() - 1,
            });
        }
        self.stimulations.push((pop_idx, stim));
        Ok(())
    }

    /// Enable spike recording.
    pub fn enable_spike_recording(&mut self) {
        self.spike_recorder = Some(SpikeRecorder::new());
    }

    /// Enable voltage recording.
    pub fn enable_voltage_recording(&mut self) {
        self.voltage_recorder = Some(VoltageRecorder::new(self.dt));
    }

    /// Enable population rate recording.
    pub fn enable_rate_recording(&mut self, window: f64) {
        self.rate_recorder = Some(PopulationRateRecorder::new(window));
    }

    /// Get a population by index.
    pub fn get_population(&self, idx: usize) -> Result<&NeuralPopulation> {
        self.populations.get(idx).ok_or(NeuralDynamicsError::InvalidPopulationIndex {
            index: idx,
            max: self.populations.len().saturating_sub(1),
        })
    }

    /// Get a mutable population by index.
    pub fn get_population_mut(&mut self, idx: usize) -> Result<&mut NeuralPopulation> {
        let max = self.populations.len().saturating_sub(1);
        self.populations.get_mut(idx).ok_or(NeuralDynamicsError::InvalidPopulationIndex {
            index: idx,
            max,
        })
    }

    /// Get current simulation time.
    pub fn current_time(&self) -> f64 {
        self.current_time
    }

    /// Get number of populations.
    pub fn num_populations(&self) -> usize {
        self.populations.len()
    }

    /// Get number of projections.
    pub fn num_projections(&self) -> usize {
        self.projections.len()
    }

    /// Step the simulation forward by one time step.
    pub fn step(&mut self) -> Result<()> {
        if self.populations.is_empty() {
            return Err(NeuralDynamicsError::EmptyNetwork);
        }

        // 1. Reset synaptic currents
        for pop in self.populations.iter_mut() {
            pop.reset_synaptic_currents();
        }

        // 2. Apply external stimulations
        for (pop_idx, stim) in self.stimulations.iter_mut() {
            let pop_size = self.populations[*pop_idx].size;
            for neuron_idx in 0..pop_size {
                let current = stim.current(neuron_idx, self.current_time, self.dt);
                if current != 0.0 {
                    self.populations[*pop_idx].add_external_current(neuron_idx, current)?;
                }
            }
        }

        // 3. Process spikes through projections and accumulate synaptic currents
        for proj in self.projections.iter_mut() {
            let target_pop = &self.populations[proj.target_pop];
            let target_voltages = target_pop.get_voltages();

            let synaptic_currents = proj.process_spikes(self.current_time, &target_voltages, self.dt)?;

            // Apply synaptic currents to target population
            for (target_idx, current) in synaptic_currents {
                self.populations[proj.target_pop].add_external_current(target_idx, current)?;
            }
        }

        // 4. Update all populations (parallel if beneficial)
        if self.populations.len() > 1 {
            // Parallel update - more complex due to borrow checker
            // For now, use sequential update
            for pop in self.populations.iter_mut() {
                pop.update(self.dt, self.current_time)?;
            }
        } else {
            for pop in self.populations.iter_mut() {
                pop.update(self.dt, self.current_time)?;
            }
        }

        // 5. Register spikes with projections
        for (pop_idx, pop) in self.populations.iter().enumerate() {
            for neuron_idx in 0..pop.size {
                let spike_times = pop.get_spike_times(neuron_idx)?;
                if let Some(&last_spike) = spike_times.last() {
                    // Check if spike occurred in this time step
                    if last_spike >= self.current_time - self.dt && last_spike < self.current_time {
                        // Register spike with all relevant projections
                        for proj in self.projections.iter_mut() {
                            if proj.source_pop == pop_idx {
                                proj.register_spike(neuron_idx, last_spike);
                            }
                        }

                        // Record spike if recording enabled
                        if let Some(ref mut recorder) = self.spike_recorder {
                            recorder.record_spike(pop_idx, neuron_idx, last_spike);
                        }
                    }
                }
            }
        }

        // 6. Record voltages if enabled
        if let Some(ref mut recorder) = self.voltage_recorder {
            for (pop_idx, pop) in self.populations.iter().enumerate() {
                for neuron_idx in 0..pop.size {
                    let voltage = pop.get_voltage(neuron_idx)?;
                    recorder.record(pop_idx, neuron_idx, self.current_time, voltage);
                }
            }
        }

        // 7. Record population rates if enabled
        if let Some(ref mut recorder) = self.rate_recorder {
            for (pop_idx, pop) in self.populations.iter().enumerate() {
                let rate = pop.instantaneous_rate(self.current_time, recorder.window);
                recorder.record(pop_idx, self.current_time, rate);
            }
        }

        // Advance time
        self.current_time += self.dt;

        Ok(())
    }

    /// Run simulation for a specified duration.
    pub fn run(&mut self, duration: f64) -> Result<()> {
        let n_steps = (duration / self.dt).ceil() as usize;

        for _ in 0..n_steps {
            self.step()?;
        }

        Ok(())
    }

    /// Reset the network to initial state.
    pub fn reset(&mut self) {
        self.current_time = 0.0;

        for pop in self.populations.iter_mut() {
            pop.reset();
        }

        for (_pop_idx, stim) in self.stimulations.iter_mut() {
            stim.reset();
        }

        if let Some(ref mut recorder) = self.spike_recorder {
            recorder.clear();
        }
        if let Some(ref mut recorder) = self.voltage_recorder {
            recorder.clear();
        }
        if let Some(ref mut recorder) = self.rate_recorder {
            recorder.clear();
        }
    }

    /// Get network statistics.
    pub fn statistics(&self) -> NetworkStats {
        let total_neurons: usize = self.populations.iter().map(|p| p.size).sum();
        let total_connections: usize = self.projections.iter().map(|p| p.num_connections()).sum();

        let total_spikes = if let Some(ref recorder) = self.spike_recorder {
            recorder.total_spikes()
        } else {
            0
        };

        NetworkStats {
            n_populations: self.populations.len(),
            n_projections: self.projections.len(),
            total_neurons,
            total_connections,
            total_spikes,
            current_time: self.current_time,
        }
    }
}

/// Network statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub n_populations: usize,
    pub n_projections: usize,
    pub total_neurons: usize,
    pub total_connections: usize,
    pub total_spikes: usize,
    pub current_time: f64,
}

/// Builder pattern for constructing networks.
pub struct NetworkBuilder {
    network: Network,
    rng: rand::rngs::ThreadRng,
}

impl NetworkBuilder {
    /// Create a new network builder.
    pub fn new(dt: f64) -> Result<Self> {
        Ok(Self {
            network: Network::new(dt)?,
            rng: rand::thread_rng(),
        })
    }

    /// Add an excitatory population.
    pub fn add_excitatory_population(
        mut self,
        id: impl Into<String>,
        size: usize,
    ) -> Result<Self> {
        let pop = NeuralPopulation::excitatory(id, size)?;
        self.network.add_population(pop);
        Ok(self)
    }

    /// Add an inhibitory population.
    pub fn add_inhibitory_population(
        mut self,
        id: impl Into<String>,
        size: usize,
    ) -> Result<Self> {
        let pop = NeuralPopulation::inhibitory(id, size)?;
        self.network.add_population(pop);
        Ok(self)
    }

    /// Add a custom population.
    pub fn add_population(mut self, population: NeuralPopulation) -> Self {
        self.network.add_population(population);
        self
    }

    /// Connect two populations.
    pub fn connect(
        mut self,
        source_idx: usize,
        target_idx: usize,
        pattern: ConnectionPattern,
        synapse_type: SynapseType,
        weight: f64,
        delay: f64,
    ) -> Result<Self> {
        let source_size = self.network.get_population(source_idx)?.size;
        let target_size = self.network.get_population(target_idx)?.size;

        let synapse = match synapse_type {
            SynapseType::Excitatory => Synapse::excitatory(weight, delay)?,
            SynapseType::Inhibitory => Synapse::inhibitory(weight, delay)?,
        };

        let projection = Projection::new(
            source_idx,
            target_idx,
            source_size,
            target_size,
            &pattern,
            &synapse,
            WeightInit::Constant(weight),
            DelayInit::Constant(delay),
            &mut self.rng,
        )?;

        self.network.add_projection(projection);
        Ok(self)
    }

    /// Add stimulation to a population.
    pub fn add_stimulation(
        mut self,
        pop_idx: usize,
        stim: Box<dyn Stimulation>,
    ) -> Result<Self> {
        self.network.add_stimulation(pop_idx, stim)?;
        Ok(self)
    }

    /// Enable spike recording.
    pub fn with_spike_recording(mut self) -> Self {
        self.network.enable_spike_recording();
        self
    }

    /// Enable voltage recording.
    pub fn with_voltage_recording(mut self) -> Self {
        self.network.enable_voltage_recording();
        self
    }

    /// Enable rate recording.
    pub fn with_rate_recording(mut self, window: f64) -> Self {
        self.network.enable_rate_recording(window);
        self
    }

    /// Build the network.
    pub fn build(self) -> Network {
        self.network
    }
}

/// Synapse type for builder.
#[derive(Debug, Clone, Copy)]
pub enum SynapseType {
    Excitatory,
    Inhibitory,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stimulation::CurrentInjection;

    #[test]
    fn test_network_creation() {
        let network = Network::new(0.1).unwrap();
        assert_eq!(network.num_populations(), 0);
        assert_eq!(network.current_time(), 0.0);
    }

    #[test]
    fn test_add_population() {
        let mut network = Network::new(0.1).unwrap();
        let pop = NeuralPopulation::excitatory("E", 10).unwrap();
        let idx = network.add_population(pop);

        assert_eq!(idx, 0);
        assert_eq!(network.num_populations(), 1);
    }

    #[test]
    fn test_network_step() {
        let mut network = Network::new(0.1).unwrap();
        let pop = NeuralPopulation::excitatory("E", 5).unwrap();
        network.add_population(pop);

        let time_before = network.current_time();
        network.step().unwrap();
        let time_after = network.current_time();

        assert_eq!(time_after, time_before + 0.1);
    }

    #[test]
    fn test_network_run() {
        let mut network = Network::new(0.1).unwrap();
        let pop = NeuralPopulation::excitatory("E", 5).unwrap();
        network.add_population(pop);

        network.run(10.0).unwrap();
        assert!((network.current_time() - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_network_with_projection() {
        let mut rng = rand::thread_rng();
        let mut network = Network::new(0.1).unwrap();

        let pop1 = NeuralPopulation::excitatory("E", 3).unwrap();
        let pop2 = NeuralPopulation::excitatory("E2", 2).unwrap();

        let idx1 = network.add_population(pop1);
        let idx2 = network.add_population(pop2);

        let synapse = Synapse::excitatory(1.0, 0.5).unwrap();
        let proj = Projection::all_to_all(idx1, idx2, 3, 2, &synapse, 1.0, 0.5, &mut rng).unwrap();

        network.add_projection(proj);

        assert_eq!(network.num_projections(), 1);

        // Should run without error
        network.run(5.0).unwrap();
    }

    #[test]
    fn test_network_with_stimulation() {
        let mut network = Network::new(0.1).unwrap();
        let pop = NeuralPopulation::excitatory("E", 5).unwrap();
        let idx = network.add_population(pop);

        let stim = CurrentInjection::new(10.0, 0.0, 10.0);
        network.add_stimulation(idx, Box::new(stim)).unwrap();

        network.run(15.0).unwrap();

        // Neurons should have spiked with strong stimulation
        let pop = network.get_population(idx).unwrap();
        let has_spikes = (0..pop.size).any(|i| !pop.get_spike_times(i).unwrap().is_empty());
        assert!(has_spikes);
    }

    #[test]
    fn test_network_reset() {
        let mut network = Network::new(0.1).unwrap();
        let pop = NeuralPopulation::excitatory("E", 5).unwrap();
        network.add_population(pop);

        network.run(10.0).unwrap();
        assert!(network.current_time() > 0.0);

        network.reset();
        assert_eq!(network.current_time(), 0.0);
    }

    #[test]
    fn test_network_builder() {
        let network = NetworkBuilder::new(0.1)
            .unwrap()
            .add_excitatory_population("E", 10)
            .unwrap()
            .add_inhibitory_population("I", 5)
            .unwrap()
            .with_spike_recording()
            .build();

        assert_eq!(network.num_populations(), 2);
        assert!(network.spike_recorder.is_some());
    }

    #[test]
    fn test_network_builder_with_connections() {
        let network = NetworkBuilder::new(0.1)
            .unwrap()
            .add_excitatory_population("E", 10)
            .unwrap()
            .add_inhibitory_population("I", 5)
            .unwrap()
            .connect(
                0,
                1,
                ConnectionPattern::FixedProbability(0.5),
                SynapseType::Excitatory,
                1.0,
                1.0,
            )
            .unwrap()
            .build();

        assert_eq!(network.num_projections(), 1);
    }

    #[test]
    fn test_spike_recording() {
        let mut network = Network::new(0.1).unwrap();
        network.enable_spike_recording();

        let pop = NeuralPopulation::excitatory("E", 3).unwrap();
        let idx = network.add_population(pop);

        let stim = CurrentInjection::new(15.0, 0.0, 50.0);
        network.add_stimulation(idx, Box::new(stim)).unwrap();

        network.run(50.0).unwrap();

        let recorder = network.spike_recorder.as_ref().unwrap();
        assert!(recorder.total_spikes() > 0);
    }

    #[test]
    fn test_network_statistics() {
        let mut network = Network::new(0.1).unwrap();
        let pop = NeuralPopulation::excitatory("E", 10).unwrap();
        network.add_population(pop);

        network.run(5.0).unwrap();

        let stats = network.statistics();
        assert_eq!(stats.n_populations, 1);
        assert_eq!(stats.total_neurons, 10);
    }
}
