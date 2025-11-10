//! Error types for the TDA library.

use thiserror::Error;

/// Result type for TDA operations.
pub type Result<T> = std::result::Result<T, TdaError>;

/// Error types that can occur during TDA computations.
#[derive(Error, Debug)]
pub enum TdaError {
    #[error("Invalid dimension: {0}")]
    InvalidDimension(String),

    #[error("Invalid simplex: {0}")]
    InvalidSimplex(String),

    #[error("Empty dataset provided")]
    EmptyDataset,

    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Computation failed: {0}")]
    ComputationError(String),

    #[error("Matrix operation failed: {0}")]
    MatrixError(String),

    #[error("Invalid filtration value: {0}")]
    InvalidFiltration(String),

    #[error("Graph error: {0}")]
    GraphError(String),

    #[error("Invalid spike train: {0}")]
    InvalidSpikeTrain(String),

    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(String),

    #[error("Numerical error: {0}")]
    NumericalError(String),
}
