//! Advanced IIT analysis example.
//!
//! This example demonstrates advanced features including concept filtering,
//! causal structure analysis, and comparison of different network architectures.

use iit::*;

fn main() -> Result<()> {
    println!("=== Advanced IIT Analysis ===\n");

    // Part 1: Analyze different network architectures
    analyze_network_architectures()?;

    // Part 2: Concept analysis with filtering
    analyze_concepts_detailed()?;

    // Part 3: State space exploration
    explore_state_space()?;

    Ok(())
}

fn analyze_network_architectures() -> Result<()> {
    println!("Part 1: Network Architecture Comparison");
    println!("=".repeat(60));

    let n = 4;
    let state = vec![1, 0, 1, 1];

    // Architecture 1: Fully connected
    let mut fully_conn = fully_connected_system(n);
    fully_conn.set_state(state.clone())?;

    // Architecture 2: Ring network
    let mut ring = IITSystem::new(n);
    for i in 0..n {
        ring.set_connection(i, (i + 1) % n, true)?;
    }
    ring.set_state(state.clone())?;

    // Architecture 3: Feedforward
    let mut feedforward = feedforward_system(n);
    feedforward.set_state(state.clone())?;

    // Architecture 4: Hub network (star topology)
    let mut hub = IITSystem::new(n);
    for i in 1..n {
        hub.set_connection(0, i, true)?;
        hub.set_connection(i, 0, true)?;
    }
    hub.set_state(state.clone())?;

    // Calculate Φ for each
    let architectures = vec![
        ("Fully Connected", &fully_conn),
        ("Ring", &ring),
        ("Feedforward", &feedforward),
        ("Hub (Star)", &hub),
    ];

    println!("\nΦ values for different architectures:");
    println!("{:<20} | {:>10} | {:>15}", "Architecture", "Φ", "Method");
    println!("{}", "-".repeat(50));

    for (name, system) in &architectures {
        let result = system.calculate_phi()?;
        println!("{:<20} | {:>10.4} | {:>15?}", name, result.phi, result.method);
    }

    println!("\nAnalysis:");
    println!("- Fully connected networks typically have highest Φ");
    println!("- Feedforward networks have lower Φ due to lack of recurrence");
    println!("- Ring networks balance connectivity and integration");
    println!("- Hub networks concentrate information flow");
    println!();

    Ok(())
}

fn analyze_concepts_detailed() -> Result<()> {
    println!("Part 2: Detailed Concept Analysis");
    println!("=".repeat(60));

    let mut system = fully_connected_system(4);
    system.set_state(vec![1, 1, 0, 1])?;

    // Identify concepts with different thresholds
    let thresholds = vec![0.0, 0.01, 0.1];

    println!("\nConcept counts at different Φ thresholds:");
    for &threshold in &thresholds {
        let mut config = ConceptConfig::default();
        config.min_phi = threshold;

        let ces = system.identify_concepts_with_config(&config)?;

        println!("  Φ >= {:.2}: {} concepts", threshold, ces.n_concepts());
    }

    // Detailed analysis of concepts
    let ces = system.identify_concepts()?;

    println!("\nConcepts by mechanism size:");
    for size in 1..=4 {
        let concepts = ces.concepts_by_size(size);
        if !concepts.is_empty() {
            println!("  Size {}: {} concepts", size, concepts.len());

            // Show top 3 for this size
            let mut sorted = concepts.clone();
            sorted.sort_by(|a, b| {
                b.phi
                    .partial_cmp(&a.phi)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            for (i, concept) in sorted.iter().take(3).enumerate() {
                println!(
                    "    {}: {:?} → Φ={:.4}",
                    i + 1,
                    concept.mechanism,
                    concept.phi
                );
            }
        }
    }

    // Qualia space statistics
    let qualia = concepts::analyze_qualia_space(&ces);

    println!("\nQualia Space Statistics:");
    println!("  Total concepts: {}", qualia.n_concepts);
    println!("  Mean Φ: {:.4} ± {:.4}", qualia.mean_phi, qualia.std_phi);
    println!("  Range: [{:.4}, {:.4}]", qualia.min_phi, qualia.max_phi);

    println!("\n  Size distribution:");
    for (size, count) in &qualia.size_distribution {
        let pct = (*count as f64 / qualia.n_concepts as f64) * 100.0;
        println!("    Size {}: {} ({:.1}%)", size, count, pct);
    }

    // Core concepts
    println!("\nCore Cause-Effect Structure (top 5 concepts):");
    let core = ces.core(5);
    for (i, concept) in core.iter().enumerate() {
        println!(
            "  {}: Mechanism {:?}, Φ={:.4}",
            i + 1,
            concept.mechanism,
            concept.phi
        );
        println!("     Cause purview: {:?}", concept.mice.cause.purview);
        println!("     Effect purview: {:?}", concept.mice.effect.purview);
    }

    println!();

    Ok(())
}

fn explore_state_space() -> Result<()> {
    println!("Part 3: State Space Exploration");
    println!("=".repeat(60));

    let n = 3;
    let mut system = fully_connected_system(n);

    println!("\nΦ across all possible states:");
    println!("{:^7} | {:>8} | {:>15}", "State", "Φ", "# Concepts");
    println!("{}", "-".repeat(35));

    let mut phi_values = Vec::new();

    for state_idx in 0..8 {
        let state = vec![
            (state_idx >> 2) & 1,
            (state_idx >> 1) & 1,
            state_idx & 1,
        ];

        system.set_state(state.clone())?;

        let phi_result = system.calculate_phi()?;

        let mut concept_config = ConceptConfig::default();
        concept_config.min_phi = 0.01; // Only significant concepts

        let ces = system.identify_concepts_with_config(&concept_config)?;

        println!(
            "{:?} | {:>8.4} | {:>15}",
            state,
            phi_result.phi,
            ces.n_concepts()
        );

        phi_values.push(phi_result.phi);
    }

    // Statistics
    let mean = phi_values.iter().sum::<f64>() / phi_values.len() as f64;
    let variance = phi_values
        .iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>()
        / phi_values.len() as f64;
    let std_dev = variance.sqrt();

    let max = phi_values
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();
    let min = phi_values
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();

    println!("\nState Space Statistics:");
    println!("  Mean Φ: {:.4} ± {:.4}", mean, std_dev);
    println!("  Range: [{:.4}, {:.4}]", min, max);

    // Find states with highest Φ
    let mut state_phi_pairs: Vec<_> = (0..8)
        .map(|i| {
            (
                vec![(i >> 2) & 1, (i >> 1) & 1, i & 1],
                phi_values[i],
            )
        })
        .collect();

    state_phi_pairs.sort_by(|a, b| {
        b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
    });

    println!("\nStates with highest Φ:");
    for (i, (state, phi)) in state_phi_pairs.iter().take(3).enumerate() {
        println!("  {}: {:?} → Φ={:.4}", i + 1, state, phi);
    }

    println!();

    Ok(())
}
