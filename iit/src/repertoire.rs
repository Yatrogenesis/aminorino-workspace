//! Cause and effect repertoire computation.
//!
//! This module implements the computation of cause and effect repertoires, which are
//! probability distributions over past and future states given a mechanism and purview.
//!
//! # Theory
//!
//! In IIT 3.0, a cause repertoire specifies the probability distribution over past states
//! of a purview given the current state of a mechanism. An effect repertoire specifies
//! the probability distribution over future states of a purview given the current state
//! of a mechanism.

use crate::error::{IITError, Result};
use ndarray::{Array, ArrayD, IxDyn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A probability repertoire (distribution).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repertoire {
    /// Probability distribution as a multi-dimensional array.
    /// Dimensions correspond to the elements in the purview.
    pub distribution: ArrayD<f64>,
    /// Elements (indices) included in this repertoire.
    pub elements: Vec<usize>,
    /// Whether this is a cause or effect repertoire.
    pub direction: Direction,
}

/// Direction of causality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    /// Cause repertoire (past states).
    Cause,
    /// Effect repertoire (future states).
    Effect,
}

impl Repertoire {
    /// Create a new repertoire.
    ///
    /// # Arguments
    ///
    /// * `distribution` - Probability distribution
    /// * `elements` - Elements in the repertoire
    /// * `direction` - Cause or effect
    pub fn new(distribution: ArrayD<f64>, elements: Vec<usize>, direction: Direction) -> Result<Self> {
        // Validate probability distribution
        let sum: f64 = distribution.iter().sum();
        if (sum - 1.0).abs() > 1e-6 {
            return Err(IITError::InvalidState(format!(
                "Distribution does not sum to 1.0 (sum = {})",
                sum
            )));
        }

        if distribution.iter().any(|&p| p < 0.0 || p > 1.0) {
            return Err(IITError::InvalidState(
                "Distribution contains invalid probabilities".to_string(),
            ));
        }

        Ok(Self {
            distribution,
            elements,
            direction,
        })
    }

    /// Create a uniform (maximum entropy) repertoire.
    ///
    /// # Arguments
    ///
    /// * `elements` - Elements in the repertoire
    /// * `direction` - Cause or effect
    pub fn uniform(elements: Vec<usize>, direction: Direction) -> Self {
        let n_states = 1 << elements.len(); // 2^n states
        let shape = vec![2; elements.len()];
        let prob = 1.0 / n_states as f64;
        let distribution = Array::from_elem(IxDyn(&shape), prob);

        Self {
            distribution,
            elements,
            direction,
        }
    }

    /// Get the number of elements in the repertoire.
    pub fn n_elements(&self) -> usize {
        self.elements.len()
    }

    /// Get the number of states in the repertoire.
    pub fn n_states(&self) -> usize {
        self.distribution.len()
    }

    /// Get the entropy of the repertoire.
    ///
    /// H(X) = -Σ p(x) log₂ p(x)
    pub fn entropy(&self) -> f64 {
        self.distribution
            .iter()
            .filter(|&&p| p > 0.0)
            .map(|&p| -p * p.log2())
            .sum()
    }

    /// Normalize the distribution to sum to 1.
    pub fn normalize(&mut self) -> Result<()> {
        let sum: f64 = self.distribution.iter().sum();
        if sum <= 0.0 {
            return Err(IITError::NumericalInstability(
                "Cannot normalize: sum is zero or negative".to_string(),
            ));
        }
        self.distribution.mapv_inplace(|p| p / sum);
        Ok(())
    }

    /// Marginalize over a subset of elements.
    ///
    /// # Arguments
    ///
    /// * `elements_to_keep` - Elements to keep (others are marginalized out)
    pub fn marginalize(&self, elements_to_keep: &[usize]) -> Result<Self> {
        if elements_to_keep.is_empty() {
            return Err(IITError::InvalidPurview(
                "Cannot marginalize to empty set".to_string(),
            ));
        }

        // Find indices of elements to keep
        let mut keep_indices = Vec::new();
        for &elem in elements_to_keep {
            if let Some(idx) = self.elements.iter().position(|&e| e == elem) {
                keep_indices.push(idx);
            } else {
                return Err(IITError::InvalidPurview(format!(
                    "Element {} not in repertoire",
                    elem
                )));
            }
        }

        // Create new distribution shape
        let new_shape = vec![2; keep_indices.len()];
        let mut new_dist = Array::zeros(IxDyn(&new_shape));

        // Sum over all states
        for (state_idx, &prob) in self.distribution.iter().enumerate() {
            // Convert flat index to multi-dimensional index
            let mut idx = state_idx;
            let mut state = vec![0; self.n_elements()];
            for i in (0..self.n_elements()).rev() {
                state[i] = idx % 2;
                idx /= 2;
            }

            // Extract kept state
            let kept_state: Vec<_> = keep_indices.iter().map(|&i| state[i]).collect();

            // Convert to flat index for new distribution
            let mut new_idx = 0;
            for &bit in &kept_state {
                new_idx = new_idx * 2 + bit;
            }

            // Add probability
            if let Some(val) = new_dist.get_mut(IxDyn(&kept_state)) {
                *val += prob;
            }
        }

        Repertoire::new(new_dist, elements_to_keep.to_vec(), self.direction)
    }
}

/// Compute cause repertoire given mechanism state, purview, and TPM.
///
/// # Arguments
///
/// * `mechanism` - Indices of mechanism elements
/// * `mechanism_state` - Current state of mechanism
/// * `purview` - Indices of purview elements
/// * `tpm` - Transition probability matrix
/// * `connectivity` - Connectivity matrix
pub fn cause_repertoire(
    mechanism: &[usize],
    mechanism_state: &[usize],
    purview: &[usize],
    tpm: &ArrayD<f64>,
    connectivity: &[Vec<bool>],
) -> Result<Repertoire> {
    if mechanism.len() != mechanism_state.len() {
        return Err(IITError::dimension_mismatch(
            mechanism.len(),
            mechanism_state.len(),
        ));
    }

    // Check connectivity: which purview elements affect mechanism
    let mut relevant_purview = Vec::new();
    for &p in purview {
        for &m in mechanism {
            if p < connectivity.len() && m < connectivity[p].len() && connectivity[p][m] {
                relevant_purview.push(p);
                break;
            }
        }
    }

    if relevant_purview.is_empty() {
        // No causal connections: return uniform distribution
        return Ok(Repertoire::uniform(purview.to_vec(), Direction::Cause));
    }

    // Compute probability of each past state given current mechanism state
    let n_purview_states = 1 << purview.len();
    let shape = vec![2; purview.len()];
    let mut distribution = Array::zeros(IxDyn(&shape));

    for past_state_idx in 0..n_purview_states {
        // Convert to binary state
        let mut past_state = vec![0; purview.len()];
        let mut idx = past_state_idx;
        for i in (0..purview.len()).rev() {
            past_state[i] = idx % 2;
            idx /= 2;
        }

        // Compute P(mechanism_state | past_state) using TPM
        let prob = compute_conditional_prob(
            mechanism,
            mechanism_state,
            purview,
            &past_state,
            tpm,
        )?;

        // Store in distribution
        if let Some(val) = distribution.get_mut(IxDyn(&past_state)) {
            *val = prob;
        }
    }

    // Normalize
    let mut repertoire = Repertoire::new(distribution, purview.to_vec(), Direction::Cause)?;
    repertoire.normalize()?;

    Ok(repertoire)
}

/// Compute effect repertoire given mechanism state, purview, and TPM.
///
/// # Arguments
///
/// * `mechanism` - Indices of mechanism elements
/// * `mechanism_state` - Current state of mechanism
/// * `purview` - Indices of purview elements
/// * `tpm` - Transition probability matrix
/// * `connectivity` - Connectivity matrix
pub fn effect_repertoire(
    mechanism: &[usize],
    mechanism_state: &[usize],
    purview: &[usize],
    tpm: &ArrayD<f64>,
    connectivity: &[Vec<bool>],
) -> Result<Repertoire> {
    if mechanism.len() != mechanism_state.len() {
        return Err(IITError::dimension_mismatch(
            mechanism.len(),
            mechanism_state.len(),
        ));
    }

    // Check connectivity: which purview elements are affected by mechanism
    let mut relevant_purview = Vec::new();
    for &p in purview {
        for &m in mechanism {
            if m < connectivity.len() && p < connectivity[m].len() && connectivity[m][p] {
                relevant_purview.push(p);
                break;
            }
        }
    }

    if relevant_purview.is_empty() {
        // No causal connections: return uniform distribution
        return Ok(Repertoire::uniform(purview.to_vec(), Direction::Effect));
    }

    // Compute probability of each future state given current mechanism state
    let n_purview_states = 1 << purview.len();
    let shape = vec![2; purview.len()];
    let mut distribution = Array::zeros(IxDyn(&shape));

    for future_state_idx in 0..n_purview_states {
        // Convert to binary state
        let mut future_state = vec![0; purview.len()];
        let mut idx = future_state_idx;
        for i in (0..purview.len()).rev() {
            future_state[i] = idx % 2;
            idx /= 2;
        }

        // Compute P(future_state | mechanism_state) using TPM
        let prob = compute_conditional_prob(
            purview,
            &future_state,
            mechanism,
            mechanism_state,
            tpm,
        )?;

        // Store in distribution
        if let Some(val) = distribution.get_mut(IxDyn(&future_state)) {
            *val = prob;
        }
    }

    // Normalize
    let mut repertoire = Repertoire::new(distribution, purview.to_vec(), Direction::Effect)?;
    repertoire.normalize()?;

    Ok(repertoire)
}

/// Compute conditional probability from TPM.
///
/// P(effect_state | cause_state)
fn compute_conditional_prob(
    effect_elements: &[usize],
    effect_state: &[usize],
    cause_elements: &[usize],
    cause_state: &[usize],
    tpm: &ArrayD<f64>,
) -> Result<f64> {
    // Build full system state index for TPM lookup
    // This is simplified - real implementation needs proper TPM indexing

    // For now, return uniform probability
    // TODO: Implement proper TPM indexing based on state-by-node format
    Ok(1.0 / (1 << effect_elements.len()) as f64)
}

/// Compute the unconstrained repertoire (maximum entropy distribution).
///
/// This is used as the reference distribution for computing integrated information.
pub fn unconstrained_repertoire(elements: &[usize], direction: Direction) -> Repertoire {
    Repertoire::uniform(elements.to_vec(), direction)
}

/// Cache for repertoire computations.
pub struct RepertoireCache {
    cache: HashMap<RepertoireKey, Repertoire>,
    max_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RepertoireKey {
    mechanism: Vec<usize>,
    mechanism_state: Vec<usize>,
    purview: Vec<usize>,
    direction: Direction,
}

impl RepertoireCache {
    /// Create a new repertoire cache.
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
        }
    }

    /// Get a cached repertoire.
    pub fn get(
        &self,
        mechanism: &[usize],
        mechanism_state: &[usize],
        purview: &[usize],
        direction: Direction,
    ) -> Option<&Repertoire> {
        let key = RepertoireKey {
            mechanism: mechanism.to_vec(),
            mechanism_state: mechanism_state.to_vec(),
            purview: purview.to_vec(),
            direction,
        };
        self.cache.get(&key)
    }

    /// Insert a repertoire into the cache.
    pub fn insert(
        &mut self,
        mechanism: Vec<usize>,
        mechanism_state: Vec<usize>,
        purview: Vec<usize>,
        direction: Direction,
        repertoire: Repertoire,
    ) {
        if self.cache.len() >= self.max_size {
            // Simple eviction
            if let Some(key) = self.cache.keys().next().cloned() {
                self.cache.remove(&key);
            }
        }
        let key = RepertoireKey {
            mechanism,
            mechanism_state,
            purview,
            direction,
        };
        self.cache.insert(key, repertoire);
    }

    /// Clear the cache.
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_repertoire() {
        let rep = Repertoire::uniform(vec![0, 1, 2], Direction::Cause);
        assert_eq!(rep.n_elements(), 3);
        assert_eq!(rep.n_states(), 8);

        // All probabilities should be 1/8
        for &p in rep.distribution.iter() {
            assert!((p - 0.125).abs() < 1e-10);
        }
    }

    #[test]
    fn test_repertoire_entropy() {
        let rep = Repertoire::uniform(vec![0, 1], Direction::Effect);
        // Uniform distribution of 4 states has entropy log2(4) = 2
        assert!((rep.entropy() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_normalize() {
        let shape = vec![2, 2];
        let dist = Array::from_vec(vec![1.0, 2.0, 3.0, 4.0]).into_shape(IxDyn(&shape)).unwrap();
        let mut rep = Repertoire::new(dist, vec![0, 1], Direction::Cause).unwrap_err(); // Should fail

        // Create unnormalized and normalize
        let dist = Array::from_vec(vec![1.0, 2.0, 3.0, 4.0]).into_shape(IxDyn(&shape)).unwrap();
        let mut rep = Repertoire {
            distribution: dist,
            elements: vec![0, 1],
            direction: Direction::Cause,
        };
        rep.normalize().unwrap();

        let sum: f64 = rep.distribution.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_repertoire_cache() {
        let mut cache = RepertoireCache::new(10);

        let rep = Repertoire::uniform(vec![0, 1], Direction::Cause);
        cache.insert(vec![0], vec![1], vec![0, 1], Direction::Cause, rep);

        let retrieved = cache.get(&[0], &[1], &[0, 1], Direction::Cause);
        assert!(retrieved.is_some());
    }
}
