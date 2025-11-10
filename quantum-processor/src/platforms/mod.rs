//! Physical platform implementations

use crate::qubits::QubitPlatform;
use serde::{Deserialize, Serialize};

/// Platform-specific parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfig {
    /// Platform type
    pub platform: QubitPlatform,
    /// Number of qubits
    pub num_qubits: usize,
    /// Gate fidelity
    pub gate_fidelity: f64,
    /// Measurement fidelity
    pub measurement_fidelity: f64,
}

impl PlatformConfig {
    /// Create configuration for a platform
    pub fn new(platform: QubitPlatform, num_qubits: usize) -> Self {
        Self {
            platform,
            num_qubits,
            gate_fidelity: platform.two_qubit_fidelity(),
            measurement_fidelity: 0.99,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_config() {
        let config = PlatformConfig::new(QubitPlatform::Superconducting, 10);
        assert_eq!(config.num_qubits, 10);
        assert!(config.gate_fidelity > 0.98);
    }
}
