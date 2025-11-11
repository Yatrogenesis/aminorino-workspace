//! Brain module for different consciousness substrates
//!
//! This module provides three types of brain architectures for testing
//! the hypothesis that different substrates exhibit different levels of
//! integrated information (Φ):
//!
//! 1. **Quantum Brain**: Pure quantum reservoir computing (existing)
//! 2. **Biological Brain**: Hodgkin-Huxley neuronal networks
//! 3. **Hybrid Brain**: Quantum reservoir coupled with HH neurons
//!
//! ## Hypothesis
//!
//! Φ_hybrid > Φ_quantum > Φ_biological
//!
//! The hybrid system is expected to maximize Φ by combining:
//! - Quantum superposition for parallel information processing
//! - Biological temporal integration for causal structure
//! - Optimal information preservation across both substrates

pub mod quantum_brain;
pub mod biological_brain;
pub mod hybrid_brain;

// Re-export main types
pub use quantum_brain::QuantumBrain;
pub use biological_brain::BiologicalBrain;
pub use hybrid_brain::HybridBrain;

/// Common trait for all brain types
pub trait BrainSubstrate {
    /// Get the current state as a vector for Φ measurement
    fn get_state_vector(&self) -> Vec<f64>;

    /// Get the number of units (neurons, oscillators, etc.)
    fn get_num_units(&self) -> usize;

    /// Evolve the brain for a given time step
    fn evolve(&mut self, dt: f64) -> crate::BrainResult<()>;

    /// Set input to the system
    fn set_input(&mut self, input: &[f64]) -> crate::BrainResult<()>;

    /// Get substrate type name
    fn substrate_type(&self) -> &'static str;
}
