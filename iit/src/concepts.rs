//! Concept identification and cause-effect structure.
//!
//! This module implements the identification of concepts, which are the fundamental
//! units of integrated information in IIT 3.0. A concept is a mechanism with
//! irreducible cause-effect power.

use crate::causality::{find_mice, MICE};
use crate::error::{IITError, Result};
use ndarray::ArrayD;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A concept in IIT 3.0.
///
/// A concept is a mechanism with irreducible cause-effect power, characterized by
/// its maximally irreducible cause-effect (MICE).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    /// The mechanism (set of elements).
    pub mechanism: Vec<usize>,
    /// The mechanism's state.
    pub state: Vec<usize>,
    /// The MICE (maximally irreducible cause-effect).
    pub mice: MICE,
    /// Integrated information of the concept (min of cause and effect φ).
    pub phi: f64,
}

impl Concept {
    /// Create a new concept.
    pub fn new(mechanism: Vec<usize>, state: Vec<usize>, mice: MICE) -> Self {
        // Φ of concept is minimum of cause and effect φ
        let phi = mice.cause.phi.min(mice.effect.phi);

        Self {
            mechanism,
            state,
            mice,
            phi,
        }
    }

    /// Check if this concept has significant integrated information.
    pub fn is_significant(&self, threshold: f64) -> bool {
        self.phi > threshold
    }

    /// Get the mechanism size.
    pub fn mechanism_size(&self) -> usize {
        self.mechanism.len()
    }
}

/// The cause-effect structure (constellation of concepts).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CauseEffectStructure {
    /// All concepts in the structure.
    pub concepts: Vec<Concept>,
    /// The total integrated information (Φ) of the structure.
    pub phi: f64,
}

impl CauseEffectStructure {
    /// Create a new cause-effect structure.
    pub fn new(concepts: Vec<Concept>) -> Self {
        // Total Φ is sum of individual concept φ values
        // (Note: In full IIT 3.0, this would be more complex)
        let phi = concepts.iter().map(|c| c.phi).sum();

        Self { concepts, phi }
    }

    /// Get the number of concepts.
    pub fn n_concepts(&self) -> usize {
        self.concepts.len()
    }

    /// Get significant concepts above a threshold.
    pub fn significant_concepts(&self, threshold: f64) -> Vec<&Concept> {
        self.concepts
            .iter()
            .filter(|c| c.is_significant(threshold))
            .collect()
    }

    /// Get concepts by mechanism size.
    pub fn concepts_by_size(&self, size: usize) -> Vec<&Concept> {
        self.concepts
            .iter()
            .filter(|c| c.mechanism_size() == size)
            .collect()
    }

    /// Get the core cause-effect structure (maximum φ concepts).
    pub fn core(&self, max_concepts: usize) -> Vec<&Concept> {
        let mut sorted: Vec<_> = self.concepts.iter().collect();
        sorted.sort_by(|a, b| {
            b.phi
                .partial_cmp(&a.phi)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted.into_iter().take(max_concepts).collect()
    }
}

/// Configuration for concept identification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptConfig {
    /// Minimum φ threshold for a concept to be included.
    pub min_phi: f64,
    /// Maximum mechanism size to consider.
    pub max_mechanism_size: Option<usize>,
    /// Whether to use parallel computation.
    pub parallel: bool,
}

impl Default for ConceptConfig {
    fn default() -> Self {
        Self {
            min_phi: 0.0,
            max_mechanism_size: None,
            parallel: true,
        }
    }
}

/// Identify all concepts in a system.
///
/// # Arguments
///
/// * `system_state` - Current state of the system
/// * `tpm` - Transition probability matrix
/// * `connectivity` - Connectivity matrix
/// * `config` - Configuration for concept identification
///
/// # Returns
///
/// The cause-effect structure containing all concepts
pub fn identify_concepts(
    system_state: &[usize],
    tpm: &ArrayD<f64>,
    connectivity: &[Vec<bool>],
    config: &ConceptConfig,
) -> Result<CauseEffectStructure> {
    let n = system_state.len();
    let system_elements: Vec<_> = (0..n).collect();

    // Generate all possible mechanisms
    let mechanisms = generate_mechanisms(n, config.max_mechanism_size);

    // Find concepts in parallel or sequential
    let concepts: Vec<Concept> = if config.parallel {
        mechanisms
            .par_iter()
            .filter_map(|mechanism| {
                identify_concept(mechanism, system_state, &system_elements, tpm, connectivity)
                    .ok()
            })
            .filter(|c| c.phi >= config.min_phi)
            .collect()
    } else {
        mechanisms
            .iter()
            .filter_map(|mechanism| {
                identify_concept(mechanism, system_state, &system_elements, tpm, connectivity)
                    .ok()
            })
            .filter(|c| c.phi >= config.min_phi)
            .collect()
    };

    Ok(CauseEffectStructure::new(concepts))
}

/// Identify a single concept for a mechanism.
fn identify_concept(
    mechanism: &[usize],
    system_state: &[usize],
    system_elements: &[usize],
    tpm: &ArrayD<f64>,
    connectivity: &[Vec<bool>],
) -> Result<Concept> {
    if mechanism.is_empty() {
        return Err(IITError::InvalidMechanism("Empty mechanism".to_string()));
    }

    // Get mechanism state
    let state: Vec<_> = mechanism.iter().map(|&i| system_state[i]).collect();

    // Find MICE
    let mice = find_mice(mechanism, &state, system_elements, tpm, connectivity)?;

    Ok(Concept::new(mechanism.to_vec(), state, mice))
}

/// Generate all possible mechanisms up to a maximum size.
fn generate_mechanisms(n: usize, max_size: Option<usize>) -> Vec<Vec<usize>> {
    let max = max_size.unwrap_or(n);

    (1..(1 << n))
        .filter_map(|mask| {
            let mechanism: Vec<_> = (0..n).filter(|&i| (mask & (1 << i)) != 0).collect();

            if mechanism.len() <= max {
                Some(mechanism)
            } else {
                None
            }
        })
        .collect()
}

/// Compare two cause-effect structures.
///
/// Returns the distance between them (using extended EMD over concept space).
pub fn compare_structures(
    ces1: &CauseEffectStructure,
    ces2: &CauseEffectStructure,
) -> Result<f64> {
    // Simplified comparison: count common concepts
    let concepts1: HashSet<_> = ces1
        .concepts
        .iter()
        .map(|c| c.mechanism.clone())
        .collect();
    let concepts2: HashSet<_> = ces2
        .concepts
        .iter()
        .map(|c| c.mechanism.clone())
        .collect();

    let common = concepts1.intersection(&concepts2).count();
    let total = concepts1.union(&concepts2).count();

    if total == 0 {
        return Ok(0.0);
    }

    // Jaccard distance
    Ok(1.0 - (common as f64 / total as f64))
}

/// Find the concept with maximum φ.
pub fn max_concept(ces: &CauseEffectStructure) -> Option<&Concept> {
    ces.concepts
        .iter()
        .max_by(|a, b| a.phi.partial_cmp(&b.phi).unwrap_or(std::cmp::Ordering::Equal))
}

/// Find concepts containing a specific element.
pub fn concepts_containing_element(
    ces: &CauseEffectStructure,
    element: usize,
) -> Vec<&Concept> {
    ces.concepts
        .iter()
        .filter(|c| c.mechanism.contains(&element))
        .collect()
}

/// Compute the qualia space (distribution of concepts).
///
/// Returns statistics about the concept distribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualiaSpace {
    /// Total number of concepts.
    pub n_concepts: usize,
    /// Mean φ across concepts.
    pub mean_phi: f64,
    /// Standard deviation of φ.
    pub std_phi: f64,
    /// Maximum φ.
    pub max_phi: f64,
    /// Minimum φ.
    pub min_phi: f64,
    /// Distribution of concept sizes.
    pub size_distribution: Vec<(usize, usize)>,
}

/// Analyze the qualia space of a cause-effect structure.
pub fn analyze_qualia_space(ces: &CauseEffectStructure) -> QualiaSpace {
    if ces.concepts.is_empty() {
        return QualiaSpace {
            n_concepts: 0,
            mean_phi: 0.0,
            std_phi: 0.0,
            max_phi: 0.0,
            min_phi: 0.0,
            size_distribution: vec![],
        };
    }

    let n_concepts = ces.n_concepts();
    let phis: Vec<_> = ces.concepts.iter().map(|c| c.phi).collect();

    let mean_phi = phis.iter().sum::<f64>() / n_concepts as f64;
    let variance = phis.iter().map(|p| (p - mean_phi).powi(2)).sum::<f64>() / n_concepts as f64;
    let std_phi = variance.sqrt();

    let max_phi = phis
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .copied()
        .unwrap_or(0.0);

    let min_phi = phis
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .copied()
        .unwrap_or(0.0);

    // Compute size distribution
    let mut size_counts: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    for concept in &ces.concepts {
        *size_counts.entry(concept.mechanism_size()).or_insert(0) += 1;
    }

    let mut size_distribution: Vec<_> = size_counts.into_iter().collect();
    size_distribution.sort_by_key(|(size, _)| *size);

    QualiaSpace {
        n_concepts,
        mean_phi,
        std_phi,
        max_phi,
        min_phi,
        size_distribution,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{Array, IxDyn};

    fn simple_tpm(n: usize) -> ArrayD<f64> {
        let shape: Vec<_> = (0..2 * n).map(|_| 2).collect();
        Array::from_elem(IxDyn(&shape), 0.5)
    }

    fn simple_connectivity(n: usize) -> Vec<Vec<bool>> {
        vec![vec![true; n]; n]
    }

    #[test]
    fn test_generate_mechanisms() {
        let mechanisms = generate_mechanisms(3, None);

        // Should have 2^3 - 1 = 7 mechanisms (excluding empty)
        assert_eq!(mechanisms.len(), 7);

        // Should not contain empty mechanism
        assert!(!mechanisms.contains(&vec![]));
    }

    #[test]
    fn test_generate_mechanisms_max_size() {
        let mechanisms = generate_mechanisms(4, Some(2));

        // Should only have mechanisms of size 1 and 2
        for mech in mechanisms {
            assert!(mech.len() <= 2);
        }
    }

    #[test]
    fn test_identify_concepts() {
        let state = vec![1, 0];
        let tpm = simple_tpm(2);
        let conn = simple_connectivity(2);
        let config = ConceptConfig::default();

        let ces = identify_concepts(&state, &tpm, &conn, &config).unwrap();

        // Should have found some concepts
        assert!(!ces.concepts.is_empty());
        assert!(ces.phi >= 0.0);
    }

    #[test]
    fn test_concept_filtering() {
        let state = vec![1, 0, 1];
        let tpm = simple_tpm(3);
        let conn = simple_connectivity(3);

        let mut config = ConceptConfig::default();
        config.min_phi = 0.1;

        let ces = identify_concepts(&state, &tpm, &conn, &config).unwrap();

        // All concepts should have φ >= min_phi
        for concept in &ces.concepts {
            assert!(concept.phi >= config.min_phi);
        }
    }

    #[test]
    fn test_significant_concepts() {
        let state = vec![1, 0];
        let tpm = simple_tpm(2);
        let conn = simple_connectivity(2);
        let config = ConceptConfig::default();

        let ces = identify_concepts(&state, &tpm, &conn, &config).unwrap();

        let significant = ces.significant_concepts(0.01);

        // All should be above threshold
        for concept in significant {
            assert!(concept.phi > 0.01);
        }
    }

    #[test]
    fn test_concepts_by_size() {
        let state = vec![1, 0, 1];
        let tpm = simple_tpm(3);
        let conn = simple_connectivity(3);
        let config = ConceptConfig::default();

        let ces = identify_concepts(&state, &tpm, &conn, &config).unwrap();

        let size_1 = ces.concepts_by_size(1);

        for concept in size_1 {
            assert_eq!(concept.mechanism_size(), 1);
        }
    }

    #[test]
    fn test_analyze_qualia_space() {
        let state = vec![1, 0];
        let tpm = simple_tpm(2);
        let conn = simple_connectivity(2);
        let config = ConceptConfig::default();

        let ces = identify_concepts(&state, &tpm, &conn, &config).unwrap();

        let qualia = analyze_qualia_space(&ces);

        assert_eq!(qualia.n_concepts, ces.n_concepts());
        assert!(qualia.mean_phi >= 0.0);
        assert!(qualia.max_phi >= qualia.min_phi);
    }

    #[test]
    fn test_compare_structures() {
        let state = vec![1, 0];
        let tpm = simple_tpm(2);
        let conn = simple_connectivity(2);
        let config = ConceptConfig::default();

        let ces1 = identify_concepts(&state, &tpm, &conn, &config).unwrap();
        let ces2 = ces1.clone();

        let distance = compare_structures(&ces1, &ces2).unwrap();

        // Identical structures should have zero distance
        assert!((distance - 0.0).abs() < 1e-10);
    }
}
