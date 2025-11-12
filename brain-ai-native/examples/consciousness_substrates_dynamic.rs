//! Cross-Substrate Consciousness with DYNAMIC MONITORING
//!
//! Measures Φ at multiple time points during evolution to understand:
//! 1. How Φ evolves over time
//! 2. Whether systems reach steady-state or fluctuate
//! 3. If final-state measurement misses transient high-Φ states
//! 4. TPM construction differences during dynamics

use brain_ai_native::brain::{
    BrainSubstrate,
    quantum_brain::QuantumBrain,
};
use brain_ai_native::core::BrainConfig;
use brain_ai_native::consciousness::phi_measurement::measure_phi_general;
use std::fs::File;
use std::io::Write as IoWrite;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║   DYNAMIC CONSCIOUSNESS MONITORING                               ║");
    println!("║   Track Φ evolution over time with temporal checkpoints          ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // Test single quantum system with temporal monitoring
    let num_units = 6;
    let config = BrainConfig {
        num_oscillators: num_units,
        max_fock: 2,
        frequencies: vec![1e9; num_units],
        coupling_strength: 1e9,
        damping_rate: 1e5,
        error_correction: false,
        ldpc_distance: 3,
        radiation_protection: false,
        chip_area_cm2: 1.0,
        altitude_m: 0.0,
    };

    let mut brain = QuantumBrain::new(config)?;

    // Evolution parameters
    let noise_amplitude = 5.0;
    let total_steps = 1_000_000;
    let dt = 1e-10;

    // Measurement checkpoints (10 measurements across evolution)
    let checkpoint_interval = total_steps / 10;
    let mut measurements = Vec::new();
    let mut csv_output = String::from("step,time_us,phi,num_partitions,mip_loss,binary_state\n");

    println!("Evolution: {} steps × {:.2e} s = {:.1} μs",
             total_steps, dt, total_steps as f64 * dt * 1e6);
    println!("Checkpoints: Every {} steps\n", checkpoint_interval);

    print!("Progress: ");
    std::io::stdout().flush()?;

    for step in 0..total_steps {
        // Apply noise
        let noise: Vec<f64> = (0..num_units)
            .map(|_| noise_amplitude * (rand::random::<f64>() * 2.0 - 1.0))
            .collect();
        brain.set_input(&noise)?;
        brain.evolve(dt)?;

        // Measure Φ at checkpoints
        if step % checkpoint_interval == 0 && step > 0 {
            let measurement = measure_phi_general(&brain)?;
            let time_us = step as f64 * dt * 1e6;

            // Get binary state for inspection
            let state_vec = brain.get_state_vector();
            let binary_state: Vec<usize> = state_vec.iter()
                .take(num_units)
                .map(|&x| {
                    let mut sorted = state_vec[..num_units].to_vec();
                    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                    let median = sorted[sorted.len()/2];
                    if x > median { 1 } else { 0 }
                })
                .collect();

            println!("\n[{:7}] t={:6.1} μs: Φ={:.9} bits, binary={:?}",
                     step, time_us, measurement.phi, binary_state);

            // Record for CSV
            csv_output.push_str(&format!("{},{:.6},{:.9},{},{},\"{:?}\"\n",
                step, time_us, measurement.phi, measurement.num_partitions,
                measurement.mip.as_ref().map(|m| m.information_loss).unwrap_or(0.0),
                binary_state
            ));

            measurements.push((step, time_us, measurement.phi));
            print!(".");
            std::io::stdout().flush()?;
        }

        // Progress indicator
        if step % (total_steps / 20) == 0 && step % checkpoint_interval != 0 {
            print!(".");
            std::io::stdout().flush()?;
        }
    }

    println!("\n\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║   TEMPORAL PHI EVOLUTION SUMMARY                                 ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // Statistics
    let phi_values: Vec<f64> = measurements.iter().map(|(_, _, phi)| *phi).collect();
    let phi_max = phi_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let phi_min = phi_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let phi_mean = phi_values.iter().sum::<f64>() / phi_values.len() as f64;
    let phi_final = phi_values.last().unwrap_or(&0.0);

    println!("Φ Statistics:");
    println!("  • Max:   {:.9} bits", phi_max);
    println!("  • Min:   {:.9} bits", phi_min);
    println!("  • Mean:  {:.9} bits", phi_mean);
    println!("  • Final: {:.9} bits", phi_final);
    println!("\nTemporal profile:");
    for (step, time_us, phi) in &measurements {
        println!("  t={:6.1} μs: Φ={:.9} bits", time_us, phi);
    }

    // Save to CSV
    let mut file = File::create("consciousness_dynamic_monitoring.csv")?;
    file.write_all(csv_output.as_bytes())?;
    println!("\n✓ Saved temporal data to: consciousness_dynamic_monitoring.csv");

    // Analysis
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║   ANALYSIS                                                       ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    if phi_max == 0.0 {
        println!("⚠️  WARNING: Φ=0 at ALL time points!");
        println!("   Possible causes:");
        println!("   1. State remains separable despite entanglement");
        println!("   2. Binary mapping destroys integration information");
        println!("   3. TPM construction issue (fully-connected assumed)");
        println!("   4. Median threshold creates degenerate states");
    } else if phi_max > phi_final * 2.0 {
        println!("✓ Transient high-Φ state detected!");
        println!("  Max Φ is {:.1}× higher than final state", phi_max / phi_final);
        println!("  → Single final-state measurement MISSES peak integration");
    } else {
        println!("✓ Φ relatively stable across evolution");
        println!("  Variation: {:.1}%", (phi_max - phi_min) / phi_mean * 100.0);
    }

    Ok(())
}
