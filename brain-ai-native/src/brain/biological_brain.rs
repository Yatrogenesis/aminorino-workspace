//! Biological Brain implementation using Hodgkin-Huxley neurons

use crate::BrainResult;
use super::BrainSubstrate;
use hodgkin_huxley::{HodgkinHuxleyNeuron, neuron_types::NeuronConfig};

/// Biological brain using Hodgkin-Huxley neuronal network
///
/// This provides a realistic biological neural substrate for consciousness
/// measurement, allowing comparison with quantum and hybrid architectures.
pub struct BiologicalBrain {
    neurons: Vec<HodgkinHuxleyNeuron>,
    coupling_strength: f64,
    num_neurons: usize,
}

/// Configuration for biological brain
#[derive(Debug, Clone)]
pub struct BiologicalBrainConfig {
    /// Number of neurons in the network
    pub num_neurons: usize,

    /// Synaptic coupling strength (mS/cm²)
    pub coupling_strength: f64,

    /// Neuron type configuration
    pub neuron_config: NeuronConfig,
}

impl Default for BiologicalBrainConfig {
    fn default() -> Self {
        Self {
            num_neurons: 4,
            coupling_strength: 0.1,  // mS/cm²
            neuron_config: NeuronConfig::regular_spiking(),
        }
    }
}

impl BiologicalBrain {
    /// Create a new biological brain with the given configuration
    pub fn new(config: BiologicalBrainConfig) -> BrainResult<Self> {
        // Create neurons
        let mut neurons = Vec::new();
        for _ in 0..config.num_neurons {
            let neuron = HodgkinHuxleyNeuron::new(config.neuron_config.clone())
                .map_err(|e| crate::BrainError::QuantumError(format!("HH neuron creation failed: {:?}", e)))?;
            neurons.push(neuron);
        }

        // Initialize all neurons to resting state
        for neuron in &mut neurons {
            neuron.initialize_rest();
        }

        Ok(Self {
            neurons,
            coupling_strength: config.coupling_strength,
            num_neurons: config.num_neurons,
        })
    }

    /// Get the neurons
    pub fn neurons(&self) -> &[HodgkinHuxleyNeuron] {
        &self.neurons
    }

    /// Evolve network with synaptic coupling
    fn evolve_coupled(&mut self, dt: f64) -> BrainResult<()> {
        // Collect current voltages
        let voltages: Vec<f64> = self.neurons.iter()
            .map(|n| n.voltage())
            .collect();

        // Evolve each neuron with synaptic input from others
        for i in 0..self.num_neurons {
            // Calculate synaptic input from all other neurons
            let mut i_syn = 0.0;
            for (j, v_j) in voltages.iter().enumerate() {
                if i != j {
                    // Simple electrical synapse (gap junction)
                    i_syn += self.coupling_strength * (v_j - voltages[i]);
                }
            }

            // Step neuron with synaptic input
            self.neurons[i].step(dt, i_syn)
                .map_err(|e| crate::BrainError::QuantumError(format!("HH evolution failed: {:?}", e)))?;
        }

        Ok(())
    }
}

impl BrainSubstrate for BiologicalBrain {
    fn get_state_vector(&self) -> Vec<f64> {
        // Get complete state from all neurons: [V, m, h, n] for each neuron
        // This is necessary for proper IIT Φ calculation
        let mut state = Vec::with_capacity(self.num_neurons * 4);
        for neuron in &self.neurons {
            let (m, h, n, _a, _b) = neuron.gates();  // Get gating variables
            state.push(neuron.voltage());  // V
            state.push(m);                  // m (Na activation)
            state.push(h);                  // h (Na inactivation)
            state.push(n);                  // n (K activation)
        }
        state
    }

    fn get_num_units(&self) -> usize {
        self.num_neurons
    }

    fn evolve(&mut self, dt: f64) -> BrainResult<()> {
        self.evolve_coupled(dt)
    }

    fn set_input(&mut self, input: &[f64]) -> BrainResult<()> {
        if input.len() != self.num_neurons {
            return Err(crate::BrainError::InvalidConfig(
                format!("Input size {} doesn't match neuron count {}", input.len(), self.num_neurons)
            ));
        }

        // Apply input as external current to each neuron
        // We'll store it and apply during next evolve step
        // For now, just do a small step with the input
        for (neuron, &i_ext) in self.neurons.iter_mut().zip(input.iter()) {
            neuron.step(1e-5, i_ext)  // Small timestep to apply input
                .map_err(|e| crate::BrainError::QuantumError(format!("HH set_input failed: {:?}", e)))?;
        }

        Ok(())
    }

    fn substrate_type(&self) -> &'static str {
        "Biological"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biological_brain_creation() {
        let config = BiologicalBrainConfig::default();
        let brain = BiologicalBrain::new(config);
        assert!(brain.is_ok());
    }

    #[test]
    fn test_substrate_type() {
        let config = BiologicalBrainConfig::default();
        let brain = BiologicalBrain::new(config).unwrap();
        assert_eq!(brain.substrate_type(), "Biological");
    }

    #[test]
    fn test_state_vector_dimensions() {
        let config = BiologicalBrainConfig {
            num_neurons: 4,
            ..Default::default()
        };
        let brain = BiologicalBrain::new(config).unwrap();
        let state = brain.get_state_vector();
        // 4 neurons × 4 state variables (V, m, h, n) = 16
        assert_eq!(state.len(), 16);
    }
}
