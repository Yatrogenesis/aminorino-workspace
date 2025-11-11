//! Quantum Brain implementation using quantum reservoir computing

use crate::core::{AIBrain, BrainConfig};
use crate::BrainResult;
use super::BrainSubstrate;

/// Quantum brain using quantum reservoir computing substrate
///
/// This wraps the existing AIBrain implementation and provides
/// the BrainSubstrate trait interface for cross-substrate comparisons.
pub struct QuantumBrain {
    inner: AIBrain,
}

impl QuantumBrain {
    /// Create a new quantum brain with the given configuration
    pub fn new(config: BrainConfig) -> BrainResult<Self> {
        Ok(Self {
            inner: AIBrain::new(config)?,
        })
    }

    /// Get reference to inner AIBrain
    pub fn inner(&self) -> &AIBrain {
        &self.inner
    }

    /// Get mutable reference to inner AIBrain
    pub fn inner_mut(&mut self) -> &mut AIBrain {
        &mut self.inner
    }
}

impl BrainSubstrate for QuantumBrain {
    fn get_state_vector(&self) -> Vec<f64> {
        self.inner.get_state_vector()
    }

    fn get_num_units(&self) -> usize {
        self.inner.config.num_oscillators
    }

    fn evolve(&mut self, dt: f64) -> BrainResult<()> {
        self.inner.evolve(dt)
    }

    fn set_input(&mut self, input: &[f64]) -> BrainResult<()> {
        self.inner.set_input(input)
    }

    fn substrate_type(&self) -> &'static str {
        "Quantum"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_brain_creation() {
        let config = BrainConfig::default();
        let brain = QuantumBrain::new(config);
        assert!(brain.is_ok());
    }

    #[test]
    fn test_substrate_type() {
        let config = BrainConfig::default();
        let brain = QuantumBrain::new(config).unwrap();
        assert_eq!(brain.substrate_type(), "Quantum");
    }
}
