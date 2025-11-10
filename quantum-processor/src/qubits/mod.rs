//! Qubit representations for heterogeneous quantum systems
//!
//! Supports multiple physical qubit implementations:
//! - Superconducting qubits (transmon, flux)
//! - Trapped ion qubits
//! - Photonic qubits (dual-rail, polarization)
//! - NV centers in diamond (room temperature)
//! - Topological qubits (Majorana zero modes)

use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Quantum state representation using state vector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumState {
    /// State vector amplitudes (|0⟩ and |1⟩ components)
    pub amplitudes: Vec<Complex64>,
    /// Number of qubits
    pub num_qubits: usize,
}

impl QuantumState {
    /// Create a new quantum state in |0...0⟩
    pub fn new(num_qubits: usize) -> Self {
        let size = 1 << num_qubits;
        let mut amplitudes = vec![Complex64::new(0.0, 0.0); size];
        amplitudes[0] = Complex64::new(1.0, 0.0); // |0...0⟩

        Self {
            amplitudes,
            num_qubits,
        }
    }

    /// Create quantum state from amplitudes
    pub fn from_amplitudes(amplitudes: Vec<Complex64>) -> Result<Self, String> {
        let size = amplitudes.len();
        if size == 0 || (size & (size - 1)) != 0 {
            return Err("Amplitude count must be a power of 2".to_string());
        }

        let num_qubits = (size as f64).log2() as usize;

        // Verify normalization
        let norm: f64 = amplitudes.iter().map(|a| a.norm_sqr()).sum();
        if (norm - 1.0).abs() > 1e-10 {
            return Err(format!("State not normalized: norm = {}", norm));
        }

        Ok(Self {
            amplitudes,
            num_qubits,
        })
    }

    /// Get probability of measuring |0⟩ on qubit i
    pub fn measure_probability_zero(&self, qubit: usize) -> f64 {
        if qubit >= self.num_qubits {
            return 0.0;
        }

        let mut prob = 0.0;
        for (idx, amp) in self.amplitudes.iter().enumerate() {
            // Check if qubit i is 0 in this basis state
            if (idx >> qubit) & 1 == 0 {
                prob += amp.norm_sqr();
            }
        }
        prob
    }

    /// Apply decoherence (amplitude damping)
    pub fn apply_decoherence(&mut self, qubit: usize, gamma: f64) {
        if qubit >= self.num_qubits || gamma <= 0.0 || gamma > 1.0 {
            return;
        }

        let size = self.amplitudes.len();
        let mut new_amplitudes = self.amplitudes.clone();

        for idx in 0..size {
            let bit_val = (idx >> qubit) & 1;

            if bit_val == 1 {
                // |1⟩ state decays to |0⟩
                let idx_flipped = idx ^ (1 << qubit);
                let alpha = self.amplitudes[idx];

                new_amplitudes[idx] = alpha * (1.0 - gamma).sqrt();
                new_amplitudes[idx_flipped] += alpha * gamma.sqrt();
            }
        }

        self.amplitudes = new_amplitudes;
        self.normalize();
    }

    /// Normalize the state vector
    pub fn normalize(&mut self) {
        let norm: f64 = self.amplitudes.iter().map(|a| a.norm_sqr()).sum::<f64>().sqrt();
        if norm > 0.0 {
            for amp in &mut self.amplitudes {
                *amp /= norm;
            }
        }
    }

    /// Calculate fidelity with another state
    pub fn fidelity(&self, other: &QuantumState) -> f64 {
        if self.num_qubits != other.num_qubits {
            return 0.0;
        }

        let overlap: Complex64 = self.amplitudes.iter()
            .zip(&other.amplitudes)
            .map(|(a, b)| a.conj() * b)
            .sum();

        overlap.norm_sqr()
    }
}

impl fmt::Display for QuantumState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "QuantumState({} qubits):", self.num_qubits)?;
        for (idx, amp) in self.amplitudes.iter().enumerate() {
            let prob = amp.norm_sqr();
            if prob > 1e-10 {
                let binary = format!("{:0width$b}", idx, width = self.num_qubits);
                writeln!(f, "  |{}⟩: {:.4} + {:.4}i (p={:.4})",
                    binary, amp.re, amp.im, prob)?;
            }
        }
        Ok(())
    }
}

/// Physical qubit platform type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QubitPlatform {
    /// Superconducting transmon qubits
    Superconducting,
    /// Trapped ion qubits (171Yb+, 40Ca+, etc.)
    TrappedIon,
    /// Photonic qubits (dual-rail encoding)
    Photonic,
    /// NV centers in diamond (room temperature operation)
    NVCenter,
    /// Topological qubits (Majorana zero modes)
    Topological,
}

impl QubitPlatform {
    /// Get typical gate time in nanoseconds
    pub fn gate_time_ns(&self) -> f64 {
        match self {
            QubitPlatform::Superconducting => 20.0,     // 10-100 ns
            QubitPlatform::TrappedIon => 10_000.0,      // ~10 μs
            QubitPlatform::Photonic => 1.0,             // ~1 ns (speed of light)
            QubitPlatform::NVCenter => 1_000.0,         // ~1 μs
            QubitPlatform::Topological => 100.0,        // Estimated ~100 ns
        }
    }

    /// Get typical coherence time in microseconds
    pub fn coherence_time_us(&self) -> f64 {
        match self {
            QubitPlatform::Superconducting => 100.0,    // ~100 μs
            QubitPlatform::TrappedIon => 10_000.0,      // ~10 ms (excellent)
            QubitPlatform::Photonic => f64::INFINITY,   // No decoherence in flight
            QubitPlatform::NVCenter => 100.0,           // ~100 μs at room temp
            QubitPlatform::Topological => 1_000_000.0,  // Topologically protected
        }
    }

    /// Get typical two-qubit gate fidelity
    pub fn two_qubit_fidelity(&self) -> f64 {
        match self {
            QubitPlatform::Superconducting => 0.99,     // 99%
            QubitPlatform::TrappedIon => 0.999,         // 99.9%
            QubitPlatform::Photonic => 0.98,            // 98% (fusion gates)
            QubitPlatform::NVCenter => 0.95,            // 95%
            QubitPlatform::Topological => 0.9999,       // 99.99% (protected)
        }
    }

    /// Operating temperature in Kelvin
    pub fn operating_temperature_k(&self) -> f64 {
        match self {
            QubitPlatform::Superconducting => 0.015,    // ~15 mK
            QubitPlatform::TrappedIon => 0.001,         // ~1 mK (ions themselves)
            QubitPlatform::Photonic => 300.0,           // Room temperature
            QubitPlatform::NVCenter => 300.0,           // Room temperature
            QubitPlatform::Topological => 0.1,          // ~100 mK
        }
    }
}

impl fmt::Display for QubitPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QubitPlatform::Superconducting => write!(f, "Superconducting"),
            QubitPlatform::TrappedIon => write!(f, "TrappedIon"),
            QubitPlatform::Photonic => write!(f, "Photonic"),
            QubitPlatform::NVCenter => write!(f, "NVCenter"),
            QubitPlatform::Topological => write!(f, "Topological"),
        }
    }
}

/// Logical qubit composed of physical qubits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalQubit {
    /// Unique identifier
    pub id: usize,
    /// Physical platform type
    pub platform: QubitPlatform,
    /// Number of physical qubits used
    pub num_physical: usize,
    /// Current quantum state
    pub state: QuantumState,
    /// Accumulated error rate
    pub error_rate: f64,
    /// Total gate operations performed
    pub gate_count: u64,
}

impl LogicalQubit {
    /// Create a new logical qubit
    pub fn new(id: usize, platform: QubitPlatform, num_physical: usize) -> Self {
        Self {
            id,
            platform,
            num_physical,
            state: QuantumState::new(1),
            error_rate: 0.0,
            gate_count: 0,
        }
    }

    /// Increment gate count and update error
    pub fn record_gate(&mut self, gate_fidelity: f64) {
        self.gate_count += 1;
        self.error_rate = 1.0 - (1.0 - self.error_rate) * gate_fidelity;
    }

    /// Check if logical error rate exceeds threshold
    pub fn needs_correction(&self, threshold: f64) -> bool {
        self.error_rate > threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_quantum_state_creation() {
        let state = QuantumState::new(2);
        assert_eq!(state.num_qubits, 2);
        assert_eq!(state.amplitudes.len(), 4);
        assert_relative_eq!(state.amplitudes[0].norm(), 1.0);
    }

    #[test]
    fn test_state_normalization() {
        let amplitudes = vec![
            Complex64::new(0.5, 0.0),
            Complex64::new(0.5, 0.0),
            Complex64::new(0.5, 0.0),
            Complex64::new(0.5, 0.0),
        ];
        let result = QuantumState::from_amplitudes(amplitudes);
        assert!(result.is_ok());

        let state = result.unwrap();
        let norm: f64 = state.amplitudes.iter().map(|a| a.norm_sqr()).sum();
        assert_relative_eq!(norm, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_measurement_probability() {
        let mut amplitudes = vec![Complex64::new(0.0, 0.0); 4];
        amplitudes[0] = Complex64::new(0.6, 0.0); // |00⟩
        amplitudes[3] = Complex64::new(0.8, 0.0); // |11⟩

        let state = QuantumState::from_amplitudes(amplitudes).unwrap();

        // Qubit 0 should be 0 with probability 0.36 (only in |00⟩)
        let prob_zero = state.measure_probability_zero(0);
        assert_relative_eq!(prob_zero, 0.36, epsilon = 1e-10);
    }

    #[test]
    fn test_fidelity() {
        let state1 = QuantumState::new(2);
        let state2 = QuantumState::new(2);

        assert_relative_eq!(state1.fidelity(&state2), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_qubit_platform_properties() {
        let platform = QubitPlatform::Superconducting;
        assert!(platform.gate_time_ns() < 100.0);
        assert!(platform.two_qubit_fidelity() > 0.98);
        assert!(platform.operating_temperature_k() < 1.0);
    }

    #[test]
    fn test_logical_qubit() {
        let mut qubit = LogicalQubit::new(0, QubitPlatform::Superconducting, 17);
        assert_eq!(qubit.error_rate, 0.0);

        qubit.record_gate(0.99);
        assert!(qubit.error_rate > 0.0);
        assert!(qubit.error_rate < 0.02);
    }

    #[test]
    fn test_decoherence() {
        let mut state = QuantumState::new(1);
        // Put in superposition: (|0⟩ + |1⟩) / sqrt(2)
        state.amplitudes[0] = Complex64::new(1.0 / 2.0_f64.sqrt(), 0.0);
        state.amplitudes[1] = Complex64::new(1.0 / 2.0_f64.sqrt(), 0.0);

        state.apply_decoherence(0, 0.1);

        // After decoherence, |1⟩ amplitude should decrease
        assert!(state.amplitudes[1].norm() < 1.0 / 2.0_f64.sqrt());

        // Check still normalized
        let norm: f64 = state.amplitudes.iter().map(|a| a.norm_sqr()).sum();
        assert_relative_eq!(norm, 1.0, epsilon = 1e-10);
    }
}
