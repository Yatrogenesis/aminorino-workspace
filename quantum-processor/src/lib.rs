//! # Quantum Processor Simulator
//!
//! Heterogeneous quantum processor simulator implementing:
//! - Multiple qubit platforms (superconducting, trapped ion, photonic, NV centers, topological)
//! - Advanced error correction (LDPC codes, surface codes)
//! - Quantum reservoir computing with coupled oscillators
//! - Distributed chaos architecture for extreme resilience
//! - Radiation simulation and protection
//! - LISP-based dynamic optimization
//!
//! ## Architecture
//!
//! Three-layer design:
//! - **Layer 1**: Universal computation core (LDPC-corrected or topological qubits)
//! - **Layer 2**: Quantum neural networks (reservoir computing with coupled oscillators)
//! - **Layer 3**: Autonomous monitoring and self-repair (AI-enhanced adaptive correction)
//!
//! ## Example
//!
//! ```rust
//! use quantum_processor::prelude::*;
//!
//! // Create heterogeneous processor
//! let mut processor = QuantumProcessorBuilder::new()
//!     .add_superconducting_qubits(10)
//!     .add_trapped_ion_qubits(5)
//!     .add_photonic_qubits(20)
//!     .with_ldpc_correction(distance: 13)
//!     .with_reservoir_computing(oscillators: 2)
//!     .build();
//!
//! // Execute quantum circuit
//! let circuit = QuantumCircuit::new(processor.num_qubits());
//! let result = processor.execute(&circuit)?;
//! ```

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod qubits;
pub mod gates;
pub mod circuits;
pub mod error_correction;
pub mod platforms;
pub mod architecture;
pub mod monitoring;

#[cfg(feature = "reservoir")]
pub mod quantum_reservoir;

#[cfg(feature = "radiation")]
pub mod radiation;

#[cfg(feature = "lisp-optimization")]
pub mod lisp_interface;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::qubits::*;
    pub use crate::gates::*;
    pub use crate::circuits::*;
    pub use crate::error_correction::*;
    pub use crate::platforms::*;
    pub use crate::architecture::*;
    pub use crate::monitoring::*;

    #[cfg(feature = "reservoir")]
    pub use crate::quantum_reservoir::*;

    #[cfg(feature = "radiation")]
    pub use crate::radiation::*;
}

use thiserror::Error;

/// Errors that can occur during quantum computation
#[derive(Error, Debug)]
pub enum QuantumError {
    /// Decoherence caused state loss
    #[error("Decoherence error: {0}")]
    Decoherence(String),

    /// Gate fidelity below threshold
    #[error("Gate fidelity too low: {fidelity:.6}, threshold: {threshold:.6}")]
    LowFidelity {
        /// Measured fidelity
        fidelity: f64,
        /// Required threshold
        threshold: f64
    },

    /// Error correction failed
    #[error("Error correction failed: {0}")]
    CorrectionFailed(String),

    /// Invalid quantum state
    #[error("Invalid quantum state: {0}")]
    InvalidState(String),

    /// Platform-specific error
    #[error("Platform error ({platform}): {message}")]
    PlatformError {
        /// Platform name
        platform: String,
        /// Error message
        message: String
    },

    /// Radiation event caused uncorrectable error
    #[error("Radiation event: {0}")]
    RadiationEvent(String),

    /// Circuit compilation failed
    #[error("Circuit compilation error: {0}")]
    CompilationError(String),
}

/// Result type for quantum operations
pub type QuantumResult<T> = Result<T, QuantumError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_error_display() {
        let err = QuantumError::Decoherence("Test decoherence".to_string());
        assert!(err.to_string().contains("Decoherence"));

        let err = QuantumError::LowFidelity { fidelity: 0.95, threshold: 0.99 };
        assert!(err.to_string().contains("fidelity"));
    }
}
