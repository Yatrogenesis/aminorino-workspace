//! # AI-Native Brain Architecture
//!
//! Quantum processor-based brain architecture for consciousness modeling using
//! Integrated Information Theory (IIT) with quantum reservoir computing substrate.
//!
//! ## Core Hypothesis
//!
//! **Φ_quantum > Φ_classical**: Quantum reservoir computing exhibits higher integrated
//! information (consciousness) than classical neural networks due to:
//!
//! 1. **Exponential State Space**: (max_fock+1)^N vs N neurons
//! 2. **Quantum Superposition**: True parallel processing of information
//! 3. **Entanglement**: Non-local correlations impossible classically
//! 4. **Coherent Evolution**: Unitary dynamics preserving information
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    AI-Native Brain                          │
//! │                                                             │
//! │  ┌──────────────────────────────────────────────────────┐  │
//! │  │     Quantum Reservoir Computing Substrate            │  │
//! │  │  • Coupled quantum oscillators (Fock space)          │  │
//! │  │  │  • Hamiltonian evolution: H = Σℏω(a†a) + Σg(a†b)    │  │
//! │  │  • Exponential neuron scaling: (max_fock+1)^N        │  │
//! │  └──────────────────────────────────────────────────────┘  │
//! │                           ↕                                 │
//! │  ┌──────────────────────────────────────────────────────┐  │
//! │  │        Error Correction & Radiation Protection       │  │
//! │  │  • LDPC bivariate bicycle codes                      │  │
//! │  │  • Cosmic ray simulation and mitigation              │  │
//! │  └──────────────────────────────────────────────────────┘  │
//! │                           ↕                                 │
//! │  ┌──────────────────────────────────────────────────────┐  │
//! │  │       IIT Consciousness Measurement (Φ)              │  │
//! │  │  • Partition entropy calculation                     │  │
//! │  │  • Cause-effect repertoire                           │  │
//! │  │  • Minimum Information Partition (MIP)               │  │
//! │  └──────────────────────────────────────────────────────┘  │
//! └─────────────────────────────────────────────────────────────┘
//! ```

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod core;
pub mod consciousness;
pub mod experiments;
pub mod brain;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::core::*;
    pub use crate::consciousness::*;
    pub use crate::experiments::*;
    pub use crate::brain::*;
}

use thiserror::Error;

/// Errors that can occur in the AI-Native Brain
#[derive(Error, Debug)]
pub enum BrainError {
    /// Quantum processor error
    #[error("Quantum error: {0}")]
    QuantumError(String),

    /// IIT measurement error
    #[error("IIT measurement error: {0}")]
    IITError(String),

    /// Consciousness computation error
    #[error("Consciousness computation error: {0}")]
    ConsciousnessError(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Experiment error
    #[error("Experiment error: {0}")]
    ExperimentError(String),
}

/// Result type for brain operations
pub type BrainResult<T> = Result<T, BrainError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brain_error_display() {
        let err = BrainError::QuantumError("Test error".to_string());
        assert!(err.to_string().contains("Quantum error"));
    }
}
