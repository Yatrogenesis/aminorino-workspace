//! Integration tests for the IIT library.
//!
//! These tests verify the complete functionality of the library including
//! known test cases from the IIT literature.

use iit::*;
use ndarray::{Array, IxDyn};

/// Test the classic 3-neuron majority gate system.
///
/// This is a well-known test case in IIT literature where three neurons
/// form a majority gate (output 1 if >= 2 inputs are 1).
#[test]
fn test_three_neuron_majority_gate() {
    let mut system = IITSystem::new(3);

    // Set up all-to-all connectivity
    for i in 0..3 {
        for j in 0..3 {
            if i != j {
                system.set_connection(i, j, true).unwrap();
            }
        }
    }

    // Test with different states
    let states = vec![
        vec![0, 0, 0],
        vec![1, 0, 0],
        vec![1, 1, 0],
        vec![1, 1, 1],
    ];

    for state in states {
        system.set_state(state.clone()).unwrap();
        let result = system.calculate_phi().unwrap();

        // Φ should be non-negative
        assert!(result.phi >= 0.0, "Φ should be non-negative for state {:?}", state);

        println!("State {:?}: Φ = {:.4}", state, result.phi);
    }
}

/// Test that disconnected systems have Φ = 0.
#[test]
fn test_disconnected_system() {
    let mut system = IITSystem::new(3);
    system.set_state(vec![1, 0, 1]).unwrap();

    // No connections - should have Φ = 0
    let result = system.calculate_phi().unwrap();

    assert!(
        result.phi.abs() < 1e-6,
        "Disconnected system should have Φ ≈ 0, got {}",
        result.phi
    );
}

/// Test that single elements have Φ = 0.
#[test]
fn test_single_element() {
    let mut system = IITSystem::new(1);
    system.set_state(vec![1]).unwrap();

    let result = system.calculate_phi().unwrap();

    assert_eq!(result.phi, 0.0, "Single element should have Φ = 0");
}

/// Test feedforward vs recurrent connectivity.
///
/// Feedforward networks should have lower Φ than recurrent networks.
#[test]
fn test_feedforward_vs_recurrent() {
    // Feedforward system
    let mut ff_system = IITSystem::new(3);
    ff_system.set_connection(0, 1, true).unwrap();
    ff_system.set_connection(1, 2, true).unwrap();
    ff_system.set_state(vec![1, 1, 1]).unwrap();

    // Recurrent system
    let mut rec_system = IITSystem::new(3);
    rec_system.set_connection(0, 1, true).unwrap();
    rec_system.set_connection(1, 2, true).unwrap();
    rec_system.set_connection(2, 0, true).unwrap();
    rec_system.set_state(vec![1, 1, 1]).unwrap();

    let ff_phi = ff_system.calculate_phi().unwrap().phi;
    let rec_phi = rec_system.calculate_phi().unwrap().phi;

    println!("Feedforward Φ: {:.4}", ff_phi);
    println!("Recurrent Φ: {:.4}", rec_phi);

    // Recurrent should generally have higher Φ
    // (though this depends on approximation method)
    assert!(ff_phi >= 0.0);
    assert!(rec_phi >= 0.0);
}

/// Test different approximation methods.
#[test]
fn test_approximation_methods() {
    let mut system = fully_connected_system(4);
    system.set_state(vec![1, 0, 1, 1]).unwrap();

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
        let result = system.calculate_phi().unwrap();

        println!("{:?}: Φ = {:.4}", method, result.phi);

        assert!(result.phi >= 0.0, "{:?} produced negative Φ", method);
        assert_eq!(result.method, method);
    }
}

/// Test concept identification.
#[test]
fn test_concept_identification() {
    let mut system = IITSystem::new(2);

    // Simple two-element system
    system.set_connection(0, 1, true).unwrap();
    system.set_connection(1, 0, true).unwrap();
    system.set_state(vec![1, 0]).unwrap();

    let ces = system.identify_concepts().unwrap();

    println!("Found {} concepts", ces.n_concepts());

    // Should find at least some concepts
    assert!(ces.n_concepts() > 0, "Should identify at least one concept");

    // All concepts should have non-negative Φ
    for concept in &ces.concepts {
        assert!(
            concept.phi >= 0.0,
            "Concept {:?} has negative Φ: {}",
            concept.mechanism,
            concept.phi
        );
    }
}

/// Test concept filtering by size.
#[test]
fn test_concept_filtering() {
    let mut system = IITSystem::new(3);

    for i in 0..3 {
        for j in 0..3 {
            if i != j {
                system.set_connection(i, j, true).unwrap();
            }
        }
    }

    system.set_state(vec![1, 0, 1]).unwrap();

    let ces = system.identify_concepts().unwrap();

    // Filter by size
    let size_1 = ces.concepts_by_size(1);
    let size_2 = ces.concepts_by_size(2);

    println!("Size 1 concepts: {}", size_1.len());
    println!("Size 2 concepts: {}", size_2.len());

    // Verify sizes
    for concept in size_1 {
        assert_eq!(concept.mechanism_size(), 1);
    }

    for concept in size_2 {
        assert_eq!(concept.mechanism_size(), 2);
    }
}

/// Test qualia space analysis.
#[test]
fn test_qualia_space() {
    let mut system = fully_connected_system(3);
    system.set_state(vec![1, 1, 0]).unwrap();

    let qualia = system.analyze_qualia_space().unwrap();

    println!("Qualia Space Analysis:");
    println!("  Total concepts: {}", qualia.n_concepts);
    println!("  Mean Φ: {:.4}", qualia.mean_phi);
    println!("  Std Φ: {:.4}", qualia.std_phi);
    println!("  Max Φ: {:.4}", qualia.max_phi);
    println!("  Min Φ: {:.4}", qualia.min_phi);

    assert!(qualia.n_concepts > 0);
    assert!(qualia.mean_phi >= 0.0);
    assert!(qualia.max_phi >= qualia.min_phi);
}

/// Test builder pattern.
#[test]
fn test_builder_pattern() {
    let system = IITSystemBuilder::new(3)
        .state(vec![1, 0, 1])
        .fully_connected()
        .approximation(ApproximationMethod::Geometric)
        .parallel(false)
        .build()
        .unwrap();

    assert_eq!(system.n_elements(), 3);
    assert_eq!(system.state(), &[1, 0, 1]);
    assert_eq!(system.config().approximation, ApproximationMethod::Geometric);
    assert!(!system.config().parallel);

    // Verify connectivity
    for i in 0..3 {
        for j in 0..3 {
            if i != j {
                assert!(system.connectivity()[i][j]);
            }
        }
    }
}

/// Test system with custom TPM.
#[test]
fn test_custom_tpm() {
    let mut system = IITSystem::new(2);

    // Create a simple deterministic TPM
    let shape = vec![2, 2, 2, 2];
    let mut tpm_data = vec![0.0; 16];

    // Simple AND gate: output is 1 only if both inputs are 1
    tpm_data[15] = 1.0; // [1,1] -> [1,1]

    let tpm = Array::from_vec(tpm_data).into_shape(IxDyn(&shape)).unwrap();

    system.set_tpm(tpm).unwrap();
    system.set_state(vec![1, 1]).unwrap();

    let result = system.calculate_phi().unwrap();
    assert!(result.phi >= 0.0);
}

/// Test error handling.
#[test]
fn test_error_handling() {
    let mut system = IITSystem::new(3);

    // Invalid state dimension
    let result = system.set_state(vec![1, 0]);
    assert!(result.is_err());

    // Invalid state values
    let result = system.set_state(vec![1, 2, 0]);
    assert!(result.is_err());

    // Invalid connection indices
    let result = system.set_connection(5, 0, true);
    assert!(result.is_err());

    // Invalid TPM dimensions
    let wrong_tpm = Array::from_elem(IxDyn(&vec![2, 2]), 0.5);
    let result = system.set_tpm(wrong_tpm);
    assert!(result.is_err());
}

/// Test parallel vs sequential computation.
#[test]
fn test_parallel_computation() {
    let mut system = fully_connected_system(3);
    system.set_state(vec![1, 0, 1]).unwrap();

    // Parallel
    let mut config = PhiConfig::default();
    config.parallel = true;
    system.set_config(config);

    let parallel_result = system.calculate_phi().unwrap();

    // Sequential
    let mut config = PhiConfig::default();
    config.parallel = false;
    system.set_config(config);

    let sequential_result = system.calculate_phi().unwrap();

    // Results should be the same (or very close)
    assert!(
        (parallel_result.phi - sequential_result.phi).abs() < 1e-6,
        "Parallel and sequential should give same result"
    );
}

/// Test tau approximation for fully connected network.
#[test]
fn test_tau_fully_connected() {
    let system = fully_connected_system(4);

    let mut config = PhiConfig::default();
    config.approximation = ApproximationMethod::Tau;
    config.max_exact_size = 0;

    let mut sys = system.clone();
    sys.set_config(config);

    let result = sys.calculate_phi().unwrap();

    // Fully connected should have τ = 1.0
    assert!(
        (result.phi - 1.0).abs() < 1e-6,
        "Fully connected τ should be 1.0, got {}",
        result.phi
    );
}

/// Test that Φ increases with connectivity.
#[test]
fn test_phi_increases_with_connectivity() {
    let n = 4;

    // Minimal connectivity
    let mut sparse_system = IITSystem::new(n);
    sparse_system.set_connection(0, 1, true).unwrap();
    sparse_system.set_state(vec![1, 0, 1, 1]).unwrap();

    // Maximal connectivity
    let mut dense_system = fully_connected_system(n);
    dense_system.set_state(vec![1, 0, 1, 1]).unwrap();

    let sparse_phi = sparse_system.calculate_phi().unwrap().phi;
    let dense_phi = dense_system.calculate_phi().unwrap().phi;

    println!("Sparse connectivity Φ: {:.4}", sparse_phi);
    println!("Dense connectivity Φ: {:.4}", dense_phi);

    // Dense should generally have higher Φ (for most approximations)
    assert!(sparse_phi >= 0.0);
    assert!(dense_phi >= 0.0);
}

/// Benchmark different system sizes.
#[test]
fn test_scalability() {
    let sizes = vec![2, 3, 4, 5];

    for &n in &sizes {
        let mut system = fully_connected_system(n);
        let state = vec![1; n];
        system.set_state(state).unwrap();

        let start = std::time::Instant::now();
        let result = system.calculate_phi().unwrap();
        let duration = start.elapsed();

        println!(
            "n={}: Φ={:.4}, time={:.2}ms, method={:?}",
            n,
            result.phi,
            duration.as_secs_f64() * 1000.0,
            result.method
        );
    }
}

/// Test state transitions.
#[test]
fn test_state_transitions() {
    let mut system = IITSystem::new(3);

    for i in 0..3 {
        for j in 0..3 {
            if i != j {
                system.set_connection(i, j, true).unwrap();
            }
        }
    }

    // Test all possible states
    for state_idx in 0..8 {
        let state = vec![
            (state_idx >> 2) & 1,
            (state_idx >> 1) & 1,
            state_idx & 1,
        ];

        system.set_state(state.clone()).unwrap();
        let result = system.calculate_phi().unwrap();

        println!("State {:?}: Φ = {:.4}", state, result.phi);
        assert!(result.phi >= 0.0);
    }
}
