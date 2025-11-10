//! Quantum circuit construction and execution

use crate::gates::*;
use crate::qubits::{QuantumState, QubitPlatform};
use crate::{QuantumError, QuantumResult};
use serde::{Deserialize, Serialize};

/// Quantum gate instruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GateInstruction {
    /// Single-qubit Pauli X
    PauliX {
        /// Target qubit
        qubit: usize
    },
    /// Single-qubit Pauli Y
    PauliY {
        /// Target qubit
        qubit: usize
    },
    /// Single-qubit Pauli Z
    PauliZ {
        /// Target qubit
        qubit: usize
    },
    /// Hadamard gate
    Hadamard {
        /// Target qubit
        qubit: usize
    },
    /// Phase gate (S)
    Phase {
        /// Target qubit
        qubit: usize
    },
    /// T gate
    T {
        /// Target qubit
        qubit: usize
    },
    /// Rotation around X
    RX {
        /// Target qubit
        qubit: usize,
        /// Rotation angle
        theta: f64
    },
    /// Rotation around Y
    RY {
        /// Target qubit
        qubit: usize,
        /// Rotation angle
        theta: f64
    },
    /// Rotation around Z
    RZ {
        /// Target qubit
        qubit: usize,
        /// Rotation angle
        theta: f64
    },
    /// Controlled-NOT
    CNOT {
        /// Control qubit
        control: usize,
        /// Target qubit
        target: usize
    },
    /// Controlled-Z
    CZ {
        /// Control qubit
        control: usize,
        /// Target qubit
        target: usize
    },
    /// SWAP gate
    SWAP {
        /// First qubit
        qubit1: usize,
        /// Second qubit
        qubit2: usize
    },
    /// Toffoli gate (CCNOT)
    Toffoli {
        /// First control qubit
        control1: usize,
        /// Second control qubit
        control2: usize,
        /// Target qubit
        target: usize,
    },
    /// Measurement (collapses state)
    Measure {
        /// Qubit to measure
        qubit: usize
    },
    /// Barrier (for visualization/optimization)
    Barrier,
}

impl GateInstruction {
    /// Get qubits affected by this instruction
    pub fn qubits(&self) -> Vec<usize> {
        match self {
            GateInstruction::PauliX { qubit }
            | GateInstruction::PauliY { qubit }
            | GateInstruction::PauliZ { qubit }
            | GateInstruction::Hadamard { qubit }
            | GateInstruction::Phase { qubit }
            | GateInstruction::T { qubit }
            | GateInstruction::RX { qubit, .. }
            | GateInstruction::RY { qubit, .. }
            | GateInstruction::RZ { qubit, .. }
            | GateInstruction::Measure { qubit } => vec![*qubit],

            GateInstruction::CNOT { control, target }
            | GateInstruction::CZ { control, target } => vec![*control, *target],

            GateInstruction::SWAP { qubit1, qubit2 } => vec![*qubit1, *qubit2],

            GateInstruction::Toffoli {
                control1,
                control2,
                target,
            } => vec![*control1, *control2, *target],

            GateInstruction::Barrier => vec![],
        }
    }

    /// Execute the gate on a quantum state
    pub fn execute(&self, state: &mut QuantumState) -> QuantumResult<Option<bool>> {
        match self {
            GateInstruction::PauliX { qubit } => {
                pauli_x(state, *qubit)?;
                Ok(None)
            }
            GateInstruction::PauliY { qubit } => {
                pauli_y(state, *qubit)?;
                Ok(None)
            }
            GateInstruction::PauliZ { qubit } => {
                pauli_z(state, *qubit)?;
                Ok(None)
            }
            GateInstruction::Hadamard { qubit } => {
                hadamard(state, *qubit)?;
                Ok(None)
            }
            GateInstruction::Phase { qubit } => {
                phase(state, *qubit)?;
                Ok(None)
            }
            GateInstruction::T { qubit } => {
                t_gate(state, *qubit)?;
                Ok(None)
            }
            GateInstruction::RX { qubit, theta } => {
                rx(state, *qubit, *theta)?;
                Ok(None)
            }
            GateInstruction::RY { qubit, theta } => {
                ry(state, *qubit, *theta)?;
                Ok(None)
            }
            GateInstruction::RZ { qubit, theta } => {
                rz(state, *qubit, *theta)?;
                Ok(None)
            }
            GateInstruction::CNOT { control, target } => {
                cnot(state, *control, *target)?;
                Ok(None)
            }
            GateInstruction::CZ { control, target } => {
                cz(state, *control, *target)?;
                Ok(None)
            }
            GateInstruction::SWAP { qubit1, qubit2 } => {
                swap(state, *qubit1, *qubit2)?;
                Ok(None)
            }
            GateInstruction::Toffoli {
                control1,
                control2,
                target,
            } => {
                toffoli(state, *control1, *control2, *target)?;
                Ok(None)
            }
            GateInstruction::Measure { qubit } => {
                use rand::Rng;

                let prob_zero = state.measure_probability_zero(*qubit);
                let mut rng = rand::thread_rng();
                let measured_zero = rng.gen::<f64>() < prob_zero;
                let result = !measured_zero; // true = measured |1⟩, false = measured |0⟩

                // Collapse the state
                let size = state.amplitudes.len();

                for idx in 0..size {
                    let bit_val = ((idx >> qubit) & 1) == 1;
                    if bit_val != result {
                        state.amplitudes[idx] = num_complex::Complex64::new(0.0, 0.0);
                    }
                }

                state.normalize();
                Ok(Some(result))
            }
            GateInstruction::Barrier => Ok(None),
        }
    }
}

/// Quantum circuit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumCircuit {
    /// Number of qubits
    pub num_qubits: usize,
    /// Circuit instructions
    pub instructions: Vec<GateInstruction>,
    /// Circuit depth (longest path)
    pub depth: usize,
}

impl QuantumCircuit {
    /// Create a new quantum circuit
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            instructions: Vec::new(),
            depth: 0,
        }
    }

    /// Add a gate instruction
    pub fn add_gate(&mut self, gate: GateInstruction) -> QuantumResult<()> {
        // Validate qubit indices
        for qubit in gate.qubits() {
            if qubit >= self.num_qubits {
                return Err(QuantumError::CompilationError(format!(
                    "Qubit index {} out of range for circuit with {} qubits",
                    qubit, self.num_qubits
                )));
            }
        }

        self.instructions.push(gate);
        self.depth += 1; // Simplified depth calculation
        Ok(())
    }

    /// Execute the circuit on an initial state
    pub fn execute(&self, initial_state: &QuantumState) -> QuantumResult<CircuitResult> {
        if initial_state.num_qubits != self.num_qubits {
            return Err(QuantumError::InvalidState(
                "Initial state qubit count mismatch".to_string(),
            ));
        }

        let mut state = initial_state.clone();
        let mut measurements = Vec::new();

        for instruction in &self.instructions {
            if let Some(measurement) = instruction.execute(&mut state)? {
                measurements.push(measurement);
            }
        }

        Ok(CircuitResult {
            final_state: state,
            measurements,
        })
    }

    /// Get circuit gate count
    pub fn gate_count(&self) -> usize {
        self.instructions.len()
    }

    /// Get two-qubit gate count
    pub fn two_qubit_gate_count(&self) -> usize {
        self.instructions
            .iter()
            .filter(|gate| gate.qubits().len() >= 2)
            .count()
    }
}

/// Result of circuit execution
#[derive(Debug, Clone)]
pub struct CircuitResult {
    /// Final quantum state
    pub final_state: QuantumState,
    /// Measurement results (in order of measurements)
    pub measurements: Vec<bool>,
}

/// Circuit builder for convenient construction
pub struct CircuitBuilder {
    circuit: QuantumCircuit,
}

impl CircuitBuilder {
    /// Create a new circuit builder
    pub fn new(num_qubits: usize) -> Self {
        Self {
            circuit: QuantumCircuit::new(num_qubits),
        }
    }

    /// Add Hadamard gate
    pub fn h(mut self, qubit: usize) -> Self {
        let _ = self.circuit.add_gate(GateInstruction::Hadamard { qubit });
        self
    }

    /// Add Pauli X gate
    pub fn x(mut self, qubit: usize) -> Self {
        let _ = self.circuit.add_gate(GateInstruction::PauliX { qubit });
        self
    }

    /// Add Pauli Y gate
    pub fn y(mut self, qubit: usize) -> Self {
        let _ = self.circuit.add_gate(GateInstruction::PauliY { qubit });
        self
    }

    /// Add Pauli Z gate
    pub fn z(mut self, qubit: usize) -> Self {
        let _ = self.circuit.add_gate(GateInstruction::PauliZ { qubit });
        self
    }

    /// Add CNOT gate
    pub fn cnot(mut self, control: usize, target: usize) -> Self {
        let _ = self
            .circuit
            .add_gate(GateInstruction::CNOT { control, target });
        self
    }

    /// Add CZ gate
    pub fn cz(mut self, control: usize, target: usize) -> Self {
        let _ = self
            .circuit
            .add_gate(GateInstruction::CZ { control, target });
        self
    }

    /// Add RX gate
    pub fn rx(mut self, qubit: usize, theta: f64) -> Self {
        let _ = self
            .circuit
            .add_gate(GateInstruction::RX { qubit, theta });
        self
    }

    /// Add RY gate
    pub fn ry(mut self, qubit: usize, theta: f64) -> Self {
        let _ = self
            .circuit
            .add_gate(GateInstruction::RY { qubit, theta });
        self
    }

    /// Add RZ gate
    pub fn rz(mut self, qubit: usize, theta: f64) -> Self {
        let _ = self
            .circuit
            .add_gate(GateInstruction::RZ { qubit, theta });
        self
    }

    /// Add measurement
    pub fn measure(mut self, qubit: usize) -> Self {
        let _ = self.circuit.add_gate(GateInstruction::Measure { qubit });
        self
    }

    /// Build the circuit
    pub fn build(self) -> QuantumCircuit {
        self.circuit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::FRAC_1_SQRT_2;

    #[test]
    fn test_circuit_builder() {
        let circuit = CircuitBuilder::new(2)
            .h(0)
            .cnot(0, 1)
            .measure(0)
            .measure(1)
            .build();

        assert_eq!(circuit.num_qubits, 2);
        assert_eq!(circuit.gate_count(), 4);
    }

    #[test]
    fn test_bell_state_circuit() {
        let circuit = CircuitBuilder::new(2).h(0).cnot(0, 1).build();

        let initial_state = QuantumState::new(2);
        let result = circuit.execute(&initial_state).unwrap();

        // Should create Bell state: (|00⟩ + |11⟩) / sqrt(2)
        assert_relative_eq!(
            result.final_state.amplitudes[0].norm(),
            FRAC_1_SQRT_2,
            epsilon = 1e-10
        );
        assert_relative_eq!(
            result.final_state.amplitudes[3].norm(),
            FRAC_1_SQRT_2,
            epsilon = 1e-10
        );
    }

    #[test]
    fn test_circuit_measurement() {
        let circuit = CircuitBuilder::new(1).x(0).measure(0).build();

        let initial_state = QuantumState::new(1);
        let result = circuit.execute(&initial_state).unwrap();

        // Should always measure 1
        assert_eq!(result.measurements.len(), 1);
        assert_eq!(result.measurements[0], true);
    }

    #[test]
    fn test_gate_count() {
        let circuit = CircuitBuilder::new(3)
            .h(0)
            .cnot(0, 1)
            .cnot(1, 2)
            .build();

        assert_eq!(circuit.gate_count(), 3);
        assert_eq!(circuit.two_qubit_gate_count(), 2);
    }
}
