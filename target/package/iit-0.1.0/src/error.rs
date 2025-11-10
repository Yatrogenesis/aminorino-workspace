//! Error types for the IIT library.
//!
//! This module defines all error types that can occur during IIT computations,
//! including matrix operations, partition searches, and Phi calculations.

use thiserror::Error;

/// Result type for IIT operations.
pub type Result<T> = std::result::Result<T, IITError>;

/// Error types for IIT computations.
#[derive(Error, Debug, Clone)]
pub enum IITError {
    /// Invalid transition probability matrix.
    #[error("Invalid TPM: {0}")]
    InvalidTPM(String),

    /// Invalid connectivity matrix.
    #[error("Invalid connectivity matrix: {0}")]
    InvalidConnectivity(String),

    /// Invalid state specification.
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// Invalid mechanism specification.
    #[error("Invalid mechanism: {0}")]
    InvalidMechanism(String),

    /// Invalid purview specification.
    #[error("Invalid purview: {0}")]
    InvalidPurview(String),

    /// Matrix operation failed.
    #[error("Matrix operation failed: {0}")]
    MatrixError(String),

    /// Dimension mismatch in matrix operations.
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    /// Singular matrix encountered.
    #[error("Singular matrix: cannot compute inverse or determinant")]
    SingularMatrix,

    /// Numerical instability detected.
    #[error("Numerical instability: {0}")]
    NumericalInstability(String),

    /// System too large for exact computation.
    #[error("System with {size} elements too large for exact computation (max: {max})")]
    SystemTooLarge { size: usize, max: usize },

    /// Computation timeout.
    #[error("Computation exceeded timeout of {timeout_secs} seconds")]
    Timeout { timeout_secs: u64 },

    /// Invalid partition.
    #[error("Invalid partition: {0}")]
    InvalidPartition(String),

    /// EMD computation failed.
    #[error("EMD computation failed: {0}")]
    EMDError(String),

    /// Cache error.
    #[error("Cache error: {0}")]
    CacheError(String),

    /// Approximation method not available.
    #[error("Approximation method '{method}' not available for this system")]
    ApproximationUnavailable { method: String },

    /// Configuration error.
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// General computation error.
    #[error("Computation error: {0}")]
    ComputationError(String),
}

impl IITError {
    /// Create a new InvalidTPM error.
    pub fn invalid_tpm(msg: impl Into<String>) -> Self {
        Self::InvalidTPM(msg.into())
    }

    /// Create a new InvalidConnectivity error.
    pub fn invalid_connectivity(msg: impl Into<String>) -> Self {
        Self::InvalidConnectivity(msg.into())
    }

    /// Create a new InvalidState error.
    pub fn invalid_state(msg: impl Into<String>) -> Self {
        Self::InvalidState(msg.into())
    }

    /// Create a new MatrixError.
    pub fn matrix_error(msg: impl Into<String>) -> Self {
        Self::MatrixError(msg.into())
    }

    /// Create a new DimensionMismatch error.
    pub fn dimension_mismatch(expected: usize, actual: usize) -> Self {
        Self::DimensionMismatch { expected, actual }
    }

    /// Create a new NumericalInstability error.
    pub fn numerical_instability(msg: impl Into<String>) -> Self {
        Self::NumericalInstability(msg.into())
    }

    /// Create a new SystemTooLarge error.
    pub fn system_too_large(size: usize, max: usize) -> Self {
        Self::SystemTooLarge { size, max }
    }

    /// Create a new ComputationError.
    pub fn computation_error(msg: impl Into<String>) -> Self {
        Self::ComputationError(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = IITError::invalid_tpm("test message");
        assert!(matches!(err, IITError::InvalidTPM(_)));
    }

    #[test]
    fn test_dimension_mismatch() {
        let err = IITError::dimension_mismatch(10, 5);
        assert!(matches!(err, IITError::DimensionMismatch { expected: 10, actual: 5 }));
    }

    #[test]
    fn test_error_display() {
        let err = IITError::SystemTooLarge { size: 30, max: 20 };
        let msg = format!("{}", err);
        assert!(msg.contains("30"));
        assert!(msg.contains("20"));
    }
}
