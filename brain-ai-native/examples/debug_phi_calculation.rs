//! Debug Φ calculation to understand why we get Φ=0

use brain_ai_native::prelude::*;
use brain_ai_native::{BrainResult, BrainError};

fn main() -> BrainResult<()> {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║              DEBUG Φ CALCULATION                                 ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // Create brain with 4 oscillators, max_fock=2
    let config = BrainConfig {
        num_oscillators: 4,
        max_fock: 2,  // Should give 3^4 = 81 neurons
        frequencies: vec![1e9; 4],
        coupling_strength: 1e7,
        damping_rate: 1e4,
        error_correction: false,
        ldpc_distance: 0,
        radiation_protection: false,
        chip_area_cm2: 0.0,
        altitude_m: 0.0,
    };

    let mut brain = AIBrain::new(config)?;

    println!("Brain Configuration:");
    println!("  Oscillators: {}", brain.config.num_oscillators);
    println!("  Max Fock: {}", brain.config.max_fock);
    println!("  Effective neurons: {}", brain.config.effective_neurons());
    println!();

    // Add some excitation
    let input = vec![0.5, 0.6, 0.7, 0.8];
    brain.set_input(&input)?;

    // Evolve briefly
    for _ in 0..100 {
        brain.evolve(1e-10)?;
    }

    // Get state vector
    let state_vector = brain.get_state_vector();
    println!("State Vector:");
    println!("  Length: {}", state_vector.len());
    println!("  Non-zero elements: {}", state_vector.iter().filter(|&&x| x.abs() > 1e-10).count());
    println!("  First 10 elements: {:?}", &state_vector[..10.min(state_vector.len())]);
    println!();

    // Try measuring Φ
    let measurement = measure_phi_quantum(&brain)?;

    println!("Φ Measurement:");
    println!("  Φ value: {}", measurement.phi);
    println!("  Num elements: {}", measurement.num_elements);  // <- KEY!
    println!("  State space size: {}", measurement.state_space_size);
    println!("  Num partitions analyzed: {}", measurement.num_partitions);
    println!("  Method: {}", measurement.method);
    println!();

    if let Some(ref mip) = measurement.mip {
        println!("Minimum Information Partition:");
        println!("  Subset A: {:?}", mip.subset_a);
        println!("  Subset B: {:?}", mip.subset_b);
        println!("  Information loss: {}", mip.information_loss);
    }

    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                          DIAGNOSIS                               ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    println!("PROBLEM IDENTIFIED:");
    println!("  IIT is analyzing {} elements (oscillators)", measurement.num_elements);
    println!("  But the system has {} effective states", measurement.state_space_size);
    println!();
    println!("The Φ calculation treats each OSCILLATOR as an element,");
    println!("not each NEURON (Fock state combination).");
    println!();
    println!("With 4 oscillators in nearly-identical states,");
    println!("there's minimal information to partition.");
    println!();
    println!("SOLUTION:");
    println!("  Need to calculate Φ on the FULL state space (81 states)");
    println!("  not just the 4 oscillator coordinates.");

    Ok(())
}
