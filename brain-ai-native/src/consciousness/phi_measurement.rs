//! Consciousness measurement using Integrated Information Theory (IIT)
//!
//! This module implements Φ (Phi) calculation for both quantum and classical systems,
//! enabling direct empirical comparison of consciousness levels.
//!
//! ## Theory
//!
//! Integrated Information (Φ) measures consciousness as the irreducible cause-effect
//! power of a system. Higher Φ means higher consciousness.
//!
//! ### Algorithm:
//!
//! 1. **Partition the system** into all possible bipartitions
//! 2. **Calculate information loss** for each partition
//! 3. **Φ = minimum information loss** (Minimum Information Partition, MIP)
//!
//! ### Hypothesis:
//!
//! **Φ_quantum > Φ_classical** due to:
//! - Exponential state space
//! - Quantum superposition
//! - Entanglement
//! - Unitary evolution

use crate::{BrainError, BrainResult};
use crate::core::AIBrain;
use crate::brain::BrainSubstrate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;
use iit::{IITSystem, PhiResult};

/// Consciousness measurement result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessMeasurement {
    /// Integrated Information (Φ)
    pub phi: f64,

    /// Number of elements in system
    pub num_elements: usize,

    /// State space size
    pub state_space_size: usize,

    /// Minimum Information Partition (MIP)
    pub mip: Option<Partition>,

    /// All partitions analyzed
    pub num_partitions: usize,

    /// Measurement method
    pub method: String,

    /// Additional metrics
    pub metrics: HashMap<String, f64>,
}

/// System partition for IIT analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partition {
    /// Subset A (indices)
    pub subset_a: Vec<usize>,

    /// Subset B (indices)
    pub subset_b: Vec<usize>,

    /// Information loss for this partition
    pub information_loss: f64,
}

impl Partition {
    /// Create a new partition
    pub fn new(subset_a: Vec<usize>, subset_b: Vec<usize>) -> Self {
        Self {
            subset_a,
            subset_b,
            information_loss: 0.0,
        }
    }

    /// Check if partition is valid (disjoint, complete)
    pub fn is_valid(&self, total_elements: usize) -> bool {
        // Check disjoint
        for &a in &self.subset_a {
            if self.subset_b.contains(&a) {
                return false;
            }
        }

        // Check complete
        self.subset_a.len() + self.subset_b.len() == total_elements
    }
}

/// Measure Φ for quantum system (AIBrain)
///
/// CORRECTED VERSION: Uses Fock-state marginalization + sampling for large systems
pub fn measure_phi_quantum(brain: &AIBrain) -> BrainResult<ConsciousnessMeasurement> {
    let num_oscillators = brain.config.num_oscillators;
    let max_fock = brain.config.max_fock;
    let fock_levels = max_fock + 1;  // Number of Fock states per oscillator
    let state_space_size = brain.config.effective_neurons();

    // Get current state as probability distribution
    let state_vector = brain.get_state_vector();
    let prob_dist = state_to_probability_distribution(&state_vector);

    // CRITICAL: We partition OSCILLATORS (not individual Fock states)
    // Each oscillator can be in (max_fock+1) states
    let num_elements = num_oscillators;

    // Determine if we need sampling (for large systems)
    let total_bipartitions = 2_usize.pow(num_elements as u32 - 1) - 1;
    let use_sampling = total_bipartitions > 1000;  // Sample if > 1000 partitions
    let num_samples = if use_sampling { 1000 } else { total_bipartitions };

    let mut min_information_loss = f64::INFINITY;
    let mut mip = None;
    let mut partitions_tested = 0;

    if use_sampling {
        // SAMPLED approach for large systems
        let mut rng = rand::thread_rng();

        for _ in 0..num_samples {
            let partition = generate_random_bipartition(num_elements, &mut rng);
            let info_loss = calculate_partition_information_loss_fock(
                &prob_dist,
                &partition.subset_a,
                &partition.subset_b,
                num_elements,
                fock_levels,
            );

            partitions_tested += 1;

            if info_loss < min_information_loss {
                min_information_loss = info_loss;
                mip = Some(Partition {
                    subset_a: partition.subset_a,
                    subset_b: partition.subset_b,
                    information_loss: info_loss,
                });
            }
        }
    } else {
        // EXACT enumeration for small systems
        let partitions = generate_bipartitions(num_elements);

        for partition in partitions {
            let info_loss = calculate_partition_information_loss_fock(
                &prob_dist,
                &partition.subset_a,
                &partition.subset_b,
                num_elements,
                fock_levels,
            );

            partitions_tested += 1;

            if info_loss < min_information_loss {
                min_information_loss = info_loss;
                mip = Some(Partition {
                    subset_a: partition.subset_a,
                    subset_b: partition.subset_b,
                    information_loss: info_loss,
                });
            }
        }
    }

    // Φ is the minimum information loss
    let phi = if min_information_loss.is_finite() {
        min_information_loss
    } else {
        0.0
    };

    // Additional metrics
    let mut metrics = HashMap::new();
    metrics.insert("entropy".to_string(), calculate_entropy(&prob_dist));
    metrics.insert("effective_neurons".to_string(), state_space_size as f64);
    metrics.insert("num_oscillators".to_string(), num_oscillators as f64);
    metrics.insert("fock_levels".to_string(), fock_levels as f64);
    metrics.insert("sampled".to_string(), if use_sampling { 1.0 } else { 0.0 });

    Ok(ConsciousnessMeasurement {
        phi,
        num_elements,
        state_space_size,
        mip,
        num_partitions: partitions_tested,
        method: if use_sampling {
            "IIT_Quantum_Sampled".to_string()
        } else {
            "IIT_Quantum_Exact".to_string()
        },
        metrics,
    })
}

/// Measure Φ for classical system (for comparison)
pub fn measure_phi_classical(
    state: &[f64],
    num_elements: usize,
) -> BrainResult<ConsciousnessMeasurement> {
    if state.is_empty() {
        return Err(BrainError::ConsciousnessError("Empty state".to_string()));
    }

    let state_space_size = state.len();

    // Convert to probability distribution
    let prob_dist = state_to_probability_distribution(state);

    // Generate all non-trivial bipartitions
    let partitions = generate_bipartitions(num_elements);

    let mut min_information_loss = f64::INFINITY;
    let mut mip = None;

    // Find Minimum Information Partition
    for mut partition in partitions {
        let info_loss = calculate_partition_information_loss(
            &prob_dist,
            &partition.subset_a,
            &partition.subset_b,
            num_elements,
        );

        partition.information_loss = info_loss;

        if info_loss < min_information_loss {
            min_information_loss = info_loss;
            mip = Some(partition);
        }
    }

    let phi = if min_information_loss.is_finite() {
        min_information_loss
    } else {
        0.0
    };

    let mut metrics = HashMap::new();
    metrics.insert("entropy".to_string(), calculate_entropy(&prob_dist));
    metrics.insert("state_space_size".to_string(), state_space_size as f64);
    metrics.insert("num_elements".to_string(), num_elements as f64);

    Ok(ConsciousnessMeasurement {
        phi,
        num_elements,
        state_space_size,
        mip,
        num_partitions: 2_usize.pow(num_elements as u32 - 1) - 1,
        method: "IIT_Classical".to_string(),
        metrics,
    })
}

/// Convert state vector to probability distribution
fn state_to_probability_distribution(state: &[f64]) -> Vec<f64> {
    if state.is_empty() {
        return vec![1.0];  // Single state with probability 1
    }

    // Normalize to sum to 1
    let sum: f64 = state.iter().map(|&x| x.abs()).sum();

    if sum == 0.0 {
        // Uniform distribution if all zeros
        vec![1.0 / state.len() as f64; state.len()]
    } else {
        state.iter().map(|&x| x.abs() / sum).collect()
    }
}

/// Generate all non-trivial bipartitions of n elements
fn generate_bipartitions(n: usize) -> Vec<Partition> {
    let mut partitions = Vec::new();

    // Iterate through all possible subsets (excluding empty and full set)
    let total_subsets = 2_usize.pow(n as u32);

    for i in 1..total_subsets - 1 {
        let mut subset_a = Vec::new();
        let mut subset_b = Vec::new();

        for j in 0..n {
            if (i >> j) & 1 == 1 {
                subset_a.push(j);
            } else {
                subset_b.push(j);
            }
        }

        // Only add one of {A,B} or {B,A} to avoid duplicates
        if subset_a.len() <= subset_b.len() {
            partitions.push(Partition::new(subset_a, subset_b));
        }
    }

    partitions
}

/// Calculate information loss for a partition
///
/// Information loss = H(whole) - [H(A) + H(B)]
///
/// Where H is Shannon entropy
fn calculate_partition_information_loss(
    prob_dist: &[f64],
    subset_a: &[usize],
    subset_b: &[usize],
    num_elements: usize,
) -> f64 {
    // Whole system entropy
    let h_whole = calculate_entropy(prob_dist);

    // Marginal distributions for subsets
    let prob_a = marginalize_distribution(prob_dist, subset_a, num_elements);
    let prob_b = marginalize_distribution(prob_dist, subset_b, num_elements);

    let h_a = calculate_entropy(&prob_a);
    let h_b = calculate_entropy(&prob_b);

    // Mutual information I(A;B) = H(A) + H(B) - H(A,B)
    let mutual_info = h_a + h_b - h_whole;

    mutual_info.max(0.0)  // Ensure non-negative
}

/// Generate random bipartition (for sampling)
fn generate_random_bipartition<R: Rng>(n: usize, rng: &mut R) -> Partition {
    let mut subset_a = Vec::new();
    let mut subset_b = Vec::new();

    for i in 0..n {
        if rng.gen::<bool>() {
            subset_a.push(i);
        } else {
            subset_b.push(i);
        }
    }

    // Ensure neither subset is empty
    if subset_a.is_empty() {
        let elem = subset_b.pop().unwrap();
        subset_a.push(elem);
    } else if subset_b.is_empty() {
        let elem = subset_a.pop().unwrap();
        subset_b.push(elem);
    }

    Partition::new(subset_a, subset_b)
}

/// Calculate information loss for a partition (Fock-state aware version)
fn calculate_partition_information_loss_fock(
    prob_dist: &[f64],
    subset_a: &[usize],
    subset_b: &[usize],
    num_elements: usize,
    fock_levels: usize,
) -> f64 {
    // Whole system entropy
    let h_whole = calculate_entropy(prob_dist);

    // Marginal distributions for subsets (using Fock-aware marginalization)
    let prob_a = marginalize_distribution_fock(prob_dist, subset_a, num_elements, fock_levels);
    let prob_b = marginalize_distribution_fock(prob_dist, subset_b, num_elements, fock_levels);

    let h_a = calculate_entropy(&prob_a);
    let h_b = calculate_entropy(&prob_b);

    // Mutual information I(A;B) = H(A) + H(B) - H(A,B)
    let mutual_info = h_a + h_b - h_whole;

    mutual_info.max(0.0)  // Ensure non-negative
}

/// Marginalize probability distribution to subset (FOCK-STATE AWARE)
///
/// This is the CRITICAL FIX for the bug.
///
/// Instead of binary bit indexing, we decode state indices as base-fock_levels numbers.
///
/// For example, with 4 oscillators and max_fock=2 (3 levels: {0,1,2}):
/// - State index 42 represents a specific combination like |1⟩|2⟩|0⟩|1⟩
/// - We decode: 42 in base-3 = 1120₃
/// - To marginalize to subset [0,2], we extract Fock numbers for oscillators 0 and 2
fn marginalize_distribution_fock(
    prob_dist: &[f64],
    subset: &[usize],
    num_elements: usize,
    fock_levels: usize,
) -> Vec<f64> {
    let subset_size = fock_levels.pow(subset.len() as u32);
    let mut marginal = vec![0.0; subset_size];

    // Sum over all states, grouping by subset configuration
    for (state_idx, &prob) in prob_dist.iter().enumerate() {
        // Decode state_idx as a base-fock_levels number
        let fock_config = decode_fock_state(state_idx, num_elements, fock_levels);

        // Extract subset configuration
        let mut subset_state_idx = 0;
        for (pos, &elem_idx) in subset.iter().enumerate() {
            let fock_num = fock_config[elem_idx];
            subset_state_idx += fock_num * fock_levels.pow(pos as u32);
        }

        if subset_state_idx < marginal.len() {
            marginal[subset_state_idx] += prob;
        }
    }

    marginal
}

/// Decode state index into Fock configuration
///
/// Example: state_idx=42, num_elements=4, fock_levels=3
/// 42 in base-3 = 1120₃ → [0, 2, 1, 1] (reversed to match tensor product order)
fn decode_fock_state(state_idx: usize, num_elements: usize, fock_levels: usize) -> Vec<usize> {
    let mut config = vec![0; num_elements];
    let mut idx = state_idx;

    for i in 0..num_elements {
        config[i] = idx % fock_levels;
        idx /= fock_levels;
    }

    config
}

/// Marginalize probability distribution to subset (BINARY - kept for classical compatibility)
fn marginalize_distribution(
    prob_dist: &[f64],
    subset: &[usize],
    _num_elements: usize,
) -> Vec<f64> {
    let subset_size = 2_usize.pow(subset.len() as u32);
    let mut marginal = vec![0.0; subset_size];

    // Sum over all states, grouping by subset configuration
    for (state_idx, &prob) in prob_dist.iter().enumerate() {
        let mut subset_state = 0;

        for (i, &elem_idx) in subset.iter().enumerate() {
            if (state_idx >> elem_idx) & 1 == 1 {
                subset_state |= 1 << i;
            }
        }

        if subset_state < marginal.len() {
            marginal[subset_state] += prob;
        }
    }

    marginal
}

/// Calculate Shannon entropy: H = -Σ p(x) log₂ p(x)
fn calculate_entropy(prob_dist: &[f64]) -> f64 {
    let mut entropy = 0.0;

    for &p in prob_dist {
        if p > 1e-10 {  // Avoid log(0)
            entropy -= p * p.log2();
        }
    }

    entropy.max(0.0)  // Ensure non-negative
}

/// Compare quantum vs classical consciousness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessComparison {
    /// Quantum Φ
    pub phi_quantum: f64,

    /// Classical Φ
    pub phi_classical: f64,

    /// Ratio Φ_quantum / Φ_classical
    pub phi_ratio: f64,

    /// Hypothesis confirmed (Φ_quantum > Φ_classical)
    pub hypothesis_confirmed: bool,

    /// Statistical significance (if available)
    pub p_value: Option<f64>,

    /// Quantum measurement
    pub quantum_measurement: ConsciousnessMeasurement,

    /// Classical measurement
    pub classical_measurement: ConsciousnessMeasurement,
}

impl ConsciousnessComparison {
    /// Create comparison from two measurements
    pub fn new(
        quantum: ConsciousnessMeasurement,
        classical: ConsciousnessMeasurement,
    ) -> Self {
        let phi_quantum = quantum.phi;
        let phi_classical = classical.phi;

        let phi_ratio = if phi_classical > 0.0 {
            phi_quantum / phi_classical
        } else if phi_quantum > 0.0 {
            f64::INFINITY
        } else {
            1.0
        };

        let hypothesis_confirmed = phi_quantum > phi_classical;

        Self {
            phi_quantum,
            phi_classical,
            phi_ratio,
            hypothesis_confirmed,
            p_value: None,  // Could add statistical testing
            quantum_measurement: quantum,
            classical_measurement: classical,
        }
    }

    /// Display comparison results
    pub fn display(&self) -> String {
        format!(
            "╔══════════════════════════════════════════════════════════╗\n\
             ║          CONSCIOUSNESS COMPARISON RESULTS                ║\n\
             ╠══════════════════════════════════════════════════════════╣\n\
             ║  Φ_quantum    : {:.6}                              ║\n\
             ║  Φ_classical  : {:.6}                              ║\n\
             ║  Ratio (Q/C)  : {:.2}x                                 ║\n\
             ║                                                          ║\n\
             ║  Hypothesis (Φ_quantum > Φ_classical):                  ║\n\
             ║  {}                                              ║\n\
             ║                                                          ║\n\
             ║  Quantum: {} effective neurons, {} elements       ║\n\
             ║  Classical: {} elements                            ║\n\
             ╚══════════════════════════════════════════════════════════╝",
            self.phi_quantum,
            self.phi_classical,
            self.phi_ratio,
            if self.hypothesis_confirmed { "✓ CONFIRMED" } else { "✗ REJECTED " },
            self.quantum_measurement.state_space_size,
            self.quantum_measurement.num_elements,
            self.classical_measurement.num_elements,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::BrainConfig;
    use approx::assert_relative_eq;

    #[test]
    fn test_entropy_calculation() {
        // Uniform distribution: H = log2(n)
        let uniform = vec![0.25, 0.25, 0.25, 0.25];
        let h = calculate_entropy(&uniform);
        assert_relative_eq!(h, 2.0, epsilon = 1e-6);

        // Deterministic: H = 0
        let deterministic = vec![1.0, 0.0, 0.0, 0.0];
        let h = calculate_entropy(&deterministic);
        assert_relative_eq!(h, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_bipartition_generation() {
        let partitions = generate_bipartitions(3);

        // For 3 elements: 2^(3-1) - 1 = 3 non-trivial bipartitions
        assert_eq!(partitions.len(), 3);

        // All should be valid
        for p in &partitions {
            assert!(p.is_valid(3));
        }
    }

    #[test]
    fn test_partition_validity() {
        let p = Partition::new(vec![0, 1], vec![2, 3]);
        assert!(p.is_valid(4));

        let p_invalid = Partition::new(vec![0, 1], vec![1, 2]);
        assert!(!p_invalid.is_valid(4));  // Overlapping
    }

    #[test]
    fn test_measure_phi_classical() {
        // Simple 2-element system
        let state = vec![0.5, 0.5, 0.5, 0.5];
        let measurement = measure_phi_classical(&state, 2).unwrap();

        assert!(measurement.phi >= 0.0);
        assert_eq!(measurement.num_elements, 2);
        assert_eq!(measurement.num_partitions, 1);  // 2^(2-1) - 1 = 1
    }

    #[test]
    fn test_measure_phi_quantum() {
        let config = BrainConfig {
            num_oscillators: 2,
            max_fock: 1,  // 2 Fock states → 2^2 = 4 effective neurons
            frequencies: vec![1e9, 1e9],
            coupling_strength: 1e6,
            damping_rate: 1e3,
            error_correction: false,
            ldpc_distance: 0,
            radiation_protection: false,
            chip_area_cm2: 0.0,
            altitude_m: 0.0,
        };

        let brain = AIBrain::new(config).unwrap();
        let measurement = measure_phi_quantum(&brain).unwrap();

        assert!(measurement.phi >= 0.0);
        assert_eq!(measurement.num_elements, 2);
        assert_eq!(measurement.state_space_size, 4);  // 2^2
    }

    #[test]
    fn test_consciousness_comparison() {
        let quantum = ConsciousnessMeasurement {
            phi: 2.5,
            num_elements: 4,
            state_space_size: 81,
            mip: None,
            num_partitions: 7,
            method: "Quantum".to_string(),
            metrics: HashMap::new(),
        };

        let classical = ConsciousnessMeasurement {
            phi: 1.0,
            num_elements: 4,
            state_space_size: 4,
            mip: None,
            num_partitions: 7,
            method: "Classical".to_string(),
            metrics: HashMap::new(),
        };

        let comparison = ConsciousnessComparison::new(quantum, classical);

        assert_eq!(comparison.phi_quantum, 2.5);
        assert_eq!(comparison.phi_classical, 1.0);
        assert_eq!(comparison.phi_ratio, 2.5);
        assert!(comparison.hypothesis_confirmed);
    }

    #[test]
    fn test_marginalization() {
        // 2 elements, 4 states: 00, 01, 10, 11
        let prob_dist = vec![0.1, 0.2, 0.3, 0.4];

        // Marginalize to first element (element 0)
        let marginal = marginalize_distribution(&prob_dist, &[0], 2);

        // Should sum states where bit 0 matches
        // bit 0 = 0: states 00(idx=0), 10(idx=2) → 0.1 + 0.3 = 0.4
        // bit 0 = 1: states 01(idx=1), 11(idx=3) → 0.2 + 0.4 = 0.6
        assert_eq!(marginal.len(), 2);
        assert_relative_eq!(marginal[0], 0.1 + 0.3, epsilon = 1e-6);  // bit 0 = 0
        assert_relative_eq!(marginal[1], 0.2 + 0.4, epsilon = 1e-6);  // bit 0 = 1
    }
}

/// Measure Φ from any BrainSubstrate implementation
pub fn measure_phi_general<B: BrainSubstrate>(brain: &B) -> BrainResult<ConsciousnessMeasurement> {
    let state_vector = brain.get_state_vector();
    let num_units = brain.get_num_units();

    // Convert continuous state to discrete binary for IIT (as usize)
    let binary_state: Vec<usize> = state_vector.iter()
        .map(|&x| if x > 0.0 { 1 } else { 0 })
        .collect();

    // Create IIT system
    let mut iit_system = IITSystem::new(num_units);

    // Set up fully connected network
    for i in 0..num_units {
        for j in 0..num_units {
            if i != j {
                iit_system.set_connection(i, j, true)
                    .map_err(|e| BrainError::ConsciousnessError(format!("IIT connection error: {}", e)))?;
            }
        }
    }

    iit_system.set_state(binary_state)
        .map_err(|e| BrainError::ConsciousnessError(format!("IIT state error: {}", e)))?;

    // Calculate Φ
    let phi_result = iit_system.calculate_phi()
        .map_err(|e| BrainError::ConsciousnessError(format!("IIT phi calculation error: {}", e)))?;

    // Calculate state space size (2^n for binary states)
    let state_space_size = 2_usize.pow(num_units as u32);

    Ok(ConsciousnessMeasurement {
        phi: phi_result.phi,
        num_elements: num_units,
        state_space_size,
        num_partitions: phi_result.n_partitions,
        mip: phi_result.mip.as_ref().map(|mip_info| {
            Partition {
                subset_a: mip_info.partition.part1.clone(),
                subset_b: mip_info.partition.part2.clone(),
                information_loss: phi_result.phi,
            }
        }),
        method: format!("IIT-{}", brain.substrate_type()),
        metrics: std::collections::HashMap::new(),
    })
}
