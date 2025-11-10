//! Earth Mover's Distance (EMD) for comparing probability distributions.
//!
//! This module implements the Earth Mover's Distance, also known as the Wasserstein metric,
//! which is used in IIT to measure the distance between cause-effect repertoires.
//!
//! # Theory
//!
//! The EMD is the minimum cost of transforming one distribution into another, where
//! the cost is the amount of probability mass times the distance it must be moved.
//! In IIT, this measures the difference between the whole system's repertoire and
//! the partitioned repertoire.

use crate::error::{IITError, Result};
use crate::repertoire::Repertoire;
use ndarray::ArrayD;
use std::cmp::Ordering;

/// Compute Earth Mover's Distance between two repertoires.
///
/// This is a simplified implementation for discrete binary states.
/// For the full EMD, linear programming solvers would be needed.
///
/// # Arguments
///
/// * `r1` - First repertoire
/// * `r2` - Second repertoire
///
/// # Returns
///
/// The EMD distance between the two repertoires
pub fn earth_movers_distance(r1: &Repertoire, r2: &Repertoire) -> Result<f64> {
    // Check compatibility
    if r1.n_states() != r2.n_states() {
        return Err(IITError::EMDError(format!(
            "Incompatible repertoires: {} vs {} states",
            r1.n_states(),
            r2.n_states()
        )));
    }

    if r1.direction != r2.direction {
        return Err(IITError::EMDError(
            "Cannot compare cause and effect repertoires".to_string(),
        ));
    }

    // For discrete binary states, use L1 distance as approximation
    // Full EMD would require solving linear program
    l1_distance(&r1.distribution, &r2.distribution)
}

/// L1 distance (total variation distance) between distributions.
///
/// This is a lower bound on the EMD and is exact when the state space is
/// totally ordered (1D). For binary states, it provides a good approximation.
///
/// d_L1(p, q) = 0.5 * Σ |p(x) - q(x)|
pub fn l1_distance(p: &ArrayD<f64>, q: &ArrayD<f64>) -> Result<f64> {
    if p.shape() != q.shape() {
        return Err(IITError::DimensionMismatch {
            expected: p.len(),
            actual: q.len(),
        });
    }

    let distance = 0.5
        * p.iter()
            .zip(q.iter())
            .map(|(pi, qi)| (pi - qi).abs())
            .sum::<f64>();

    Ok(distance)
}

/// Kullback-Leibler divergence from q to p.
///
/// KL(p || q) = Σ p(x) log(p(x) / q(x))
///
/// This is an alternative distance measure used in some IIT formulations.
pub fn kl_divergence(p: &ArrayD<f64>, q: &ArrayD<f64>) -> Result<f64> {
    if p.shape() != q.shape() {
        return Err(IITError::DimensionMismatch {
            expected: p.len(),
            actual: q.len(),
        });
    }

    let mut kl = 0.0;
    for (pi, qi) in p.iter().zip(q.iter()) {
        if *pi > 0.0 {
            if *qi <= 0.0 {
                // Infinite divergence
                return Ok(f64::INFINITY);
            }
            kl += pi * (pi / qi).ln();
        }
    }

    Ok(kl / std::f64::consts::LN_2) // Convert to bits
}

/// Jensen-Shannon divergence (symmetric version of KL).
///
/// JSD(p, q) = 0.5 * KL(p || m) + 0.5 * KL(q || m)
/// where m = 0.5 * (p + q)
pub fn js_divergence(p: &ArrayD<f64>, q: &ArrayD<f64>) -> Result<f64> {
    if p.shape() != q.shape() {
        return Err(IITError::DimensionMismatch {
            expected: p.len(),
            actual: q.len(),
        });
    }

    // Compute mixture
    let m: ArrayD<f64> = (p + q).mapv(|x| x * 0.5);

    let kl_pm = kl_divergence(p, &m)?;
    let kl_qm = kl_divergence(q, &m)?;

    Ok(0.5 * (kl_pm + kl_qm))
}

/// Compute the EMD using a simple greedy matching algorithm.
///
/// This is an approximation suitable for small discrete distributions.
/// For large or continuous distributions, use a proper EMD solver.
pub fn emd_greedy(r1: &Repertoire, r2: &Repertoire) -> Result<f64> {
    if r1.n_states() != r2.n_states() {
        return Err(IITError::EMDError(
            "Incompatible repertoire sizes".to_string(),
        ));
    }

    let n = r1.n_states();

    // Create list of (state_index, probability) pairs
    let mut supply: Vec<_> = r1
        .distribution
        .iter()
        .enumerate()
        .map(|(i, &p)| (i, p))
        .collect();

    let mut demand: Vec<_> = r2
        .distribution
        .iter()
        .enumerate()
        .map(|(i, &p)| (i, p))
        .collect();

    let mut total_cost = 0.0;

    // Greedy matching: repeatedly match closest states
    while !supply.is_empty() && !demand.is_empty() {
        // Find closest pair
        let mut min_dist = f64::INFINITY;
        let mut best_s = 0;
        let mut best_d = 0;

        for (si, &(s_idx, s_prob)) in supply.iter().enumerate() {
            for (di, &(d_idx, d_prob)) in demand.iter().enumerate() {
                if s_prob > 0.0 && d_prob > 0.0 {
                    let dist = hamming_distance(s_idx, d_idx, r1.n_elements());
                    if dist < min_dist {
                        min_dist = dist;
                        best_s = si;
                        best_d = di;
                    }
                }
            }
        }

        // Move as much as possible
        let (s_idx, s_prob) = supply[best_s];
        let (d_idx, d_prob) = demand[best_d];
        let amount = s_prob.min(d_prob);

        total_cost += amount * min_dist;

        // Update supply and demand
        supply[best_s].1 -= amount;
        demand[best_d].1 -= amount;

        // Remove depleted entries
        supply.retain(|(_, p)| *p > 1e-10);
        demand.retain(|(_, p)| *p > 1e-10);
    }

    Ok(total_cost)
}

/// Hamming distance between two states (represented as indices).
fn hamming_distance(s1: usize, s2: usize, n_bits: usize) -> f64 {
    let mut dist = 0;
    let mut xor = s1 ^ s2;

    for _ in 0..n_bits {
        if (xor & 1) != 0 {
            dist += 1;
        }
        xor >>= 1;
    }

    dist as f64
}

/// Compute mutual information between two distributions.
///
/// This is related to KL divergence and is sometimes used in IIT.
pub fn mutual_information(
    joint: &ArrayD<f64>,
    marginal_x: &ArrayD<f64>,
    marginal_y: &ArrayD<f64>,
) -> Result<f64> {
    if joint.len() != marginal_x.len() * marginal_y.len() {
        return Err(IITError::DimensionMismatch {
            expected: marginal_x.len() * marginal_y.len(),
            actual: joint.len(),
        });
    }

    let mut mi = 0.0;

    for (i, px) in marginal_x.iter().enumerate() {
        for (j, py) in marginal_y.iter().enumerate() {
            // Linear index for joint distribution
            let idx = i * marginal_y.len() + j;
            if let Some(&pxy) = joint.get(idx) {
                if pxy > 0.0 && *px > 0.0 && *py > 0.0 {
                    mi += pxy * (pxy / (px * py)).log2();
                }
            }
        }
    }

    Ok(mi)
}

/// Effective information: mutual information between cause and effect.
///
/// This measures how much information the cause provides about the effect.
pub fn effective_information(cause: &Repertoire, effect: &Repertoire) -> Result<f64> {
    // For now, use entropy difference as approximation
    // Full implementation would compute I(cause; effect)
    let h_effect = effect.entropy();
    let h_unconstrained = (effect.n_states() as f64).log2();

    Ok(h_unconstrained - h_effect)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repertoire::Direction;
    use ndarray::{Array, IxDyn};

    #[test]
    fn test_l1_distance_identical() {
        let shape = vec![2, 2];
        let p = Array::from_vec(vec![0.25, 0.25, 0.25, 0.25])
            .into_shape(IxDyn(&shape))
            .unwrap();
        let q = p.clone();

        let dist = l1_distance(&p, &q).unwrap();
        assert!(dist.abs() < 1e-10);
    }

    #[test]
    fn test_l1_distance_different() {
        let shape = vec![2, 2];
        let p = Array::from_vec(vec![1.0, 0.0, 0.0, 0.0])
            .into_shape(IxDyn(&shape))
            .unwrap();
        let q = Array::from_vec(vec![0.0, 0.0, 0.0, 1.0])
            .into_shape(IxDyn(&shape))
            .unwrap();

        let dist = l1_distance(&p, &q).unwrap();
        assert!((dist - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_kl_divergence() {
        let shape = vec![2];
        let p = Array::from_vec(vec![0.5, 0.5])
            .into_shape(IxDyn(&shape))
            .unwrap();
        let q = p.clone();

        let kl = kl_divergence(&p, &q).unwrap();
        assert!(kl.abs() < 1e-10);
    }

    #[test]
    fn test_js_divergence() {
        let shape = vec![2];
        let p = Array::from_vec(vec![1.0, 0.0])
            .into_shape(IxDyn(&shape))
            .unwrap();
        let q = Array::from_vec(vec![0.0, 1.0])
            .into_shape(IxDyn(&shape))
            .unwrap();

        let js = js_divergence(&p, &q).unwrap();
        assert!(js > 0.0);
        assert!(js <= 1.0); // JS divergence is bounded by 1
    }

    #[test]
    fn test_hamming_distance() {
        // 000 vs 111 (3 bits different)
        assert_eq!(hamming_distance(0b000, 0b111, 3), 3.0);

        // 010 vs 011 (1 bit different)
        assert_eq!(hamming_distance(0b010, 0b011, 3), 1.0);

        // Same state
        assert_eq!(hamming_distance(0b101, 0b101, 3), 0.0);
    }

    #[test]
    fn test_emd_repertoires() {
        let r1 = Repertoire::uniform(vec![0, 1], Direction::Cause);
        let r2 = r1.clone();

        let emd = earth_movers_distance(&r1, &r2).unwrap();
        assert!(emd.abs() < 1e-10);
    }

    #[test]
    fn test_effective_information() {
        let cause = Repertoire::uniform(vec![0, 1], Direction::Cause);
        let effect = Repertoire::uniform(vec![0, 1], Direction::Effect);

        let ei = effective_information(&cause, &effect).unwrap();
        // Uniform distribution has maximum entropy, so EI should be 0
        assert!(ei.abs() < 1e-10);
    }
}
