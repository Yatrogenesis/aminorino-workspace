//! Cause-effect structure and causality analysis.
//!
//! This module implements analysis of causal relationships in IIT, including
//! cause-effect spaces and the maximally irreducible cause-effect (MICE).

use crate::emd::earth_movers_distance;
use crate::error::{IITError, Result};
use crate::repertoire::{cause_repertoire, effect_repertoire, Direction, Repertoire};
use ndarray::ArrayD;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

/// A cause-effect pair for a mechanism.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CauseEffect {
    /// The mechanism (set of elements).
    pub mechanism: Vec<usize>,
    /// The mechanism's current state.
    pub state: Vec<usize>,
    /// The cause purview (past).
    pub cause_purview: Vec<usize>,
    /// The effect purview (future).
    pub effect_purview: Vec<usize>,
    /// Cause repertoire.
    pub cause_repertoire: Repertoire,
    /// Effect repertoire.
    pub effect_repertoire: Repertoire,
    /// Cause information (φ_cause).
    pub phi_cause: f64,
    /// Effect information (φ_effect).
    pub phi_effect: f64,
}

/// Maximally Irreducible Cause (MIC).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MIC {
    /// The mechanism.
    pub mechanism: Vec<usize>,
    /// The purview that maximizes φ_cause.
    pub purview: Vec<usize>,
    /// The cause repertoire.
    pub repertoire: Repertoire,
    /// The φ_cause value.
    pub phi: f64,
}

/// Maximally Irreducible Effect (MIE).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MIE {
    /// The mechanism.
    pub mechanism: Vec<usize>,
    /// The purview that maximizes φ_effect.
    pub purview: Vec<usize>,
    /// The effect repertoire.
    pub repertoire: Repertoire,
    /// The φ_effect value.
    pub phi: f64,
}

/// Maximally Irreducible Cause-Effect (MICE).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MICE {
    /// The mechanism.
    pub mechanism: Vec<usize>,
    /// The cause component (MIC).
    pub cause: MIC,
    /// The effect component (MIE).
    pub effect: MIE,
}

/// Find the Maximally Irreducible Cause for a mechanism.
///
/// # Arguments
///
/// * `mechanism` - Elements in the mechanism
/// * `mechanism_state` - Current state of the mechanism
/// * `system_elements` - All elements in the system
/// * `tpm` - Transition probability matrix
/// * `connectivity` - Connectivity matrix
///
/// # Returns
///
/// The MIC (maximally irreducible cause)
pub fn find_mic(
    mechanism: &[usize],
    mechanism_state: &[usize],
    system_elements: &[usize],
    tpm: &ArrayD<f64>,
    connectivity: &[Vec<bool>],
) -> Result<MIC> {
    if mechanism.is_empty() {
        return Err(IITError::InvalidMechanism("Empty mechanism".to_string()));
    }

    // Find all possible purviews (subsets of system)
    let purviews = generate_purviews(system_elements);

    // Compute φ_cause for each purview in parallel
    let results: Vec<_> = purviews
        .par_iter()
        .filter_map(|purview| {
            if purview.is_empty() {
                return None;
            }

            // Compute whole repertoire
            let whole_rep = match cause_repertoire(
                mechanism,
                mechanism_state,
                purview,
                tpm,
                connectivity,
            ) {
                Ok(rep) => rep,
                Err(_) => return None,
            };

            // Compute unconstrained repertoire
            let unconstrained = Repertoire::uniform(purview.clone(), Direction::Cause);

            // Compute distance
            let phi = match earth_movers_distance(&whole_rep, &unconstrained) {
                Ok(dist) => dist,
                Err(_) => return None,
            };

            Some((purview.clone(), whole_rep, phi))
        })
        .collect();

    // Find purview with maximum φ
    results
        .into_iter()
        .max_by(|(_, _, phi1), (_, _, phi2)| {
            phi1.partial_cmp(phi2).unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(purview, repertoire, phi)| MIC {
            mechanism: mechanism.to_vec(),
            purview,
            repertoire,
            phi,
        })
        .ok_or_else(|| IITError::ComputationError("No valid purviews found".to_string()))
}

/// Find the Maximally Irreducible Effect for a mechanism.
///
/// # Arguments
///
/// * `mechanism` - Elements in the mechanism
/// * `mechanism_state` - Current state of the mechanism
/// * `system_elements` - All elements in the system
/// * `tpm` - Transition probability matrix
/// * `connectivity` - Connectivity matrix
///
/// # Returns
///
/// The MIE (maximally irreducible effect)
pub fn find_mie(
    mechanism: &[usize],
    mechanism_state: &[usize],
    system_elements: &[usize],
    tpm: &ArrayD<f64>,
    connectivity: &[Vec<bool>],
) -> Result<MIE> {
    if mechanism.is_empty() {
        return Err(IITError::InvalidMechanism("Empty mechanism".to_string()));
    }

    // Find all possible purviews
    let purviews = generate_purviews(system_elements);

    // Compute φ_effect for each purview in parallel
    let results: Vec<_> = purviews
        .par_iter()
        .filter_map(|purview| {
            if purview.is_empty() {
                return None;
            }

            // Compute whole repertoire
            let whole_rep = match effect_repertoire(
                mechanism,
                mechanism_state,
                purview,
                tpm,
                connectivity,
            ) {
                Ok(rep) => rep,
                Err(_) => return None,
            };

            // Compute unconstrained repertoire
            let unconstrained = Repertoire::uniform(purview.clone(), Direction::Effect);

            // Compute distance
            let phi = match earth_movers_distance(&whole_rep, &unconstrained) {
                Ok(dist) => dist,
                Err(_) => return None,
            };

            Some((purview.clone(), whole_rep, phi))
        })
        .collect();

    // Find purview with maximum φ
    results
        .into_iter()
        .max_by(|(_, _, phi1), (_, _, phi2)| {
            phi1.partial_cmp(phi2).unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(purview, repertoire, phi)| MIE {
            mechanism: mechanism.to_vec(),
            purview,
            repertoire,
            phi,
        })
        .ok_or_else(|| IITError::ComputationError("No valid purviews found".to_string()))
}

/// Find the Maximally Irreducible Cause-Effect for a mechanism.
///
/// This combines the MIC and MIE for a mechanism.
pub fn find_mice(
    mechanism: &[usize],
    mechanism_state: &[usize],
    system_elements: &[usize],
    tpm: &ArrayD<f64>,
    connectivity: &[Vec<bool>],
) -> Result<MICE> {
    let cause = find_mic(mechanism, mechanism_state, system_elements, tpm, connectivity)?;
    let effect = find_mie(mechanism, mechanism_state, system_elements, tpm, connectivity)?;

    Ok(MICE {
        mechanism: mechanism.to_vec(),
        cause,
        effect,
    })
}

/// Generate all possible purviews (power set).
fn generate_purviews(elements: &[usize]) -> Vec<Vec<usize>> {
    let n = elements.len();
    (0..(1 << n))
        .map(|mask| {
            elements
                .iter()
                .enumerate()
                .filter(|(i, _)| (mask & (1 << i)) != 0)
                .map(|(_, &e)| e)
                .collect()
        })
        .collect()
}

/// Compute effective information between a mechanism and purview.
///
/// This measures how much the mechanism constrains the purview.
pub fn effective_information(
    mechanism: &[usize],
    mechanism_state: &[usize],
    purview: &[usize],
    tpm: &ArrayD<f64>,
    connectivity: &[Vec<bool>],
    direction: Direction,
) -> Result<f64> {
    let repertoire = match direction {
        Direction::Cause => {
            cause_repertoire(mechanism, mechanism_state, purview, tpm, connectivity)?
        }
        Direction::Effect => {
            effect_repertoire(mechanism, mechanism_state, purview, tpm, connectivity)?
        }
    };

    let unconstrained = Repertoire::uniform(purview.to_vec(), direction);

    earth_movers_distance(&repertoire, &unconstrained)
}

/// Compute the cause-effect space for a mechanism.
///
/// This returns the full distribution of φ values across all purviews.
pub fn cause_effect_space(
    mechanism: &[usize],
    mechanism_state: &[usize],
    system_elements: &[usize],
    tpm: &ArrayD<f64>,
    connectivity: &[Vec<bool>],
) -> Result<Vec<(Vec<usize>, f64, f64)>> {
    let purviews = generate_purviews(system_elements);

    let results: Vec<_> = purviews
        .par_iter()
        .filter_map(|purview| {
            if purview.is_empty() {
                return None;
            }

            let phi_cause = effective_information(
                mechanism,
                mechanism_state,
                purview,
                tpm,
                connectivity,
                Direction::Cause,
            )
            .ok()?;

            let phi_effect = effective_information(
                mechanism,
                mechanism_state,
                purview,
                tpm,
                connectivity,
                Direction::Effect,
            )
            .ok()?;

            Some((purview.clone(), phi_cause, phi_effect))
        })
        .collect();

    Ok(results)
}

/// Analyze the causal structure of a system.
///
/// Returns all mechanisms with their MICE.
pub fn analyze_causal_structure(
    system_state: &[usize],
    tpm: &ArrayD<f64>,
    connectivity: &[Vec<bool>],
) -> Result<Vec<MICE>> {
    let n = system_state.len();
    let system_elements: Vec<_> = (0..n).collect();

    // Generate all mechanisms (power set excluding empty)
    let mechanisms: Vec<Vec<usize>> = (1..(1 << n))
        .map(|mask| {
            (0..n)
                .filter(|&i| (mask & (1 << i)) != 0)
                .collect()
        })
        .collect();

    // Find MICE for each mechanism
    let results: Vec<_> = mechanisms
        .par_iter()
        .filter_map(|mechanism| {
            let mech_state: Vec<_> = mechanism.iter().map(|&i| system_state[i]).collect();
            find_mice(mechanism, &mech_state, &system_elements, tpm, connectivity).ok()
        })
        .collect();

    Ok(results)
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
    fn test_generate_purviews() {
        let elements = vec![0, 1, 2];
        let purviews = generate_purviews(&elements);

        // Should have 2^3 = 8 purviews (including empty)
        assert_eq!(purviews.len(), 8);

        // Should include empty set
        assert!(purviews.contains(&vec![]));

        // Should include full set
        assert!(purviews.contains(&vec![0, 1, 2]));
    }

    #[test]
    fn test_find_mic() {
        let mechanism = vec![0];
        let state = vec![1];
        let elements = vec![0, 1];
        let tpm = simple_tpm(2);
        let conn = simple_connectivity(2);

        let mic = find_mic(&mechanism, &state, &elements, &tpm, &conn).unwrap();

        assert_eq!(mic.mechanism, mechanism);
        assert!(mic.phi >= 0.0);
    }

    #[test]
    fn test_find_mie() {
        let mechanism = vec![0];
        let state = vec![1];
        let elements = vec![0, 1];
        let tpm = simple_tpm(2);
        let conn = simple_connectivity(2);

        let mie = find_mie(&mechanism, &state, &elements, &tpm, &conn).unwrap();

        assert_eq!(mie.mechanism, mechanism);
        assert!(mie.phi >= 0.0);
    }

    #[test]
    fn test_find_mice() {
        let mechanism = vec![0];
        let state = vec![1];
        let elements = vec![0, 1];
        let tpm = simple_tpm(2);
        let conn = simple_connectivity(2);

        let mice = find_mice(&mechanism, &state, &elements, &tpm, &conn).unwrap();

        assert_eq!(mice.mechanism, mechanism);
        assert!(mice.cause.phi >= 0.0);
        assert!(mice.effect.phi >= 0.0);
    }

    #[test]
    fn test_effective_information() {
        let mechanism = vec![0];
        let state = vec![1];
        let purview = vec![1];
        let tpm = simple_tpm(2);
        let conn = simple_connectivity(2);

        let ei_cause =
            effective_information(&mechanism, &state, &purview, &tpm, &conn, Direction::Cause)
                .unwrap();
        let ei_effect =
            effective_information(&mechanism, &state, &purview, &tpm, &conn, Direction::Effect)
                .unwrap();

        assert!(ei_cause >= 0.0);
        assert!(ei_effect >= 0.0);
    }

    #[test]
    fn test_cause_effect_space() {
        let mechanism = vec![0];
        let state = vec![1];
        let elements = vec![0, 1];
        let tpm = simple_tpm(2);
        let conn = simple_connectivity(2);

        let space = cause_effect_space(&mechanism, &state, &elements, &tpm, &conn).unwrap();

        // Should have multiple purviews
        assert!(!space.is_empty());

        // All phi values should be non-negative
        for (_, phi_c, phi_e) in space {
            assert!(phi_c >= 0.0);
            assert!(phi_e >= 0.0);
        }
    }
}
