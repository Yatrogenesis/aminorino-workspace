//! Error types for neural dynamics library.

use thiserror::Error;

/// Result type for neural dynamics operations.
pub type Result<T> = std::result::Result<T, NeuralDynamicsError>;

/// Errors that can occur in neural dynamics simulations.
#[derive(Error, Debug, Clone)]
pub enum NeuralDynamicsError {
    /// Invalid parameter value
    #[error("Invalid parameter {parameter}: {reason} (value: {value})")]
    InvalidParameter {
        parameter: String,
        value: f64,
        reason: String,
    },

    /// Invalid neuron index
    #[error("Invalid neuron index {index} (max: {max})")]
    InvalidNeuronIndex { index: usize, max: usize },

    /// Invalid population index
    #[error("Invalid population index {index} (max: {max})")]
    InvalidPopulationIndex { index: usize, max: usize },

    /// Invalid projection index
    #[error("Invalid projection index {index} (max: {max})")]
    InvalidProjectionIndex { index: usize, max: usize },

    /// Population size mismatch
    #[error("Population size mismatch: expected {expected}, got {actual}")]
    SizeMismatch { expected: usize, actual: usize },

    /// Empty population
    #[error("Population is empty")]
    EmptyPopulation,

    /// Empty network
    #[error("Network is empty (no populations)")]
    EmptyNetwork,

    /// Connectivity error
    #[error("Connectivity error: {reason}")]
    ConnectivityError { reason: String },

    /// Simulation error
    #[error("Simulation error: {reason}")]
    SimulationError { reason: String },

    /// Non-finite value encountered
    #[error("Non-finite value encountered in {location}: {value}")]
    NonFiniteValue { location: String, value: f64 },

    /// Integration error
    #[error("Integration error: {reason}")]
    IntegrationError { reason: String },

    /// Recording error
    #[error("Recording error: {reason}")]
    RecordingError { reason: String },

    /// Analysis error
    #[error("Analysis error: {reason}")]
    AnalysisError { reason: String },

    /// IO error
    #[error("IO error: {0}")]
    IoError(String),

    /// Propagated error from hodgkin-huxley
    #[error("Hodgkin-Huxley error: {0}")]
    HodgkinHuxleyError(String),

    /// Propagated error from synapse-models
    #[error("Synapse error: {0}")]
    SynapseError(String),
}

impl From<hodgkin_huxley::error::HHError> for NeuralDynamicsError {
    fn from(err: hodgkin_huxley::error::HHError) -> Self {
        NeuralDynamicsError::HodgkinHuxleyError(err.to_string())
    }
}

impl From<synapse_models::error::SynapseError> for NeuralDynamicsError {
    fn from(err: synapse_models::error::SynapseError) -> Self {
        NeuralDynamicsError::SynapseError(err.to_string())
    }
}

impl From<std::io::Error> for NeuralDynamicsError {
    fn from(err: std::io::Error) -> Self {
        NeuralDynamicsError::IoError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = NeuralDynamicsError::InvalidParameter {
            parameter: "weight".to_string(),
            value: -1.0,
            reason: "must be non-negative".to_string(),
        };
        assert!(err.to_string().contains("Invalid parameter"));
        assert!(err.to_string().contains("weight"));
    }

    #[test]
    fn test_error_conversions() {
        let hh_err = hodgkin_huxley::error::HHError::InvalidParameter {
            parameter: "c_m".to_string(),
            value: -1.0,
            reason: "negative".to_string(),
        };
        let nd_err: NeuralDynamicsError = hh_err.into();
        assert!(matches!(nd_err, NeuralDynamicsError::HodgkinHuxleyError(_)));
    }
}
