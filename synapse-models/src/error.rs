//! Error types for synapse models library.

use thiserror::Error;

/// Errors that can occur in synaptic operations.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum SynapseError {
    /// Invalid synaptic weight value.
    #[error("Invalid synaptic weight: {0}. Expected value in range [{1}, {2}]")]
    InvalidWeight(f64, f64, f64),

    /// Invalid time constant.
    #[error("Invalid time constant: {0}. Must be positive")]
    InvalidTimeConstant(f64),

    /// Invalid delay value.
    #[error("Invalid delay: {0}. Must be non-negative")]
    InvalidDelay(f64),

    /// Invalid probability value.
    #[error("Invalid probability: {0}. Must be in range [0, 1]")]
    InvalidProbability(f64),

    /// Invalid voltage value.
    #[error("Invalid voltage: {0} mV")]
    InvalidVoltage(f64),

    /// Invalid concentration value.
    #[error("Invalid concentration: {0}. Must be non-negative")]
    InvalidConcentration(f64),

    /// Synapse not found in network.
    #[error("Synapse not found: {0}")]
    SynapseNotFound(usize),

    /// Neuron not found in network.
    #[error("Neuron not found: {0}")]
    NeuronNotFound(usize),

    /// Invalid network configuration.
    #[error("Invalid network configuration: {0}")]
    InvalidNetwork(String),

    /// Numerical integration error.
    #[error("Numerical integration error: {0}")]
    IntegrationError(String),
}

/// Result type for synapse operations.
pub type Result<T> = std::result::Result<T, SynapseError>;
