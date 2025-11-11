//! Cross-substrate consciousness comparison
//!
//! This module provides tools to compare integrated information (Φ) across
//! different computational substrates: quantum, biological, and hybrid.

use crate::BrainResult;
use crate::consciousness::phi_measurement::ConsciousnessMeasurement;
use serde::{Serialize, Deserialize};

/// Alias for ConsciousnessMeasurement for convenience
pub type PhiMeasurement = ConsciousnessMeasurement;

/// Results from cross-substrate consciousness comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossSubstrateResults {
    /// Quantum substrate Φ
    pub quantum_phi: f64,

    /// Biological substrate Φ
    pub biological_phi: f64,

    /// Hybrid substrate Φ
    pub hybrid_phi: f64,

    /// Quantum measurement details
    pub quantum_measurement: PhiMeasurement,

    /// Biological measurement details
    pub biological_measurement: PhiMeasurement,

    /// Hybrid measurement details
    pub hybrid_measurement: PhiMeasurement,

    /// Number of units in each substrate
    pub quantum_units: usize,
    /// Number of biological units
    pub biological_units: usize,
    /// Number of hybrid units
    pub hybrid_units: usize,

    /// Statistical comparison
    pub comparison: SubstrateComparison,
}

/// Statistical comparison between substrates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstrateComparison {
    /// Φ_hybrid / Φ_quantum ratio
    pub hybrid_quantum_ratio: f64,

    /// Φ_hybrid / Φ_biological ratio
    pub hybrid_biological_ratio: f64,

    /// Φ_quantum / Φ_biological ratio
    pub quantum_biological_ratio: f64,

    /// Winner (highest Φ)
    pub winner: String,

    /// Hypothesis confirmed? (Φ_hybrid > Φ_quantum > Φ_biological)
    pub hypothesis_confirmed: bool,
}

impl CrossSubstrateResults {
    /// Create results from measurements
    pub fn new(
        quantum_measurement: PhiMeasurement,
        biological_measurement: PhiMeasurement,
        hybrid_measurement: PhiMeasurement,
        quantum_units: usize,
        biological_units: usize,
        hybrid_units: usize,
    ) -> Self {
        let quantum_phi = quantum_measurement.phi;
        let biological_phi = biological_measurement.phi;
        let hybrid_phi = hybrid_measurement.phi;

        // Calculate ratios (handle division by zero)
        let hybrid_quantum_ratio = if quantum_phi > 0.0 {
            hybrid_phi / quantum_phi
        } else {
            f64::INFINITY
        };

        let hybrid_biological_ratio = if biological_phi > 0.0 {
            hybrid_phi / biological_phi
        } else {
            f64::INFINITY
        };

        let quantum_biological_ratio = if biological_phi > 0.0 {
            quantum_phi / biological_phi
        } else {
            f64::INFINITY
        };

        // Determine winner
        let winner = if hybrid_phi > quantum_phi && hybrid_phi > biological_phi {
            "Hybrid".to_string()
        } else if quantum_phi > biological_phi {
            "Quantum".to_string()
        } else {
            "Biological".to_string()
        };

        // Check hypothesis: Φ_hybrid > Φ_quantum > Φ_biological
        let hypothesis_confirmed = hybrid_phi > quantum_phi && quantum_phi > biological_phi;

        let comparison = SubstrateComparison {
            hybrid_quantum_ratio,
            hybrid_biological_ratio,
            quantum_biological_ratio,
            winner,
            hypothesis_confirmed,
        };

        Self {
            quantum_phi,
            biological_phi,
            hybrid_phi,
            quantum_measurement,
            biological_measurement,
            hybrid_measurement,
            quantum_units,
            biological_units,
            hybrid_units,
            comparison,
        }
    }

    /// Display results in a formatted table
    pub fn display(&self) -> String {
        format!(
            r#"
╔══════════════════════════════════════════════════════════════════╗
║          CROSS-SUBSTRATE CONSCIOUSNESS COMPARISON                ║
╚══════════════════════════════════════════════════════════════════╝

SUBSTRATE MEASUREMENTS:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

 Quantum (Pure Quantum Reservoir):
   • Units:        {} oscillators
   • Φ:            {:.9} bits
   • Partitions:   {}
   • MIP loss:     {:.9} bits

 Biological (Hodgkin-Huxley Neurons):
   • Units:        {} neurons
   • Φ:            {:.9} bits
   • Partitions:   {}
   • MIP loss:     {:.9} bits

 Hybrid (Quantum + Biological):
   • Units:        {} total ({} quantum + {} biological)
   • Φ:            {:.9} bits
   • Partitions:   {}
   • MIP loss:     {:.9} bits

STATISTICAL COMPARISON:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

 Ratios:
   • Φ_hybrid / Φ_quantum     = {:.3}x
   • Φ_hybrid / Φ_biological  = {:.3}x
   • Φ_quantum / Φ_biological = {:.3}x

 Winner: {} (highest Φ)

 Hypothesis (Φ_hybrid > Φ_quantum > Φ_biological): {}

"#,
            self.quantum_units,
            self.quantum_phi,
            self.quantum_measurement.num_partitions,
            self.quantum_measurement.mip.as_ref().map(|m| m.information_loss).unwrap_or(0.0),
            self.biological_units,
            self.biological_phi,
            self.biological_measurement.num_partitions,
            self.biological_measurement.mip.as_ref().map(|m| m.information_loss).unwrap_or(0.0),
            self.hybrid_units,
            self.hybrid_phi,
            self.hybrid_units - self.biological_units,  // quantum units in hybrid
            self.biological_units,
            self.hybrid_measurement.num_partitions,
            self.hybrid_measurement.mip.as_ref().map(|m| m.information_loss).unwrap_or(0.0),
            self.comparison.hybrid_quantum_ratio,
            self.comparison.hybrid_biological_ratio,
            self.comparison.quantum_biological_ratio,
            self.comparison.winner,
            if self.comparison.hypothesis_confirmed { "✅ CONFIRMED" } else { "❌ REJECTED" }
        )
    }

    /// Export to JSON
    pub fn to_json(&self) -> BrainResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| crate::BrainError::ExperimentError(format!("JSON serialization failed: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    // Note: Tests removed temporarily - will be restored once ConsciousnessMeasurement
    // can be constructed easily in tests
}
