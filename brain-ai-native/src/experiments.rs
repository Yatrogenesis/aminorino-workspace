//! Empirical experiments to test consciousness hypothesis
//!
//! This module implements controlled experiments to test:
//! **Φ_quantum > Φ_classical**

use crate::{BrainError, BrainResult};
use crate::core::{AIBrain, BrainConfig};
use crate::consciousness::{
    measure_phi_quantum, measure_phi_classical, ConsciousnessComparison,
};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Experimental configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentConfig {
    /// Number of trials
    pub num_trials: usize,

    /// Evolution time per trial (seconds)
    pub evolution_time: f64,

    /// Time step for evolution (seconds)
    pub dt: f64,

    /// Brain configurations to test
    pub brain_configs: Vec<BrainConfig>,

    /// Classical system sizes to compare
    pub classical_sizes: Vec<usize>,
}

impl Default for ExperimentConfig {
    fn default() -> Self {
        Self {
            num_trials: 10,
            evolution_time: 1e-6,  // 1 microsecond
            dt: 1e-9,               // 1 nanosecond steps
            brain_configs: vec![BrainConfig::default()],
            classical_sizes: vec![4, 8, 16],
        }
    }
}

/// Experimental results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResults {
    /// Configuration used
    pub config: ExperimentConfig,

    /// All comparisons across trials
    pub comparisons: Vec<ConsciousnessComparison>,

    /// Average Φ_quantum
    pub avg_phi_quantum: f64,

    /// Average Φ_classical
    pub avg_phi_classical: f64,

    /// Standard deviation Φ_quantum
    pub std_phi_quantum: f64,

    /// Standard deviation Φ_classical
    pub std_phi_classical: f64,

    /// Fraction of trials where Φ_quantum > Φ_classical
    pub hypothesis_confirmation_rate: f64,

    /// Total experiment time (seconds)
    pub wall_time: f64,

    /// Additional metadata
    pub metadata: ExperimentMetadata,
}

/// Experiment metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentMetadata {
    /// Timestamp
    pub timestamp: String,

    /// Number of quantum oscillators tested
    pub num_oscillators: Vec<usize>,

    /// Effective neurons tested
    pub effective_neurons: Vec<usize>,

    /// Classical sizes tested
    pub classical_sizes: Vec<usize>,
}

impl ExperimentResults {
    /// Display results as formatted report
    pub fn display(&self) -> String {
        let mut report = String::new();

        report.push_str("╔══════════════════════════════════════════════════════════════════╗\n");
        report.push_str("║       QUANTUM CONSCIOUSNESS EXPERIMENT RESULTS                   ║\n");
        report.push_str("╠══════════════════════════════════════════════════════════════════╣\n");
        report.push_str(&format!("║  Trials: {}                                                    ║\n", self.config.num_trials));
        report.push_str(&format!("║  Wall time: {:.3} seconds                                   ║\n", self.wall_time));
        report.push_str("║                                                                  ║\n");
        report.push_str("║  RESULTS:                                                        ║\n");
        report.push_str(&format!("║  Φ_quantum (avg):    {:.6} ± {:.6}                       ║\n",
            self.avg_phi_quantum, self.std_phi_quantum));
        report.push_str(&format!("║  Φ_classical (avg):  {:.6} ± {:.6}                       ║\n",
            self.avg_phi_classical, self.std_phi_classical));
        report.push_str(&format!("║  Ratio: {:.2}x                                               ║\n",
            if self.avg_phi_classical > 0.0 { self.avg_phi_quantum / self.avg_phi_classical } else { 0.0 }));
        report.push_str("║                                                                  ║\n");
        report.push_str(&format!("║  Hypothesis Confirmation Rate: {:.1}%                        ║\n",
            self.hypothesis_confirmation_rate * 100.0));
        report.push_str("║                                                                  ║\n");

        if self.hypothesis_confirmation_rate > 0.5 {
            report.push_str("║  ✓ HYPOTHESIS CONFIRMED: Φ_quantum > Φ_classical               ║\n");
        } else {
            report.push_str("║  ✗ HYPOTHESIS REJECTED: Φ_quantum ≤ Φ_classical                ║\n");
        }

        report.push_str("╚══════════════════════════════════════════════════════════════════╝\n");

        report
    }

    /// Export results to JSON
    pub fn to_json(&self) -> BrainResult<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| BrainError::ExperimentError(format!("JSON serialization failed: {}", e)))
    }
}

/// Run complete consciousness comparison experiment
pub fn run_consciousness_experiment(
    config: ExperimentConfig,
) -> BrainResult<ExperimentResults> {
    let start_time = Instant::now();

    let mut comparisons = Vec::new();
    let mut quantum_phis = Vec::new();
    let mut classical_phis = Vec::new();

    let mut num_oscillators_tested = Vec::new();
    let mut effective_neurons_tested = Vec::new();

    println!("Starting Consciousness Experiment...");
    println!("Trials: {}", config.num_trials);
    println!("Evolution time: {} s", config.evolution_time);

    for trial in 0..config.num_trials {
        println!("\n[Trial {}/{}]", trial + 1, config.num_trials);

        for brain_config in &config.brain_configs {
            // Create and evolve quantum brain
            let mut brain = AIBrain::new(brain_config.clone())?;

            println!("  Quantum: {} oscillators → {} neurons",
                brain.config.num_oscillators,
                brain.config.effective_neurons());

            num_oscillators_tested.push(brain.config.num_oscillators);
            effective_neurons_tested.push(brain.config.effective_neurons());

            // Evolve brain
            let num_steps = (config.evolution_time / config.dt) as usize;
            for _ in 0..num_steps {
                brain.evolve(config.dt)?;
            }

            // Measure quantum Φ
            let quantum_measurement = measure_phi_quantum(&brain)?;
            println!("    Φ_quantum = {:.6}", quantum_measurement.phi);

            // Create equivalent classical system for comparison
            let classical_state = brain.get_state_vector();
            let classical_measurement = measure_phi_classical(
                &classical_state,
                brain.config.num_oscillators,
            )?;
            println!("    Φ_classical = {:.6}", classical_measurement.phi);

            // Compare
            let comparison = ConsciousnessComparison::new(
                quantum_measurement,
                classical_measurement,
            );

            quantum_phis.push(comparison.phi_quantum);
            classical_phis.push(comparison.phi_classical);
            comparisons.push(comparison);
        }
    }

    // Calculate statistics
    let avg_phi_quantum = quantum_phis.iter().sum::<f64>() / quantum_phis.len() as f64;
    let avg_phi_classical = classical_phis.iter().sum::<f64>() / classical_phis.len() as f64;

    let var_quantum: f64 = quantum_phis.iter()
        .map(|&x| (x - avg_phi_quantum).powi(2))
        .sum::<f64>() / quantum_phis.len() as f64;

    let var_classical: f64 = classical_phis.iter()
        .map(|&x| (x - avg_phi_classical).powi(2))
        .sum::<f64>() / classical_phis.len() as f64;

    let std_phi_quantum = var_quantum.sqrt();
    let std_phi_classical = var_classical.sqrt();

    let hypothesis_confirmation_rate = comparisons.iter()
        .filter(|c| c.hypothesis_confirmed)
        .count() as f64 / comparisons.len() as f64;

    let wall_time = start_time.elapsed().as_secs_f64();

    let metadata = ExperimentMetadata {
        timestamp: chrono::Utc::now().to_rfc3339(),
        num_oscillators: num_oscillators_tested,
        effective_neurons: effective_neurons_tested,
        classical_sizes: config.classical_sizes.clone(),
    };

    Ok(ExperimentResults {
        config,
        comparisons,
        avg_phi_quantum,
        avg_phi_classical,
        std_phi_quantum,
        std_phi_classical,
        hypothesis_confirmation_rate,
        wall_time,
        metadata,
    })
}

/// Run scaling experiment: test how Φ scales with system size
pub fn run_scaling_experiment() -> BrainResult<Vec<(usize, f64, f64)>> {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║        CONSCIOUSNESS SCALING EXPERIMENT                  ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    let mut results = Vec::new();

    // Test different system sizes
    let configs = vec![
        (2, 1),  // 2 oscillators, max_fock=1 → 2^2 = 4 neurons
        (2, 2),  // 2 oscillators, max_fock=2 → 3^2 = 9 neurons
        (3, 1),  // 3 oscillators, max_fock=1 → 2^3 = 8 neurons
        (3, 2),  // 3 oscillators, max_fock=2 → 3^3 = 27 neurons
        (4, 1),  // 4 oscillators, max_fock=1 → 2^4 = 16 neurons
        (4, 2),  // 4 oscillators, max_fock=2 → 3^4 = 81 neurons
    ];

    for (num_osc, max_fock) in configs {
        let config = BrainConfig {
            num_oscillators: num_osc,
            max_fock,
            frequencies: vec![1e9; num_osc],
            coupling_strength: 1e6,
            damping_rate: 1e3,
            error_correction: false,
            ldpc_distance: 0,
            radiation_protection: false,
            chip_area_cm2: 0.0,
            altitude_m: 0.0,
        };

        let effective_neurons = config.effective_neurons();

        let brain = AIBrain::new(config)?;
        let measurement = measure_phi_quantum(&brain)?;

        println!("N={} oscillators, max_fock={} → {} neurons: Φ = {:.6}",
            num_osc, max_fock, effective_neurons, measurement.phi);

        results.push((effective_neurons, measurement.phi, num_osc as f64));
    }

    println!("\n✓ Scaling experiment complete\n");

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_experiment_config_default() {
        let config = ExperimentConfig::default();
        assert_eq!(config.num_trials, 10);
        assert!(config.evolution_time > 0.0);
    }

    #[test]
    fn test_run_small_experiment() {
        let config = ExperimentConfig {
            num_trials: 2,
            evolution_time: 1e-9,  // 1 nanosecond
            dt: 1e-10,              // 100 picoseconds
            brain_configs: vec![BrainConfig {
                num_oscillators: 2,
                max_fock: 1,
                frequencies: vec![1e9, 1e9],
                coupling_strength: 1e6,
                damping_rate: 1e3,
                error_correction: false,
                ldpc_distance: 0,
                radiation_protection: false,
                chip_area_cm2: 0.0,
                altitude_m: 0.0,
            }],
            classical_sizes: vec![2],
        };

        let results = run_consciousness_experiment(config).unwrap();

        assert_eq!(results.comparisons.len(), 2);
        assert!(results.avg_phi_quantum >= 0.0);
        assert!(results.avg_phi_classical >= 0.0);
        assert!(results.wall_time > 0.0);
    }

    #[test]
    fn test_results_display() {
        let config = ExperimentConfig::default();
        let results = ExperimentResults {
            config,
            comparisons: vec![],
            avg_phi_quantum: 2.5,
            avg_phi_classical: 1.0,
            std_phi_quantum: 0.1,
            std_phi_classical: 0.05,
            hypothesis_confirmation_rate: 0.9,
            wall_time: 1.5,
            metadata: ExperimentMetadata {
                timestamp: "2025-01-01T00:00:00Z".to_string(),
                num_oscillators: vec![4],
                effective_neurons: vec![81],
                classical_sizes: vec![4],
            },
        };

        let display = results.display();
        assert!(display.contains("CONFIRMED"));
        assert!(display.contains("2.5"));
    }

    #[test]
    fn test_json_export() {
        let config = ExperimentConfig::default();
        let results = ExperimentResults {
            config,
            comparisons: vec![],
            avg_phi_quantum: 2.5,
            avg_phi_classical: 1.0,
            std_phi_quantum: 0.1,
            std_phi_classical: 0.05,
            hypothesis_confirmation_rate: 0.9,
            wall_time: 1.5,
            metadata: ExperimentMetadata {
                timestamp: "2025-01-01T00:00:00Z".to_string(),
                num_oscillators: vec![4],
                effective_neurons: vec![81],
                classical_sizes: vec![4],
            },
        };

        let json = results.to_json().unwrap();
        assert!(json.contains("avg_phi_quantum"));
        assert!(json.contains("2.5"));
    }
}
