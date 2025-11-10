//! Radiation effects simulation
//!
//! Models cosmic radiation impact on quantum processors
//!
//! TODO: Full implementation with particle physics and error injection

use serde::{Deserialize, Serialize};

/// Radiation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationEvent {
    /// Event energy in MeV
    pub energy_mev: f64,
    /// Affected qubits
    pub affected_qubits: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radiation_event() {
        let event = RadiationEvent {
            energy_mev: 10.0,
            affected_qubits: vec![0, 1, 2],
        };
        assert_eq!(event.affected_qubits.len(), 3);
    }
}
