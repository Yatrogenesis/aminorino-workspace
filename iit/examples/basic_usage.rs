//! Basic usage example of the IIT library.
//!
//! This example demonstrates how to create a simple neural system and calculate
//! its integrated information (Φ).

use iit::*;

fn main() -> Result<()> {
    println!("=== IIT Library Basic Usage ===\n");

    // Example 1: Simple 3-neuron system
    println!("Example 1: 3-neuron fully connected system");
    println!("-".repeat(50));

    let mut system = IITSystem::new(3);

    // Set up all-to-all connectivity
    for i in 0..3 {
        for j in 0..3 {
            if i != j {
                system.set_connection(i, j, true)?;
            }
        }
    }

    // Set system state
    system.set_state(vec![1, 0, 1])?;

    // Calculate Φ
    let result = system.calculate_phi()?;
    println!("State: {:?}", system.state());
    println!("Φ = {:.4}", result.phi);
    println!("Method: {:?}", result.method);
    println!("Computation time: {} ms", result.computation_time_ms);
    println!();

    // Example 2: Using the builder pattern
    println!("Example 2: Using builder pattern");
    println!("-".repeat(50));

    let system = IITSystemBuilder::new(4)
        .state(vec![1, 1, 0, 1])
        .fully_connected()
        .approximation(ApproximationMethod::Geometric)
        .build()?;

    let result = system.calculate_phi()?;
    println!("State: {:?}", system.state());
    println!("Φ = {:.4} ({})", result.phi, "Geometric approximation");
    println!();

    // Example 3: Different approximation methods
    println!("Example 3: Comparing approximation methods");
    println!("-".repeat(50));

    let mut system = fully_connected_system(5);
    system.set_state(vec![1, 0, 1, 1, 0])?;

    let methods = vec![
        ApproximationMethod::Geometric,
        ApproximationMethod::Spectral,
        ApproximationMethod::MeanField,
        ApproximationMethod::Tau,
    ];

    for method in methods {
        let mut config = PhiConfig::default();
        config.approximation = method;
        config.max_exact_size = 0; // Force approximation

        system.set_config(config);
        let result = system.calculate_phi()?;

        println!("{:12?}: Φ = {:.4}", method, result.phi);
    }
    println!();

    // Example 4: Concept identification
    println!("Example 4: Identifying concepts");
    println!("-".repeat(50));

    let mut system = IITSystem::new(3);
    for i in 0..3 {
        for j in 0..3 {
            if i != j {
                system.set_connection(i, j, true)?;
            }
        }
    }
    system.set_state(vec![1, 1, 0])?;

    let ces = system.identify_concepts()?;
    println!("Total concepts: {}", ces.n_concepts());
    println!("Total Φ: {:.4}", ces.phi);

    // Show top concepts
    let top_concepts = ces.core(5);
    println!("\nTop {} concepts:", top_concepts.len());
    for (i, concept) in top_concepts.iter().enumerate() {
        println!(
            "  {}: Mechanism {:?}, Φ = {:.4}",
            i + 1,
            concept.mechanism,
            concept.phi
        );
    }
    println!();

    // Example 5: Qualia space analysis
    println!("Example 5: Qualia space analysis");
    println!("-".repeat(50));

    let qualia = system.analyze_qualia_space()?;
    println!("Number of concepts: {}", qualia.n_concepts);
    println!("Mean Φ: {:.4}", qualia.mean_phi);
    println!("Std Φ: {:.4}", qualia.std_phi);
    println!("Max Φ: {:.4}", qualia.max_phi);
    println!("Min Φ: {:.4}", qualia.min_phi);

    println!("\nConcept size distribution:");
    for (size, count) in &qualia.size_distribution {
        println!("  Size {}: {} concepts", size, count);
    }
    println!();

    // Example 6: Feedforward vs recurrent
    println!("Example 6: Feedforward vs recurrent networks");
    println!("-".repeat(50));

    // Feedforward
    let ff_system = feedforward_system(4);
    let mut ff_sys = ff_system.clone();
    ff_sys.set_state(vec![1, 1, 1, 0])?;
    let ff_phi = ff_sys.calculate_phi()?.phi;

    // Recurrent
    let mut rec_system = feedforward_system(4);
    rec_system.set_connection(3, 0, true)?; // Add feedback
    rec_system.set_state(vec![1, 1, 1, 0])?;
    let rec_phi = rec_system.calculate_phi()?.phi;

    println!("Feedforward Φ: {:.4}", ff_phi);
    println!("Recurrent Φ: {:.4}", rec_phi);
    println!();

    // Example 7: State exploration
    println!("Example 7: Exploring all states of a 3-neuron system");
    println!("-".repeat(50));

    let mut system = fully_connected_system(3);

    println!("State  | Φ");
    println!("-------|-------");

    for state_idx in 0..8 {
        let state = vec![
            (state_idx >> 2) & 1,
            (state_idx >> 1) & 1,
            state_idx & 1,
        ];

        system.set_state(state.clone())?;
        let result = system.calculate_phi()?;

        println!("{:?} | {:.4}", state, result.phi);
    }

    println!("\n=== Examples completed successfully! ===");

    Ok(())
}
