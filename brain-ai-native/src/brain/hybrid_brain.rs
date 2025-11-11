//! Hybrid Brain implementation combining quantum reservoir and HH neurons

use crate::BrainResult;
use super::{BrainSubstrate, QuantumBrain, BiologicalBrain};
use crate::core::BrainConfig;
use super::biological_brain::BiologicalBrainConfig;

/// Hybrid brain combining quantum reservoir with biological neurons
///
/// This architecture tests the hypothesis that hybrid quantum-biological
/// systems can achieve higher integrated information (Φ) than either
/// substrate alone, by combining:
/// - Quantum superposition for parallel processing
/// - Biological dynamics for temporal integration
///
/// ## Architecture
///
/// ```text
/// Input → [Quantum Reservoir] → Coupling Layer → [HH Neurons] → Output
///         ↑                                                       ↓
///         └─────────────────── Feedback ─────────────────────────┘
/// ```
pub struct HybridBrain {
    quantum: QuantumBrain,
    biological: BiologicalBrain,
    coupling_strength: f64,
    num_units: usize,
}

/// Configuration for hybrid brain
#[derive(Debug, Clone)]
pub struct HybridBrainConfig {
    /// Quantum brain configuration
    pub quantum_config: BrainConfig,

    /// Biological brain configuration
    pub biological_config: BiologicalBrainConfig,

    /// Coupling strength between quantum and biological (0.0-1.0)
    pub coupling_strength: f64,
}

impl Default for HybridBrainConfig {
    fn default() -> Self {
        Self {
            quantum_config: BrainConfig::default(),
            biological_config: BiologicalBrainConfig::default(),
            coupling_strength: 0.5,
        }
    }
}

impl HybridBrain {
    /// Create a new hybrid brain with the given configuration
    pub fn new(config: HybridBrainConfig) -> BrainResult<Self> {
        let quantum = QuantumBrain::new(config.quantum_config)?;
        let biological = BiologicalBrain::new(config.biological_config)?;

        // Total units is sum of quantum oscillators and biological neurons
        let num_units = quantum.get_num_units() + biological.get_num_units();

        Ok(Self {
            quantum,
            biological,
            coupling_strength: config.coupling_strength,
            num_units,
        })
    }

    /// Get reference to quantum brain
    pub fn quantum(&self) -> &QuantumBrain {
        &self.quantum
    }

    /// Get reference to biological brain
    pub fn biological(&self) -> &BiologicalBrain {
        &self.biological
    }

    /// Get coupling strength
    pub fn coupling_strength(&self) -> f64 {
        self.coupling_strength
    }

    /// Couple quantum output to biological input
    fn quantum_to_biological(&mut self) -> BrainResult<()> {
        // Get quantum state (normalized to 0-1 range)
        let quantum_state = self.quantum.get_state_vector();
        let num_bio = self.biological.get_num_units();

        // Map quantum state to biological input
        // Take mean of quantum state components and scale by coupling strength
        let mean_quantum = quantum_state.iter().sum::<f64>() / quantum_state.len() as f64;

        // Create biological input (current injection in μA/cm²)
        let bio_input: Vec<f64> = (0..num_bio)
            .map(|i| {
                let base = quantum_state.get(i % quantum_state.len()).unwrap_or(&mean_quantum);
                self.coupling_strength * base * 10.0  // Scale to biological range
            })
            .collect();

        self.biological.set_input(&bio_input)?;
        Ok(())
    }

    /// Couple biological output to quantum input
    fn biological_to_quantum(&mut self) -> BrainResult<()> {
        // Get biological state (voltages from neurons)
        let bio_state = self.biological.get_state_vector();
        let num_quantum = self.quantum.get_num_units();

        // Extract voltages (every 4th element: V, m, h, n, V, m, h, n, ...)
        let voltages: Vec<f64> = bio_state.iter()
            .step_by(4)
            .copied()
            .collect();

        // Map biological voltages to quantum input
        // Normalize voltages from typical HH range (-70 to +50 mV) to 0-1
        let quantum_input: Vec<f64> = (0..num_quantum)
            .map(|i| {
                let voltage = voltages.get(i % voltages.len()).unwrap_or(&0.0);
                let normalized = (voltage + 70.0) / 120.0;  // Map -70 to +50 → 0 to 1
                self.coupling_strength * normalized
            })
            .collect();

        self.quantum.set_input(&quantum_input)?;
        Ok(())
    }
}

impl BrainSubstrate for HybridBrain {
    fn get_state_vector(&self) -> Vec<f64> {
        // Combine quantum and biological states
        let mut state = self.quantum.get_state_vector();
        state.extend(self.biological.get_state_vector());
        state
    }

    fn get_num_units(&self) -> usize {
        self.num_units
    }

    fn evolve(&mut self, dt: f64) -> BrainResult<()> {
        // Bidirectional coupling: quantum → biological → quantum

        // 1. Evolve quantum
        self.quantum.evolve(dt / 2.0)?;

        // 2. Couple quantum → biological
        self.quantum_to_biological()?;

        // 3. Evolve biological
        self.biological.evolve(dt)?;

        // 4. Couple biological → quantum
        self.biological_to_quantum()?;

        // 5. Evolve quantum again
        self.quantum.evolve(dt / 2.0)?;

        Ok(())
    }

    fn set_input(&mut self, input: &[f64]) -> BrainResult<()> {
        // Split input between quantum and biological
        let num_quantum = self.quantum.get_num_units();
        let num_bio = self.biological.get_num_units();

        if input.len() != num_quantum + num_bio {
            return Err(crate::BrainError::InvalidConfig(
                format!("Input size {} doesn't match total units {}",
                    input.len(), num_quantum + num_bio)
            ));
        }

        // First part to quantum
        self.quantum.set_input(&input[..num_quantum])?;

        // Second part to biological
        self.biological.set_input(&input[num_quantum..])?;

        Ok(())
    }

    fn substrate_type(&self) -> &'static str {
        "Hybrid"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hybrid_brain_creation() {
        let config = HybridBrainConfig::default();
        let brain = HybridBrain::new(config);
        assert!(brain.is_ok());
    }

    #[test]
    fn test_substrate_type() {
        let config = HybridBrainConfig::default();
        let brain = HybridBrain::new(config).unwrap();
        assert_eq!(brain.substrate_type(), "Hybrid");
    }

    #[test]
    fn test_combined_units() {
        let config = HybridBrainConfig {
            quantum_config: BrainConfig {
                num_oscillators: 4,
                ..Default::default()
            },
            biological_config: BiologicalBrainConfig {
                num_neurons: 4,
                ..Default::default()
            },
            ..Default::default()
        };
        let brain = HybridBrain::new(config).unwrap();
        // 4 quantum oscillators + 4 biological neurons = 8 total units
        assert_eq!(brain.get_num_units(), 8);
    }
}
