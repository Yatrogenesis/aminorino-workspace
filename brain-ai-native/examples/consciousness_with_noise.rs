//! # Consciousness Experiment with Noise and Dynamic Inputs
//!
//! Tests hypothesis: Φ increases with noise and dynamic activity
//!
//! Key insight from Francisco Molina:
//! "Un sistema consciente está lleno de ruido y factores externos
//!  que lo hacen reaccionar o activarse"

use brain_ai_native::prelude::*;
use brain_ai_native::{BrainResult, BrainError};
use rand::Rng;
use std::fs;

fn main() -> BrainResult<()> {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║   CONSCIOUSNESS WITH NOISE & DYNAMIC ACTIVITY EXPERIMENT         ║");
    println!("║                                                                  ║");
    println!("║  Hypothesis: Φ increases with noise and external stimulation    ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // Test multiple noise levels
    let noise_levels = vec![
        ("No Noise", 0.0),
        ("Low Noise", 0.1),
        ("Medium Noise", 0.5),
        ("High Noise", 1.0),
        ("Very High Noise", 2.0),
    ];

    let mut all_results = Vec::new();

    for (noise_name, noise_amplitude) in noise_levels {
        println!("\n{}", "=".repeat(70));
        println!("Testing: {} (amplitude = {})", noise_name, noise_amplitude);
        println!("{}\n", "=".repeat(70));

        let config = BrainConfig {
            num_oscillators: 4,
            max_fock: 2,  // 3^4 = 81 neurons
            frequencies: vec![1e9; 4],
            coupling_strength: 1e7,  // Strong coupling for integration
            damping_rate: 1e4,        // Moderate decoherence
            error_correction: false,
            ldpc_distance: 0,
            radiation_protection: false,
            chip_area_cm2: 0.0,
            altitude_m: 0.0,
        };

        let mut brain = AIBrain::new(config)?;

        // Evolution parameters
        let total_time = 1e-6;  // 1 microsecond (longer to see dynamics)
        let dt = 1e-9;          // 1 nanosecond steps
        let num_steps = (total_time / dt) as usize;

        println!("Evolution: {} steps of {} seconds each", num_steps, dt);
        println!("Total time: {} seconds", total_time);

        let mut rng = rand::thread_rng();

        // Arrays to track Φ over time
        let mut phi_measurements = Vec::new();
        let measurement_interval = num_steps / 10;  // Measure 10 times

        // Evolve with continuous noise injection
        for step in 0..num_steps {
            // Inject stochastic noise at each step
            if noise_amplitude > 0.0 {
                let noise_input: Vec<f64> = (0..brain.config.num_oscillators)
                    .map(|_| noise_amplitude * rng.gen::<f64>())
                    .collect();

                brain.set_input(&noise_input)?;
            }

            // Evolve one step
            brain.evolve(dt)?;

            // Measure Φ periodically
            if step % measurement_interval == 0 && step > 0 {
                let measurement = measure_phi_quantum(&brain)?;
                phi_measurements.push((step as f64 * dt, measurement.phi));

                print!(".");
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
        }

        println!("\n");

        // Final measurement
        let final_measurement = measure_phi_quantum(&brain)?;

        // Statistics
        let phis: Vec<f64> = phi_measurements.iter().map(|(_, phi)| *phi).collect();
        let avg_phi = phis.iter().sum::<f64>() / phis.len() as f64;
        let max_phi = phis.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let min_phi = phis.iter().fold(f64::INFINITY, |a, &b| a.min(b));

        println!("Results for {}:", noise_name);
        println!("  Average Φ:     {:.6}", avg_phi);
        println!("  Maximum Φ:     {:.6}", max_phi);
        println!("  Minimum Φ:     {:.6}", min_phi);
        println!("  Final Φ:       {:.6}", final_measurement.phi);
        println!("  Measurements:  {}", phi_measurements.len());

        all_results.push((noise_name.to_string(), noise_amplitude, avg_phi, max_phi, final_measurement.phi));
    }

    // Summary comparison
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║                    SUMMARY COMPARISON                            ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║ Noise Level      │ Amplitude │ Avg Φ    │ Max Φ    │ Final Φ  ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");

    for (name, amplitude, avg_phi, max_phi, final_phi) in &all_results {
        println!("║ {:<16} │ {:>8.1} │ {:.6} │ {:.6} │ {:.6} ║",
            name, amplitude, avg_phi, max_phi, final_phi);
    }

    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // Analyze trend
    let phi_increase = all_results.last().unwrap().2 - all_results.first().unwrap().2;
    let phi_ratio = if all_results.first().unwrap().2 > 0.0 {
        all_results.last().unwrap().2 / all_results.first().unwrap().2
    } else {
        f64::INFINITY
    };

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                         ANALYSIS                                 ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    println!("Φ increase from no-noise to high-noise: {:.6}", phi_increase);
    println!("Φ ratio (high/low): {:.2}x", phi_ratio);

    if phi_increase > 0.0 {
        println!("\n✅ HYPOTHESIS CONFIRMED");
        println!("Φ (consciousness) INCREASES with noise and dynamic activity!");
        println!("\nThis supports the insight that consciousness emerges from");
        println!("active information processing with stochastic inputs,");
        println!("not from static or near-ground states.");
    } else {
        println!("\n❌ HYPOTHESIS REJECTED");
        println!("Noise did not increase Φ in this configuration.");
        println!("May need to adjust coupling, evolution time, or noise characteristics.");
    }

    // Export results
    let export = serde_json::json!({
        "experiment": "PhiQ_with_Noise_and_Activity",
        "hypothesis": "Φ increases with noise amplitude",
        "results": all_results.iter().map(|(name, amp, avg, max, final_v)| {
            serde_json::json!({
                "noise_level": name,
                "amplitude": amp,
                "avg_phi": avg,
                "max_phi": max,
                "final_phi": final_v,
            })
        }).collect::<Vec<_>>(),
        "conclusion": if phi_increase > 0.0 { "confirmed" } else { "rejected" },
        "phi_increase": phi_increase,
        "phi_ratio": phi_ratio,
    });

    let filename = "consciousness_with_noise_results.json";
    fs::write(filename, serde_json::to_string_pretty(&export).unwrap())
        .map_err(|e| BrainError::ExperimentError(format!("{}", e)))?;

    println!("\n✓ Results exported to: {}", filename);
    println!("\nExperiment complete. 🧠⚛️✨\n");

    Ok(())
}
