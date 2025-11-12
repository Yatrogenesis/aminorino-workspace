//! Debug TPM Construction and Φ Calculation
//!
//! Shows EXACTLY how:
//! 1. Binary state is constructed from quantum state
//! 2. TPM is built (fully-connected network)
//! 3. Φ is calculated (all partitions)
//! 4. Why Φ=0 (diagnostic)

use iit::{IITSystem, PhiResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║   TPM CONSTRUCTION AND Φ CALCULATION DEBUG                       ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    // Test with actual binary states from dynamic monitoring
    let test_cases = vec![
        ("Checkpoint 1", vec![0, 0, 1, 0, 0, 1]),
        ("Checkpoint 2", vec![0, 0, 1, 0, 0, 1]),  // Same state
        ("Checkpoint 3", vec![1, 0, 0, 1, 0, 0]),  // Different
        ("Checkpoint 4", vec![0, 0, 0, 0, 1, 1]),
        ("Checkpoint 5", vec![1, 1, 0, 0, 0, 0]),
    ];

    for (name, binary_state) in &test_cases {
        println!("\n{}", "━".repeat(70));
        println!("{}: binary_state = {:?}", name, binary_state);
        println!("{}", "━".repeat(70));

        let n = binary_state.len();

        // Create IIT system
        let mut iit_system = IITSystem::new(n);

        // STEP 1: Set up fully-connected network
        println!("\n[STEP 1] Network topology (fully-connected):");
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    iit_system.set_connection(i, j, true)?;
                    print!("  {}->{} ✓", i, j);
                }
            }
            println!();
        }

        // STEP 2: Set current state
        println!("\n[STEP 2] Setting state:");
        println!("  State: {:?}", binary_state);
        iit_system.set_state(binary_state.clone())?;

        // STEP 3: Calculate Φ
        println!("\n[STEP 3] Calculating Φ...");
        let phi_result: PhiResult = iit_system.calculate_phi()?;

        println!("\n[RESULTS]:");
        println!("  Φ = {:.9} bits", phi_result.phi);
        println!("  Partitions tested: {}", phi_result.n_partitions);

        if let Some(mip) = &phi_result.mip {
            println!("  MIP found:");
            println!("    Partition: {:?}", mip.partition);
            println!("    MIP Φ: {:.9} bits", mip.phi);
            println!("    Cuts: {}", mip.cuts);
        } else {
            println!("  MIP: None");
        }

        // STEP 4: Diagnostic - WHY Φ=0?
        println!("\n[DIAGNOSTIC]:");
        if phi_result.phi == 0.0 {
            println!("  ⚠️  Φ=0 detected!");
            println!("  Possible causes:");
            println!("    1. State is a product state (completely separable)");
            println!("    2. All partitions have same information loss = 0");
            println!("    3. Network is not actually integrated");
            println!("    4. TPM construction assumes deterministic dynamics");

            // Check if state is all 0s or all 1s
            let all_zero = binary_state.iter().all(|&x| x == 0);
            let all_one = binary_state.iter().all(|&x| x == 1);
            if all_zero || all_one {
                println!("\n  → State is homogeneous ({})!", if all_zero { "all 0s" } else { "all 1s" });
                println!("     This gives Φ=0 trivially (no information)");
            }

            // Check hamming weight
            let hamming_weight: usize = binary_state.iter().sum();
            println!("\n  Hamming weight: {}/{} ({:.1}%)",
                     hamming_weight, n, 100.0 * hamming_weight as f64 / n as f64);

            if hamming_weight == 0 || hamming_weight == n {
                println!("  → Extreme hamming weight (all same)");
            } else if hamming_weight == n/2 {
                println!("  → Balanced hamming weight (good for integration)");
            }
        } else {
            println!("  ✓ Φ > 0 (integration detected)");
        }

        println!();
    }

    // CRITICAL TEST: Manual state with known Φ>0
    println!("\n{}", "═".repeat(70));
    println!("CRITICAL TEST: Known high-Φ state (XOR gate)");
    println!("{}", "═".repeat(70));

    // XOR gate: state [1,0] should have Φ ≈ 0.189 bits (from IIT literature)
    let xor_state = vec![1, 0];
    let mut xor_system = IITSystem::new(2);

    // Set XOR connections (both inputs affect each other)
    xor_system.set_connection(0, 1, true)?;
    xor_system.set_connection(1, 0, true)?;
    xor_system.set_state(xor_state.clone())?;

    let xor_phi = xor_system.calculate_phi()?;

    println!("XOR state: {:?}", xor_state);
    println!("Expected Φ: ~0.189 bits (from literature)");
    println!("Actual Φ:   {:.9} bits", xor_phi.phi);

    if xor_phi.phi > 0.0 {
        println!("✓ IIT library working correctly!");
    } else {
        println!("⚠️  CRITICAL: Even XOR gives Φ=0!");
        println!("   This indicates TPM construction issue, not quantum state issue");
    }

    Ok(())
}
