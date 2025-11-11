//! Cross-Substrate Consciousness Comparison
//!
//! Paper 2: Comparing integrated information (Φ) across three substrates:
//! 1. Quantum (pure quantum reservoir)
//! 2. Biological (Hodgkin-Huxley neurons)
//! 3. Hybrid (quantum + biological coupling)
//!
//! Hypothesis: Φ_hybrid > Φ_quantum > Φ_biological

use brain_ai_native::brain::{
    BrainSubstrate,
    quantum_brain::QuantumBrain,
    biological_brain::{BiologicalBrain, BiologicalBrainConfig},
    hybrid_brain::{HybridBrain, HybridBrainConfig},
};
use brain_ai_native::core::BrainConfig;
use brain_ai_native::consciousness::phi_measurement::measure_phi_general;
use brain_ai_native::consciousness::cross_substrate::CrossSubstrateResults;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║   CROSS-SUBSTRATE CONSCIOUSNESS COMPARISON                       ║");
    println!("║                                                                  ║");
    println!("║   Hypothesis: Φ_hybrid > Φ_quantum > Φ_biological              ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // Test 4 system sizes
    let sizes = vec![
        ("Small", 4),
        ("Medium", 6),
        ("Large", 8),
        ("XLarge", 10),
    ];

    let mut all_results = Vec::new();

    for (size_name, num_units) in sizes {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Testing {} system ({} units)", size_name, num_units);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        // 1. Quantum substrate
        println!("[1/3] Quantum substrate...");
        let quantum_config = BrainConfig {
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

        let mut quantum_brain = QuantumBrain::new(quantum_config)?;

        // Evolve with noise
        let noise_amplitude = 5.0;
        let evolution_steps = 10000;
        let dt = 1e-10;

        for _ in 0..evolution_steps {
            let noise: Vec<f64> = (0..num_units)
                .map(|_| noise_amplitude * (rand::random::<f64>() * 2.0 - 1.0))
                .collect();
            quantum_brain.set_input(&noise)?;
            quantum_brain.evolve(dt)?;
        }

        let quantum_measurement = measure_phi_general(&quantum_brain)?;
        println!("  Φ_quantum = {:.9} bits", quantum_measurement.phi);

        // 2. Biological substrate
        println!("[2/3] Biological substrate...");
        let bio_config = BiologicalBrainConfig {
            num_neurons: num_units,
            coupling_strength: 0.1,
            neuron_config: hodgkin_huxley::neuron_types::NeuronConfig::regular_spiking(),
        };

        let mut bio_brain = BiologicalBrain::new(bio_config)?;

        // Evolve with input
        for _ in 0..evolution_steps {
            let input: Vec<f64> = (0..num_units)
                .map(|_| 15.0 + rand::random::<f64>() * 5.0)  // 15-20 μA/cm²
                .collect();
            bio_brain.set_input(&input)?;
            bio_brain.evolve(0.01)?;  // 0.01 ms timestep for HH
        }

        let bio_measurement = measure_phi_general(&bio_brain)?;
        println!("  Φ_biological = {:.9} bits", bio_measurement.phi);

        // 3. Hybrid substrate
        println!("[3/3] Hybrid substrate...");
        let hybrid_config = HybridBrainConfig {
            quantum_config: BrainConfig {
                num_oscillators: num_units / 2,  // Half quantum, half biological
                max_fock: 2,
                frequencies: vec![1e9; num_units / 2],
                coupling_strength: 1e9,
                damping_rate: 1e5,
                error_correction: false,
                ldpc_distance: 3,
                radiation_protection: false,
                chip_area_cm2: 1.0,
                altitude_m: 0.0,
            },
            biological_config: BiologicalBrainConfig {
                num_neurons: num_units / 2,
                coupling_strength: 0.1,
                neuron_config: hodgkin_huxley::neuron_types::NeuronConfig::regular_spiking(),
            },
            coupling_strength: 0.5,
        };

        let mut hybrid_brain = HybridBrain::new(hybrid_config)?;

        // Evolve hybrid
        for _ in 0..evolution_steps {
            let quantum_noise: Vec<f64> = (0..num_units/2)
                .map(|_| noise_amplitude * (rand::random::<f64>() * 2.0 - 1.0))
                .collect();
            let bio_input: Vec<f64> = (0..num_units/2)
                .map(|_| 15.0 + rand::random::<f64>() * 5.0)
                .collect();

            let mut combined = quantum_noise;
            combined.extend(bio_input);

            hybrid_brain.set_input(&combined)?;
            hybrid_brain.evolve(dt)?;
        }

        let hybrid_measurement = measure_phi_general(&hybrid_brain)?;
        println!("  Φ_hybrid = {:.9} bits", hybrid_measurement.phi);

        // Create comparison results
        let results = CrossSubstrateResults::new(
            quantum_measurement.clone(),
            bio_measurement.clone(),
            hybrid_measurement.clone(),
            num_units,
            num_units,
            num_units,
        );

        println!("\n{}", results.display());

        all_results.push((size_name.to_string(), results));
    }

    // Summary across all sizes
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║                    SUMMARY ACROSS ALL SIZES                      ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    println!("{:<10} {:<15} {:<15} {:<15} {:<10}",
        "Size", "Φ_Quantum", "Φ_Biological", "Φ_Hybrid", "Winner");
    println!("{}", "─".repeat(70));

    let mut hypothesis_confirmations = 0;
    for (size, results) in &all_results {
        println!("{:<10} {:<15.9} {:<15.9} {:<15.9} {:<10}",
            size,
            results.quantum_phi,
            results.biological_phi,
            results.hybrid_phi,
            results.comparison.winner
        );

        if results.comparison.hypothesis_confirmed {
            hypothesis_confirmations += 1;
        }
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("Hypothesis confirmed in {}/{} cases ({:.1}%)",
        hypothesis_confirmations,
        all_results.len(),
        (hypothesis_confirmations as f64 / all_results.len() as f64) * 100.0
    );

    if hypothesis_confirmations == all_results.len() {
        println!("✅ Hypothesis FULLY CONFIRMED: Φ_hybrid > Φ_quantum > Φ_biological");
    } else if hypothesis_confirmations > all_results.len() / 2 {
        println!("⚠️  Hypothesis PARTIALLY CONFIRMED");
    } else {
        println!("❌ Hypothesis REJECTED");
    }

    // Save results to JSON
    let json_results = serde_json::to_string_pretty(&all_results)?;
    std::fs::write("consciousness_substrates_results.json", json_results)?;
    println!("\n📁 Results saved to: consciousness_substrates_results.json");

    println!("\n✅ Cross-substrate comparison complete!");

    Ok(())
}
