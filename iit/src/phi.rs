//! Integrated Information (Φ) calculation methods.
//!
//! This module implements various methods for calculating Φ (Phi), the measure of
//! integrated information in IIT 3.0. Multiple approximation methods are provided
//! for handling systems of different sizes.
//!
//! # Theory
//!
//! Φ measures the irreducibility of a system to its parts. It is defined as the
//! distance between the cause-effect structure of the whole system and the
//! cause-effect structure of the system's Minimum Information Partition (MIP).
//!
//! # Methods
//!
//! - **Exact**: Exhaustive search over all partitions (≤ 15 elements)
//! - **Geometric**: Balduzzi & Tononi (2008) geometric approximation
//! - **Spectral**: Based on eigenvalue decomposition
//! - **Mean Field**: Statistical physics approximation for large systems
//! - **Tau (τ)**: Simplified approximation based on connectivity

use crate::emd::earth_movers_distance;
use crate::error::{IITError, Result};
use crate::partition::{find_mip, CutType, Partition, PartitionInfo};
use crate::repertoire::{cause_repertoire, effect_repertoire, Repertoire};
use nalgebra::DMatrix;
use ndarray::ArrayD;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Configuration for Φ calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhiConfig {
    /// Maximum system size for exact calculation.
    pub max_exact_size: usize,
    /// Approximation method to use for large systems.
    pub approximation: ApproximationMethod,
    /// Cut type (unidirectional or bidirectional).
    pub cut_type: CutType,
    /// Whether to use parallel computation.
    pub parallel: bool,
    /// Cache size for intermediate results.
    pub cache_size: usize,
}

impl Default for PhiConfig {
    fn default() -> Self {
        Self {
            max_exact_size: 15,
            approximation: ApproximationMethod::Geometric,
            cut_type: CutType::Unidirectional,
            parallel: true,
            cache_size: 1000,
        }
    }
}

/// Approximation methods for large systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApproximationMethod {
    /// Exact exhaustive search.
    Exact,
    /// Geometric approximation (Balduzzi & Tononi 2008).
    Geometric,
    /// Spectral approximation based on eigenvalues.
    Spectral,
    /// Mean field theory approximation.
    MeanField,
    /// Tau (τ) approximation based on connectivity.
    Tau,
}

/// Result of a Φ calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhiResult {
    /// The integrated information value.
    pub phi: f64,
    /// The minimum information partition.
    pub mip: Option<PartitionInfo>,
    /// Method used for calculation.
    pub method: ApproximationMethod,
    /// Number of partitions evaluated.
    pub n_partitions: usize,
    /// Computation time in milliseconds.
    pub computation_time_ms: u128,
}

/// Calculate Φ for a system in a given state.
///
/// # Arguments
///
/// * `system_state` - Current state of the system
/// * `tpm` - Transition probability matrix
/// * `connectivity` - Connectivity matrix
/// * `config` - Configuration for calculation
///
/// # Returns
///
/// Result containing the Φ value and related information
pub fn calculate_phi(
    system_state: &[usize],
    tpm: &ArrayD<f64>,
    connectivity: &[Vec<bool>],
    config: &PhiConfig,
) -> Result<PhiResult> {
    let start = std::time::Instant::now();
    let n = system_state.len();

    if n == 0 {
        return Err(IITError::InvalidState("Empty system".to_string()));
    }

    if n == 1 {
        return Ok(PhiResult {
            phi: 0.0,
            mip: None,
            method: ApproximationMethod::Exact,
            n_partitions: 0,
            computation_time_ms: start.elapsed().as_millis(),
        });
    }

    // Choose method based on system size
    let method = if n <= config.max_exact_size {
        ApproximationMethod::Exact
    } else {
        config.approximation
    };

    let (phi, mip, n_partitions) = match method {
        ApproximationMethod::Exact => {
            calculate_phi_exact(system_state, tpm, connectivity, config)?
        }
        ApproximationMethod::Geometric => {
            let phi = calculate_phi_geometric(connectivity)?;
            (phi, None, 0)
        }
        ApproximationMethod::Spectral => {
            let phi = calculate_phi_spectral(connectivity)?;
            (phi, None, 0)
        }
        ApproximationMethod::MeanField => {
            let phi = calculate_phi_mean_field(system_state, tpm, connectivity)?;
            (phi, None, 0)
        }
        ApproximationMethod::Tau => {
            let phi = calculate_tau(connectivity)?;
            (phi, None, 0)
        }
    };

    Ok(PhiResult {
        phi,
        mip,
        method,
        n_partitions,
        computation_time_ms: start.elapsed().as_millis(),
    })
}

/// Calculate Φ using exact exhaustive search.
fn calculate_phi_exact(
    system_state: &[usize],
    tpm: &ArrayD<f64>,
    connectivity: &[Vec<bool>],
    config: &PhiConfig,
) -> Result<(f64, Option<PartitionInfo>, usize)> {
    let n = system_state.len();
    let mechanism: Vec<_> = (0..n).collect();

    // Compute whole system repertoires
    let whole_cause = cause_repertoire(&mechanism, system_state, &mechanism, tpm, connectivity)?;
    let whole_effect = effect_repertoire(&mechanism, system_state, &mechanism, tpm, connectivity)?;

    let n_partitions = Arc::new(Mutex::new(0));

    // Find MIP
    let phi_fn = |partition: &Partition| -> Result<f64> {
        *n_partitions.lock().unwrap() += 1;

        // Compute partitioned repertoires
        let part_cause = partitioned_repertoire(
            &mechanism,
            system_state,
            &mechanism,
            tpm,
            connectivity,
            partition,
            true,
        )?;

        let part_effect = partitioned_repertoire(
            &mechanism,
            system_state,
            &mechanism,
            tpm,
            connectivity,
            partition,
            false,
        )?;

        // Compute distance
        let dist_cause = earth_movers_distance(&whole_cause, &part_cause)?;
        let dist_effect = earth_movers_distance(&whole_effect, &part_effect)?;

        Ok(dist_cause.min(dist_effect))
    };

    let mip = find_mip(n, phi_fn, config.cut_type)?;
    let phi = mip.phi;
    let n_parts = *n_partitions.lock().unwrap();

    Ok((phi, Some(mip), n_parts))
}

/// Calculate Φ using geometric approximation (Balduzzi & Tononi 2008).
///
/// This approximation is based on the geometric properties of the connectivity matrix.
fn calculate_phi_geometric(connectivity: &[Vec<bool>]) -> Result<f64> {
    let n = connectivity.len();

    // Convert to float matrix
    let mut matrix = DMatrix::zeros(n, n);
    for i in 0..n {
        for j in 0..n {
            if j < connectivity[i].len() && connectivity[i][j] {
                matrix[(i, j)] = 1.0;
            }
        }
    }

    // Compute effective connectivity
    let forward = &matrix;
    let backward = matrix.transpose();

    // Geometric mean of forward and backward effective information
    let mut phi = 0.0;

    for i in 0..n {
        let mut in_degree = 0.0;
        let mut out_degree = 0.0;

        for j in 0..n {
            in_degree += backward[(i, j)];
            out_degree += forward[(i, j)];
        }

        if in_degree > 0.0 && out_degree > 0.0 {
            phi += ((in_degree * out_degree) as f64).sqrt();
        }
    }

    // Normalize by system size
    phi /= n as f64;

    Ok(phi)
}

/// Calculate Φ using spectral approximation.
///
/// This method uses eigenvalue decomposition of the connectivity matrix.
fn calculate_phi_spectral(connectivity: &[Vec<bool>]) -> Result<f64> {
    let n = connectivity.len();

    // Build connectivity matrix
    let mut matrix = DMatrix::zeros(n, n);
    for i in 0..n {
        for j in 0..n {
            if j < connectivity[i].len() && connectivity[i][j] {
                matrix[(i, j)] = 1.0;
            }
        }
    }

    // Compute eigenvalues
    let eigen = match matrix.clone().symmetric_eigen() {
        eigen => eigen,
    };

    // Use spectral gap as approximation
    let mut eigenvalues: Vec<f64> = eigen.eigenvalues.iter().map(|x: &f64| x.abs()).collect();
    eigenvalues.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

    if eigenvalues.len() < 2 {
        return Ok(0.0);
    }

    // Φ approximated by spectral gap
    let phi = eigenvalues[0] - eigenvalues[eigenvalues.len() - 1];

    Ok(phi.max(0.0))
}

/// Calculate Φ using mean field approximation.
///
/// This statistical physics approach approximates interactions.
fn calculate_phi_mean_field(
    system_state: &[usize],
    _tpm: &ArrayD<f64>,
    connectivity: &[Vec<bool>],
) -> Result<f64> {
    let n = system_state.len();

    // Compute mean field parameters
    let mut mean_activation = 0.0;
    for &state in system_state {
        mean_activation += state as f64;
    }
    mean_activation /= n as f64;

    // Effective coupling strength
    let mut total_connections = 0;
    for row in connectivity {
        for &conn in row {
            if conn {
                total_connections += 1;
            }
        }
    }

    let coupling = total_connections as f64 / (n * n) as f64;

    // Mean field Φ approximation
    let phi = mean_activation * coupling * (1.0 - mean_activation);

    Ok(phi * n as f64)
}

/// Calculate τ (tau), a simplified measure of integration.
///
/// Tau is based on the connectivity pattern and is much faster to compute.
fn calculate_tau(connectivity: &[Vec<bool>]) -> Result<f64> {
    let n = connectivity.len();

    // Count bidirectional connections
    let mut bidirectional = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if j < connectivity[i].len()
                && i < connectivity[j].len()
                && connectivity[i][j]
                && connectivity[j][i]
            {
                bidirectional += 1;
            }
        }
    }

    // Tau is the ratio of bidirectional to possible connections
    let max_connections = n * (n - 1) / 2;
    let tau = bidirectional as f64 / max_connections as f64;

    Ok(tau)
}

/// Compute partitioned repertoire.
fn partitioned_repertoire(
    mechanism: &[usize],
    mechanism_state: &[usize],
    purview: &[usize],
    tpm: &ArrayD<f64>,
    connectivity: &[Vec<bool>],
    partition: &Partition,
    is_cause: bool,
) -> Result<Repertoire> {
    // Create modified connectivity matrix with partition cuts
    let mut cut_connectivity = connectivity.to_vec();
    for i in 0..connectivity.len() {
        for j in 0..connectivity[i].len() {
            if partition.cuts_connection(i, j) {
                cut_connectivity[i][j] = false;
            }
        }
    }

    // Compute repertoire with cut connectivity
    if is_cause {
        cause_repertoire(mechanism, mechanism_state, purview, tpm, &cut_connectivity)
    } else {
        effect_repertoire(mechanism, mechanism_state, purview, tpm, &cut_connectivity)
    }
}

/// Calculate Φ for all possible mechanisms in a system.
///
/// This is used for computing the full cause-effect structure.
pub fn calculate_phi_all_mechanisms(
    system_state: &[usize],
    tpm: &ArrayD<f64>,
    connectivity: &[Vec<bool>],
    config: &PhiConfig,
) -> Result<Vec<(Vec<usize>, PhiResult)>> {
    let n = system_state.len();

    // Generate all possible mechanisms (power set excluding empty)
    let mechanisms: Vec<Vec<usize>> = (1..(1 << n))
        .map(|mask| {
            (0..n)
                .filter(|&i| (mask & (1 << i)) != 0)
                .collect()
        })
        .collect();

    // Calculate Φ for each mechanism in parallel
    let results: Vec<_> = if config.parallel {
        mechanisms
            .par_iter()
            .filter_map(|mech| {
                let mech_state: Vec<_> = mech.iter().map(|&i| system_state[i]).collect();
                calculate_phi(&mech_state, tpm, connectivity, config)
                    .ok()
                    .map(|result| (mech.clone(), result))
            })
            .collect()
    } else {
        mechanisms
            .iter()
            .filter_map(|mech| {
                let mech_state: Vec<_> = mech.iter().map(|&i| system_state[i]).collect();
                calculate_phi(&mech_state, tpm, connectivity, config)
                    .ok()
                    .map(|result| (mech.clone(), result))
            })
            .collect()
    };

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{Array, IxDyn};

    fn simple_connectivity(n: usize) -> Vec<Vec<bool>> {
        vec![vec![false; n]; n]
    }

    fn fully_connected(n: usize) -> Vec<Vec<bool>> {
        let mut conn = vec![vec![true; n]; n];
        for i in 0..n {
            conn[i][i] = false; // No self-connections
        }
        conn
    }

    #[test]
    fn test_single_element_phi() {
        let state = vec![1];
        let shape = vec![2, 2];
        let tpm = Array::from_elem(IxDyn(&shape), 0.5);
        let conn = simple_connectivity(1);
        let config = PhiConfig::default();

        let result = calculate_phi(&state, &tpm, &conn, &config).unwrap();
        assert_eq!(result.phi, 0.0);
    }

    #[test]
    fn test_tau_disconnected() {
        let conn = simple_connectivity(3);
        let tau = calculate_tau(&conn).unwrap();
        assert_eq!(tau, 0.0);
    }

    #[test]
    fn test_tau_fully_connected() {
        let conn = fully_connected(3);
        let tau = calculate_tau(&conn).unwrap();
        assert_eq!(tau, 1.0);
    }

    #[test]
    fn test_geometric_approximation() {
        let conn = fully_connected(4);
        let phi = calculate_phi_geometric(&conn).unwrap();
        assert!(phi > 0.0);
    }

    #[test]
    fn test_spectral_approximation() {
        let conn = fully_connected(3);
        let phi = calculate_phi_spectral(&conn).unwrap();
        assert!(phi >= 0.0);
    }

    #[test]
    fn test_mean_field_approximation() {
        let state = vec![1, 0, 1];
        let shape = vec![2, 2, 2, 2, 2, 2];
        let tpm = Array::from_elem(IxDyn(&shape), 0.5);
        let conn = fully_connected(3);

        let phi = calculate_phi_mean_field(&state, &tpm, &conn).unwrap();
        assert!(phi >= 0.0);
    }

    #[test]
    fn test_approximation_methods() {
        let state = vec![1, 0, 1, 1];
        let shape = vec![2; 8];
        let tpm = Array::from_elem(IxDyn(&shape), 0.5);
        let conn = fully_connected(4);

        for method in &[
            ApproximationMethod::Geometric,
            ApproximationMethod::Spectral,
            ApproximationMethod::MeanField,
            ApproximationMethod::Tau,
        ] {
            let mut config = PhiConfig::default();
            config.approximation = *method;
            config.max_exact_size = 0; // Force approximation

            let result = calculate_phi(&state, &tpm, &conn, &config).unwrap();
            assert!(result.phi >= 0.0);
            assert_eq!(result.method, *method);
        }
    }
}
