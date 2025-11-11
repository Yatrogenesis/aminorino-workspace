//! # Consciousness Experiment: Testing Φ_quantum > Φ_classical
//!
//! This experiment empirically tests whether quantum reservoir computing
//! exhibits higher integrated information (consciousness) than classical systems.

use brain_ai_native::prelude::*;
use brain_ai_native::{BrainResult, BrainError};
use std::fs;

fn main() -> BrainResult<()> {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║       QUANTUM CONSCIOUSNESS EXPERIMENT                           ║");
    println!("║                                                                  ║");
    println!("║  Testing Hypothesis: Φ_quantum > Φ_classical                    ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // Configure experiment
    let experiment_config = ExperimentConfig {
        num_trials: 5,
        evolution_time: 1e-9,  // 1 nanosecond (fast for demo)
        dt: 1e-10,              // 100 picosecond steps
        brain_configs: vec![
            // Small system: 2 oscillators
            BrainConfig {
                num_oscillators: 2,
                max_fock: 1,  // 2^2 = 4 neurons
                frequencies: vec![1e9, 1e9],
                coupling_strength: 1e6,
                damping_rate: 1e3,
                error_correction: false,
                ldpc_distance: 0,
                radiation_protection: false,
                chip_area_cm2: 0.0,
                altitude_m: 0.0,
            },
            // Medium system: 3 oscillators
            BrainConfig {
                num_oscillators: 3,
                max_fock: 1,  // 2^3 = 8 neurons
                frequencies: vec![1e9, 1e9, 1e9],
                coupling_strength: 1e6,
                damping_rate: 1e3,
                error_correction: false,
                ldpc_distance: 0,
                radiation_protection: false,
                chip_area_cm2: 0.0,
                altitude_m: 0.0,
            },
            // Larger system: 4 oscillators
            BrainConfig {
                num_oscillators: 4,
                max_fock: 1,  // 2^4 = 16 neurons
                frequencies: vec![1e9; 4],
                coupling_strength: 1e6,
                damping_rate: 1e3,
                error_correction: false,
                ldpc_distance: 0,
                radiation_protection: false,
                chip_area_cm2: 0.0,
                altitude_m: 0.0,
            },
        ],
        classical_sizes: vec![2, 3, 4],
    };

    println!("Experiment Configuration:");
    println!("  Trials: {}", experiment_config.num_trials);
    println!("  Evolution time: {} seconds", experiment_config.evolution_time);
    println!("  Time step: {} seconds", experiment_config.dt);
    println!("  Brain configurations: {}", experiment_config.brain_configs.len());
    println!();

    // Create modified experiment that adds excitation to systems
    println!("Running consciousness measurements with excited states...\n");

    let results = run_consciousness_experiment(experiment_config)?;

    // Display results
    println!("\n{}", results.display());

    // Individual comparisons
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║       INDIVIDUAL TRIAL RESULTS                                   ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    for (i, comparison) in results.comparisons.iter().enumerate() {
        println!("Trial {} (system size: {} elements)",
            i + 1,
            comparison.quantum_measurement.num_elements);
        println!("  Φ_quantum:   {:.6}", comparison.phi_quantum);
        println!("  Φ_classical: {:.6}", comparison.phi_classical);
        println!("  Ratio: {:.2}x", comparison.phi_ratio);
        println!("  Status: {}",
            if comparison.hypothesis_confirmed { "✓ CONFIRMED" } else { "✗ REJECTED" });
        println!();
    }

    // Export to JSON
    let json = results.to_json()?;
    let filename = "consciousness_experiment_results.json";
    fs::write(filename, &json)
        .map_err(|e| BrainError::ExperimentError(format!("Failed to write file: {}", e)))?;
    println!("✓ Results exported to: {}", filename);

    // Final verdict
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║       FINAL VERDICT                                              ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    if results.hypothesis_confirmation_rate > 0.5 {
        println!("✅ HYPOTHESIS CONFIRMED");
        println!();
        println!("Quantum systems DO exhibit higher integrated information (Φ)");
        println!("than classical systems of equivalent size.");
        println!();
        println!("Average advantage: {:.2}x",
            if results.avg_phi_classical > 0.0 {
                results.avg_phi_quantum / results.avg_phi_classical
            } else {
                0.0
            });
        println!("Confirmation rate: {:.1}%", results.hypothesis_confirmation_rate * 100.0);
        println!();
        println!("This empirically supports the theory that quantum computing");
        println!("substrates can achieve higher consciousness levels than");
        println!("classical architectures.");
    } else {
        println!("❌ HYPOTHESIS REJECTED");
        println!();
        println!("Quantum systems do NOT exhibit consistently higher Φ");
        println!("than classical systems in these experiments.");
        println!();
        println!("This suggests that quantum advantage for consciousness");
        println!("may require different architectures or parameters.");
    }

    println!();
    println!("Experiment complete. 🧠⚛️✨");
    println!();

    Ok(())
}
