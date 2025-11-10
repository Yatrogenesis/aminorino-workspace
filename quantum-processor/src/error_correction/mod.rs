//! Quantum error correction codes
//!
//! Implements:
//! - LDPC (Low-Density Parity-Check) codes with sum-product decoding
//! - Surface codes
//! - Distributed error correction

pub mod ldpc;

use serde::{Deserialize, Serialize};
pub use ldpc::LDPCCode;

/// Error correction code type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorCorrectionCode {
    /// Surface code with given distance
    Surface {
        /// Code distance
        distance: usize
    },
    /// LDPC code
    LDPC {
        /// LDPC code parameters
        code: LDPCCode
    },
    /// No error correction
    None,
}

impl ErrorCorrectionCode {
    /// Get number of physical qubits needed for one logical qubit
    pub fn physical_qubits_per_logical(&self) -> usize {
        match self {
            ErrorCorrectionCode::Surface { distance } => 2 * distance * distance,
            ErrorCorrectionCode::LDPC { code } => code.n / code.k.max(1),
            ErrorCorrectionCode::None => 1,
        }
    }

    /// Code distance
    pub fn distance(&self) -> usize {
        match self {
            ErrorCorrectionCode::Surface { distance } => *distance,
            ErrorCorrectionCode::LDPC { code } => code.distance,
            ErrorCorrectionCode::None => 1,
        }
    }

    /// Create a new LDPC code with bivariate bicycle construction
    pub fn new_ldpc_bicycle(l: usize) -> Self {
        let a = vec![0, 1];
        let b = vec![0, 2];
        let code = LDPCCode::bivariate_bicycle(l, a, b);
        ErrorCorrectionCode::LDPC { code }
    }

    /// Create a regular LDPC code
    pub fn new_ldpc_regular(n: usize, k: usize, row_weight: usize, col_weight: usize) -> Self {
        let code = LDPCCode::regular(n, k, row_weight, col_weight);
        ErrorCorrectionCode::LDPC { code }
    }
}

/// Error syndrome
#[derive(Debug, Clone)]
pub struct Syndrome {
    /// Syndrome bits
    pub bits: Vec<bool>,
}

impl Syndrome {
    /// Check if syndrome indicates no errors
    pub fn is_zero(&self) -> bool {
        self.bits.iter().all(|&b| !b)
    }

    /// Hamming weight (number of violated checks)
    pub fn weight(&self) -> usize {
        self.bits.iter().filter(|&&b| b).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical_qubits_surface_code() {
        let code = ErrorCorrectionCode::Surface { distance: 5 };
        assert_eq!(code.physical_qubits_per_logical(), 50);
        assert_eq!(code.distance(), 5);
    }

    #[test]
    fn test_ldpc_bicycle_creation() {
        let code = ErrorCorrectionCode::new_ldpc_bicycle(3);
        assert!(code.distance() >= 3);
    }

    #[test]
    fn test_syndrome() {
        let syndrome = Syndrome {
            bits: vec![false, false, false],
        };
        assert!(syndrome.is_zero());
        assert_eq!(syndrome.weight(), 0);

        let syndrome_with_errors = Syndrome {
            bits: vec![true, false, true],
        };
        assert!(!syndrome_with_errors.is_zero());
        assert_eq!(syndrome_with_errors.weight(), 2);
    }
}
