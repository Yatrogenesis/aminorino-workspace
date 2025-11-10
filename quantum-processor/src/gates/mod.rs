//! Quantum gates for universal quantum computation
//!
//! Implements single-qubit and multi-qubit gates with realistic noise models

use crate::qubits::{QuantumState, QubitPlatform};
use crate::{QuantumError, QuantumResult};
use num_complex::Complex64;
use serde::{Deserialize, Serialize};
use std::f64::consts::{FRAC_1_SQRT_2, PI};

/// Single-qubit Pauli X gate (NOT gate)
pub fn pauli_x(state: &mut QuantumState, qubit: usize) -> QuantumResult<()> {
    if qubit >= state.num_qubits {
        return Err(QuantumError::InvalidState(format!(
            "Qubit index {} out of range", qubit
        )));
    }

    let size = state.amplitudes.len();
    let mask = 1 << qubit;

    for idx in 0..size {
        if (idx & mask) == 0 {
            let idx_flipped = idx | mask;
            state.amplitudes.swap(idx, idx_flipped);
        }
    }

    Ok(())
}

/// Single-qubit Pauli Y gate
pub fn pauli_y(state: &mut QuantumState, qubit: usize) -> QuantumResult<()> {
    if qubit >= state.num_qubits {
        return Err(QuantumError::InvalidState(format!(
            "Qubit index {} out of range", qubit
        )));
    }

    let size = state.amplitudes.len();
    let mask = 1 << qubit;
    let mut new_amps = state.amplitudes.clone();

    for idx in 0..size {
        let bit_val = (idx >> qubit) & 1;
        let idx_flipped = idx ^ mask;

        if bit_val == 0 {
            // |0⟩ -> i|1⟩
            new_amps[idx_flipped] = Complex64::new(0.0, 1.0) * state.amplitudes[idx];
        } else {
            // |1⟩ -> -i|0⟩
            new_amps[idx_flipped] = Complex64::new(0.0, -1.0) * state.amplitudes[idx];
        }
    }

    state.amplitudes = new_amps;
    Ok(())
}

/// Single-qubit Pauli Z gate
pub fn pauli_z(state: &mut QuantumState, qubit: usize) -> QuantumResult<()> {
    if qubit >= state.num_qubits {
        return Err(QuantumError::InvalidState(format!(
            "Qubit index {} out of range", qubit
        )));
    }

    let size = state.amplitudes.len();
    let mask = 1 << qubit;

    for idx in 0..size {
        if (idx & mask) != 0 {
            state.amplitudes[idx] *= -1.0;
        }
    }

    Ok(())
}

/// Hadamard gate (creates superposition)
pub fn hadamard(state: &mut QuantumState, qubit: usize) -> QuantumResult<()> {
    if qubit >= state.num_qubits {
        return Err(QuantumError::InvalidState(format!(
            "Qubit index {} out of range", qubit
        )));
    }

    let size = state.amplitudes.len();
    let mask = 1 << qubit;
    let sqrt2_inv = FRAC_1_SQRT_2;

    for idx in 0..size {
        if (idx & mask) == 0 {
            let idx_flipped = idx | mask;
            let amp0 = state.amplitudes[idx];
            let amp1 = state.amplitudes[idx_flipped];

            state.amplitudes[idx] = (amp0 + amp1) * sqrt2_inv;
            state.amplitudes[idx_flipped] = (amp0 - amp1) * sqrt2_inv;
        }
    }

    Ok(())
}

/// Phase gate (S gate)
pub fn phase(state: &mut QuantumState, qubit: usize) -> QuantumResult<()> {
    if qubit >= state.num_qubits {
        return Err(QuantumError::InvalidState(format!(
            "Qubit index {} out of range", qubit
        )));
    }

    let size = state.amplitudes.len();
    let mask = 1 << qubit;

    for idx in 0..size {
        if (idx & mask) != 0 {
            state.amplitudes[idx] *= Complex64::new(0.0, 1.0); // multiply by i
        }
    }

    Ok(())
}

/// T gate (π/8 gate)
pub fn t_gate(state: &mut QuantumState, qubit: usize) -> QuantumResult<()> {
    if qubit >= state.num_qubits {
        return Err(QuantumError::InvalidState(format!(
            "Qubit index {} out of range", qubit
        )));
    }

    let size = state.amplitudes.len();
    let mask = 1 << qubit;
    let phase = Complex64::from_polar(1.0, PI / 4.0);

    for idx in 0..size {
        if (idx & mask) != 0 {
            state.amplitudes[idx] *= phase;
        }
    }

    Ok(())
}

/// Rotation around X axis
pub fn rx(state: &mut QuantumState, qubit: usize, theta: f64) -> QuantumResult<()> {
    if qubit >= state.num_qubits {
        return Err(QuantumError::InvalidState(format!(
            "Qubit index {} out of range", qubit
        )));
    }

    let size = state.amplitudes.len();
    let mask = 1 << qubit;
    let cos_half = (theta / 2.0).cos();
    let sin_half = (theta / 2.0).sin();

    for idx in 0..size {
        if (idx & mask) == 0 {
            let idx_flipped = idx | mask;
            let amp0 = state.amplitudes[idx];
            let amp1 = state.amplitudes[idx_flipped];

            state.amplitudes[idx] = amp0 * cos_half + amp1 * Complex64::new(0.0, -sin_half);
            state.amplitudes[idx_flipped] = amp1 * cos_half + amp0 * Complex64::new(0.0, -sin_half);
        }
    }

    Ok(())
}

/// Rotation around Y axis
pub fn ry(state: &mut QuantumState, qubit: usize, theta: f64) -> QuantumResult<()> {
    if qubit >= state.num_qubits {
        return Err(QuantumError::InvalidState(format!(
            "Qubit index {} out of range", qubit
        )));
    }

    let size = state.amplitudes.len();
    let mask = 1 << qubit;
    let cos_half = (theta / 2.0).cos();
    let sin_half = (theta / 2.0).sin();

    for idx in 0..size {
        if (idx & mask) == 0 {
            let idx_flipped = idx | mask;
            let amp0 = state.amplitudes[idx];
            let amp1 = state.amplitudes[idx_flipped];

            state.amplitudes[idx] = amp0 * cos_half - amp1 * sin_half;
            state.amplitudes[idx_flipped] = amp1 * cos_half + amp0 * sin_half;
        }
    }

    Ok(())
}

/// Rotation around Z axis
pub fn rz(state: &mut QuantumState, qubit: usize, theta: f64) -> QuantumResult<()> {
    if qubit >= state.num_qubits {
        return Err(QuantumError::InvalidState(format!(
            "Qubit index {} out of range", qubit
        )));
    }

    let size = state.amplitudes.len();
    let mask = 1 << qubit;
    let phase_pos = Complex64::from_polar(1.0, theta / 2.0);
    let phase_neg = Complex64::from_polar(1.0, -theta / 2.0);

    for idx in 0..size {
        if (idx & mask) == 0 {
            state.amplitudes[idx] *= phase_neg;
        } else {
            state.amplitudes[idx] *= phase_pos;
        }
    }

    Ok(())
}

/// Controlled-NOT (CNOT) gate
pub fn cnot(state: &mut QuantumState, control: usize, target: usize) -> QuantumResult<()> {
    if control >= state.num_qubits || target >= state.num_qubits {
        return Err(QuantumError::InvalidState(
            "Qubit indices out of range".to_string()
        ));
    }
    if control == target {
        return Err(QuantumError::InvalidState(
            "Control and target must be different".to_string()
        ));
    }

    let size = state.amplitudes.len();
    let control_mask = 1 << control;
    let target_mask = 1 << target;

    for idx in 0..size {
        // Only swap if control bit is 1
        if (idx & control_mask) != 0 && (idx & target_mask) == 0 {
            let idx_flipped = idx | target_mask;
            state.amplitudes.swap(idx, idx_flipped);
        }
    }

    Ok(())
}

/// Controlled-Z (CZ) gate
pub fn cz(state: &mut QuantumState, control: usize, target: usize) -> QuantumResult<()> {
    if control >= state.num_qubits || target >= state.num_qubits {
        return Err(QuantumError::InvalidState(
            "Qubit indices out of range".to_string()
        ));
    }

    let size = state.amplitudes.len();
    let control_mask = 1 << control;
    let target_mask = 1 << target;

    for idx in 0..size {
        // Apply phase if both qubits are 1
        if (idx & control_mask) != 0 && (idx & target_mask) != 0 {
            state.amplitudes[idx] *= -1.0;
        }
    }

    Ok(())
}

/// SWAP gate
pub fn swap(state: &mut QuantumState, qubit1: usize, qubit2: usize) -> QuantumResult<()> {
    if qubit1 >= state.num_qubits || qubit2 >= state.num_qubits {
        return Err(QuantumError::InvalidState(
            "Qubit indices out of range".to_string()
        ));
    }
    if qubit1 == qubit2 {
        return Ok(()); // No-op
    }

    let size = state.amplitudes.len();
    let mask1 = 1 << qubit1;
    let mask2 = 1 << qubit2;

    for idx in 0..size {
        let bit1 = (idx & mask1) != 0;
        let bit2 = (idx & mask2) != 0;

        if bit1 != bit2 {
            let idx_swapped = idx ^ mask1 ^ mask2;
            if idx < idx_swapped {
                state.amplitudes.swap(idx, idx_swapped);
            }
        }
    }

    Ok(())
}

/// Toffoli gate (CCNOT - controlled-controlled-NOT)
pub fn toffoli(state: &mut QuantumState, control1: usize, control2: usize, target: usize) -> QuantumResult<()> {
    if control1 >= state.num_qubits || control2 >= state.num_qubits || target >= state.num_qubits {
        return Err(QuantumError::InvalidState(
            "Qubit indices out of range".to_string()
        ));
    }

    let size = state.amplitudes.len();
    let mask1 = 1 << control1;
    let mask2 = 1 << control2;
    let target_mask = 1 << target;

    for idx in 0..size {
        // Only flip target if both control qubits are 1
        if (idx & mask1) != 0 && (idx & mask2) != 0 && (idx & target_mask) == 0 {
            let idx_flipped = idx | target_mask;
            state.amplitudes.swap(idx, idx_flipped);
        }
    }

    Ok(())
}

/// Gate with depolarizing noise
pub fn apply_depolarizing_noise(
    state: &mut QuantumState,
    qubit: usize,
    probability: f64,
) -> QuantumResult<()> {
    use rand::Rng;

    if probability <= 0.0 {
        return Ok(());
    }

    let mut rng = rand::thread_rng();
    let r: f64 = rng.gen();

    if r < probability {
        // Apply random Pauli error
        let error_type: u8 = rng.gen_range(0..3);
        match error_type {
            0 => pauli_x(state, qubit)?,
            1 => pauli_y(state, qubit)?,
            2 => pauli_z(state, qubit)?,
            _ => unreachable!(),
        }
    }

    Ok(())
}

/// Quantum gate with platform-specific noise
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoisyGate {
    /// Gate fidelity (1.0 = perfect)
    pub fidelity: f64,
    /// Platform type
    pub platform: QubitPlatform,
    /// Gate duration in nanoseconds
    pub duration_ns: f64,
}

impl NoisyGate {
    /// Create a new noisy gate
    pub fn new(platform: QubitPlatform) -> Self {
        Self {
            fidelity: platform.two_qubit_fidelity(),
            platform,
            duration_ns: platform.gate_time_ns(),
        }
    }

    /// Apply gate with realistic noise
    pub fn apply_with_noise<F>(
        &self,
        state: &mut QuantumState,
        gate_fn: F,
        qubits: &[usize],
    ) -> QuantumResult<()>
    where
        F: FnOnce(&mut QuantumState) -> QuantumResult<()>,
    {
        // Apply the gate
        gate_fn(state)?;

        // Apply depolarizing noise based on fidelity
        let error_prob = 1.0 - self.fidelity;
        for &qubit in qubits {
            apply_depolarizing_noise(state, qubit, error_prob / qubits.len() as f64)?;
        }

        // Apply decoherence based on gate duration
        let decoherence_rate = self.duration_ns / (self.platform.coherence_time_us() * 1000.0);
        for &qubit in qubits {
            state.apply_decoherence(qubit, decoherence_rate);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_pauli_x() {
        let mut state = QuantumState::new(1);
        pauli_x(&mut state, 0).unwrap();

        // Should flip |0⟩ to |1⟩
        assert_relative_eq!(state.amplitudes[0].norm(), 0.0);
        assert_relative_eq!(state.amplitudes[1].norm(), 1.0);
    }

    #[test]
    fn test_hadamard() {
        let mut state = QuantumState::new(1);
        hadamard(&mut state, 0).unwrap();

        // Should create equal superposition
        assert_relative_eq!(state.amplitudes[0].norm(), FRAC_1_SQRT_2, epsilon = 1e-10);
        assert_relative_eq!(state.amplitudes[1].norm(), FRAC_1_SQRT_2, epsilon = 1e-10);
    }

    #[test]
    fn test_cnot() {
        let mut state = QuantumState::new(2);

        // Create superposition on control qubit
        hadamard(&mut state, 0).unwrap();

        // Apply CNOT
        cnot(&mut state, 0, 1).unwrap();

        // Should create Bell state: (|00⟩ + |11⟩) / sqrt(2)
        assert_relative_eq!(state.amplitudes[0].norm(), FRAC_1_SQRT_2, epsilon = 1e-10);
        assert_relative_eq!(state.amplitudes[3].norm(), FRAC_1_SQRT_2, epsilon = 1e-10);
        assert_relative_eq!(state.amplitudes[1].norm(), 0.0, epsilon = 1e-10);
        assert_relative_eq!(state.amplitudes[2].norm(), 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_rotation_gates() {
        let mut state = QuantumState::new(1);

        // RX(π) should act like X gate
        rx(&mut state, 0, PI).unwrap();
        assert_relative_eq!(state.amplitudes[1].norm(), 1.0, epsilon = 1e-10);

        // Reset
        state = QuantumState::new(1);

        // RY(π) should also flip the state
        ry(&mut state, 0, PI).unwrap();
        assert_relative_eq!(state.amplitudes[1].norm(), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_toffoli() {
        let mut state = QuantumState::new(3);

        // Set both control qubits to |1⟩
        pauli_x(&mut state, 0).unwrap();
        pauli_x(&mut state, 1).unwrap();

        // Apply Toffoli
        toffoli(&mut state, 0, 1, 2).unwrap();

        // Target should be flipped: |110⟩ -> |111⟩
        assert_relative_eq!(state.amplitudes[0b111].norm(), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_noisy_gate() {
        let platform = QubitPlatform::Superconducting;
        let gate = NoisyGate::new(platform);

        assert!(gate.fidelity > 0.98);
        assert!(gate.duration_ns < 100.0);
    }
}
