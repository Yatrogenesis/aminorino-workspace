//! Heterogeneous quantum processor architecture

use crate::error_correction::ErrorCorrectionCode;
use crate::platforms::PlatformConfig;
use crate::qubits::{LogicalQubit, QubitPlatform};
use serde::{Deserialize, Serialize};

/// Quantum processor architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumProcessor {
    /// Logical qubits
    pub qubits: Vec<LogicalQubit>,
    /// Platform configurations
    pub platforms: Vec<PlatformConfig>,
    /// Error correction scheme
    pub error_correction: ErrorCorrectionCode,
}

impl QuantumProcessor {
    /// Create a new processor
    pub fn new(error_correction: ErrorCorrectionCode) -> Self {
        Self {
            qubits: Vec::new(),
            platforms: Vec::new(),
            error_correction,
        }
    }

    /// Add qubits of a specific platform
    pub fn add_platform(&mut self, platform: QubitPlatform, num_qubits: usize) {
        let config = PlatformConfig::new(platform, num_qubits);
        let physical_per_logical = self.error_correction.physical_qubits_per_logical();

        let start_id = self.qubits.len();
        for i in 0..num_qubits {
            let qubit = LogicalQubit::new(start_id + i, platform, physical_per_logical);
            self.qubits.push(qubit);
        }

        self.platforms.push(config);
    }

    /// Total number of logical qubits
    pub fn num_logical_qubits(&self) -> usize {
        self.qubits.len()
    }

    /// Total number of physical qubits
    pub fn num_physical_qubits(&self) -> usize {
        self.qubits.iter().map(|q| q.num_physical).sum()
    }
}

/// Builder for quantum processor
pub struct QuantumProcessorBuilder {
    processor: QuantumProcessor,
}

impl QuantumProcessorBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            processor: QuantumProcessor::new(ErrorCorrectionCode::None),
        }
    }

    /// Set error correction code
    pub fn with_error_correction(mut self, code: ErrorCorrectionCode) -> Self {
        self.processor.error_correction = code;
        self
    }

    /// Add superconducting qubits
    pub fn add_superconducting_qubits(mut self, num: usize) -> Self {
        self.processor
            .add_platform(QubitPlatform::Superconducting, num);
        self
    }

    /// Add trapped ion qubits
    pub fn add_trapped_ion_qubits(mut self, num: usize) -> Self {
        self.processor.add_platform(QubitPlatform::TrappedIon, num);
        self
    }

    /// Add photonic qubits
    pub fn add_photonic_qubits(mut self, num: usize) -> Self {
        self.processor.add_platform(QubitPlatform::Photonic, num);
        self
    }

    /// Add NV center qubits
    pub fn add_nv_qubits(mut self, num: usize) -> Self {
        self.processor.add_platform(QubitPlatform::NVCenter, num);
        self
    }

    /// Add topological qubits
    pub fn add_topological_qubits(mut self, num: usize) -> Self {
        self.processor
            .add_platform(QubitPlatform::Topological, num);
        self
    }

    /// Build the processor
    pub fn build(self) -> QuantumProcessor {
        self.processor
    }
}

impl Default for QuantumProcessorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_builder() {
        let processor = QuantumProcessorBuilder::new()
            .add_superconducting_qubits(10)
            .add_photonic_qubits(20)
            .build();

        assert_eq!(processor.num_logical_qubits(), 30);
    }

    #[test]
    fn test_with_error_correction() {
        let processor = QuantumProcessorBuilder::new()
            .with_error_correction(ErrorCorrectionCode::Surface { distance: 5 })
            .add_superconducting_qubits(1)
            .build();

        assert_eq!(processor.num_logical_qubits(), 1);
        assert_eq!(processor.num_physical_qubits(), 50);
    }
}
