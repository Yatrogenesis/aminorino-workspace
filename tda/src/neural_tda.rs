//! Neuroscience-specific topological data analysis.
//!
//! This module provides specialized TDA tools for neuroscience applications:
//! - Spike train analysis and persistence
//! - Cell assembly detection using clique topology
//! - Functional connectivity from spike data
//! - Time-windowed topological analysis

use crate::distances::{SpikeTrain, victor_purpura_distance, van_rossum_distance, victor_purpura_matrix};
use crate::error::{Result, TdaError};
use crate::persistent_homology::{persistent_homology_rips, PersistentPair};
use crate::persistence_diagram::PersistenceDiagram;
use crate::simplicial_complex::{Simplex, SimplicialComplex, vietoris_rips_complex};
use nalgebra::DMatrix;
use std::collections::{HashMap, HashSet};

/// Represents a population of neurons with spike trains.
#[derive(Debug, Clone)]
pub struct NeuralPopulation {
    /// Spike trains for each neuron
    pub spike_trains: Vec<SpikeTrain>,
    /// Optional neuron IDs
    pub neuron_ids: Option<Vec<String>>,
}

impl NeuralPopulation {
    /// Create a new neural population from spike trains.
    pub fn new(spike_trains: Vec<SpikeTrain>) -> Result<Self> {
        if spike_trains.is_empty() {
            return Err(TdaError::EmptyDataset);
        }
        Ok(NeuralPopulation {
            spike_trains,
            neuron_ids: None,
        })
    }

    /// Create with neuron IDs.
    pub fn with_ids(spike_trains: Vec<SpikeTrain>, neuron_ids: Vec<String>) -> Result<Self> {
        if spike_trains.len() != neuron_ids.len() {
            return Err(TdaError::InvalidParameter(
                "Number of IDs must match number of spike trains".to_string(),
            ));
        }
        Ok(NeuralPopulation {
            spike_trains,
            neuron_ids: Some(neuron_ids),
        })
    }

    /// Get the number of neurons.
    pub fn size(&self) -> usize {
        self.spike_trains.len()
    }

    /// Compute the Victor-Purpura distance matrix.
    pub fn victor_purpura_matrix(&self, q: f64) -> Result<DMatrix<f64>> {
        victor_purpura_matrix(&self.spike_trains, q)
    }

    /// Compute persistence diagram using Victor-Purpura distance.
    ///
    /// # Arguments
    /// * `q` - Cost parameter for Victor-Purpura distance
    /// * `max_radius` - Maximum filtration radius
    /// * `max_dimension` - Maximum homology dimension to compute
    pub fn persistence_diagram_vp(
        &self,
        q: f64,
        max_radius: f64,
        max_dimension: usize,
    ) -> Result<PersistenceDiagram> {
        let dist_matrix = self.victor_purpura_matrix(q)?;
        let pairs = persistent_homology_rips(&dist_matrix, max_radius, max_dimension)?;
        Ok(PersistenceDiagram::from_pairs(&pairs, None))
    }
}

/// Sliding window analysis for time-varying topology.
#[derive(Debug, Clone)]
pub struct SlidingWindowAnalysis {
    /// Window size in time units
    pub window_size: f64,
    /// Step size for sliding window
    pub step_size: f64,
    /// Start time
    pub start_time: f64,
    /// End time
    pub end_time: f64,
}

impl SlidingWindowAnalysis {
    /// Create a new sliding window configuration.
    pub fn new(window_size: f64, step_size: f64, start_time: f64, end_time: f64) -> Result<Self> {
        if window_size <= 0.0 || step_size <= 0.0 {
            return Err(TdaError::InvalidParameter(
                "Window and step sizes must be positive".to_string(),
            ));
        }
        if end_time <= start_time {
            return Err(TdaError::InvalidParameter(
                "End time must be after start time".to_string(),
            ));
        }
        Ok(SlidingWindowAnalysis {
            window_size,
            step_size,
            start_time,
            end_time,
        })
    }

    /// Extract spike train within a time window.
    pub fn window_spike_train(&self, train: &SpikeTrain, window_start: f64) -> SpikeTrain {
        let window_end = window_start + self.window_size;
        let windowed_times: Vec<f64> = train
            .times
            .iter()
            .filter(|&&t| t >= window_start && t < window_end)
            .map(|&t| t - window_start) // Shift to start at 0
            .collect();

        SpikeTrain::new(windowed_times).unwrap_or_else(|_| SpikeTrain { times: vec![] })
    }

    /// Analyze population over sliding windows.
    ///
    /// Returns persistence diagrams for each time window.
    pub fn analyze_population(
        &self,
        population: &NeuralPopulation,
        q: f64,
        max_radius: f64,
        max_dimension: usize,
    ) -> Result<Vec<(f64, PersistenceDiagram)>> {
        let mut results = Vec::new();
        let mut window_start = self.start_time;

        while window_start + self.window_size <= self.end_time {
            // Extract windowed spike trains
            let windowed_trains: Vec<SpikeTrain> = population
                .spike_trains
                .iter()
                .map(|train| self.window_spike_train(train, window_start))
                .collect();

            let windowed_pop = NeuralPopulation::new(windowed_trains)?;

            // Compute persistence diagram for this window
            let diagram = windowed_pop.persistence_diagram_vp(q, max_radius, max_dimension)?;

            results.push((window_start, diagram));
            window_start += self.step_size;
        }

        Ok(results)
    }
}

/// Cell assembly detection using clique topology.
///
/// Detects groups of neurons that fire together, forming functional assemblies.
#[derive(Debug, Clone)]
pub struct CellAssembly {
    /// Neurons in the assembly
    pub neurons: Vec<usize>,
    /// Strength of the assembly (based on persistence or synchrony)
    pub strength: f64,
    /// Time window where assembly was detected
    pub time_window: Option<(f64, f64)>,
}

/// Detect cell assemblies using topological methods.
///
/// Uses clique topology: groups of neurons are considered assemblies if they
/// form cliques in the functional connectivity graph.
///
/// # Arguments
/// * `population` - Neural population
/// * `correlation_threshold` - Threshold for considering neurons functionally connected
/// * `min_assembly_size` - Minimum number of neurons in an assembly
///
/// # Returns
/// Vector of detected cell assemblies
pub fn detect_cell_assemblies(
    population: &NeuralPopulation,
    correlation_threshold: f64,
    min_assembly_size: usize,
) -> Result<Vec<CellAssembly>> {
    let n = population.size();

    // Build correlation matrix based on spike train similarity
    let mut correlation_matrix = DMatrix::zeros(n, n);

    for i in 0..n {
        for j in (i + 1)..n {
            let corr = compute_spike_correlation(&population.spike_trains[i],
                                                 &population.spike_trains[j])?;
            correlation_matrix[(i, j)] = corr;
            correlation_matrix[(j, i)] = corr;
        }
    }

    // Build adjacency graph
    let mut adjacency: HashMap<usize, HashSet<usize>> = HashMap::new();
    for i in 0..n {
        adjacency.insert(i, HashSet::new());
    }

    for i in 0..n {
        for j in (i + 1)..n {
            if correlation_matrix[(i, j)] >= correlation_threshold {
                adjacency.get_mut(&i).unwrap().insert(j);
                adjacency.get_mut(&j).unwrap().insert(i);
            }
        }
    }

    // Find maximal cliques (simplified implementation)
    let cliques = find_cliques(&adjacency, min_assembly_size)?;

    // Convert cliques to cell assemblies
    let assemblies: Vec<CellAssembly> = cliques
        .iter()
        .map(|clique| {
            // Compute assembly strength as average pairwise correlation
            let mut total_corr = 0.0;
            let mut count = 0;

            for &i in clique.iter() {
                for &j in clique.iter() {
                    if i < j {
                        total_corr += correlation_matrix[(i, j)];
                        count += 1;
                    }
                }
            }

            let strength = if count > 0 {
                total_corr / count as f64
            } else {
                0.0
            };

            CellAssembly {
                neurons: clique.clone(),
                strength,
                time_window: None,
            }
        })
        .collect();

    Ok(assemblies)
}

/// Compute spike correlation using cross-correlation measure.
fn compute_spike_correlation(train1: &SpikeTrain, train2: &SpikeTrain) -> Result<f64> {
    if train1.is_empty() && train2.is_empty() {
        return Ok(1.0);
    }
    if train1.is_empty() || train2.is_empty() {
        return Ok(0.0);
    }

    // Use Victor-Purpura distance and convert to similarity
    let dist = victor_purpura_distance(train1, train2, 1.0)?;
    let max_len = train1.len().max(train2.len()) as f64;

    // Convert distance to similarity (0 = dissimilar, 1 = identical)
    let similarity = if max_len > 0.0 {
        1.0 - (dist / max_len).min(1.0)
    } else {
        0.0
    };

    Ok(similarity)
}

/// Find cliques in an adjacency graph (simplified Bron-Kerbosch algorithm).
fn find_cliques(
    adjacency: &HashMap<usize, HashSet<usize>>,
    min_size: usize,
) -> Result<Vec<Vec<usize>>> {
    let mut cliques = Vec::new();
    let vertices: Vec<usize> = adjacency.keys().copied().collect();

    // Simple greedy approach for finding cliques
    for &start in &vertices {
        let mut clique = vec![start];
        let mut candidates: HashSet<usize> = adjacency[&start].clone();

        while !candidates.is_empty() {
            // Find vertex connected to all in current clique
            let next = candidates.iter().find(|&&v| {
                clique.iter().all(|&u| adjacency[&u].contains(&v))
            }).copied();

            if let Some(v) = next {
                clique.push(v);
                candidates = candidates.intersection(&adjacency[&v]).copied().collect();
            } else {
                break;
            }
        }

        if clique.len() >= min_size {
            cliques.push(clique);
        }
    }

    // Remove duplicate cliques
    cliques.sort();
    cliques.dedup();

    Ok(cliques)
}

/// Compute functional connectivity matrix from spike trains.
///
/// Returns a matrix where entry (i,j) represents the functional connection strength
/// between neurons i and j.
///
/// # Arguments
/// * `population` - Neural population
/// * `method` - Method for computing connectivity ("correlation", "victor-purpura", "van-rossum")
/// * `params` - Parameters for the chosen method (e.g., q for Victor-Purpura)
pub fn functional_connectivity(
    population: &NeuralPopulation,
    method: &str,
    params: &HashMap<String, f64>,
) -> Result<DMatrix<f64>> {
    let n = population.size();

    match method {
        "correlation" => {
            let mut conn_matrix = DMatrix::zeros(n, n);
            for i in 0..n {
                conn_matrix[(i, i)] = 1.0;
                for j in (i + 1)..n {
                    let corr = compute_spike_correlation(
                        &population.spike_trains[i],
                        &population.spike_trains[j],
                    )?;
                    conn_matrix[(i, j)] = corr;
                    conn_matrix[(j, i)] = corr;
                }
            }
            Ok(conn_matrix)
        }
        "victor-purpura" => {
            let q = params.get("q").copied().unwrap_or(1.0);
            population.victor_purpura_matrix(q)
        }
        "van-rossum" => {
            let tau = params.get("tau").copied().unwrap_or(0.5);
            let trains = &population.spike_trains;
            crate::distances::van_rossum_matrix(trains, tau)
        }
        _ => Err(TdaError::InvalidParameter(format!(
            "Unknown connectivity method: {}",
            method
        ))),
    }
}

/// Compute topological features for spike train data.
///
/// Returns a feature vector including:
/// - Number of persistent features by dimension
/// - Average persistence by dimension
/// - Entropy of persistence diagram
///
/// # Arguments
/// * `population` - Neural population
/// * `q` - Victor-Purpura cost parameter
/// * `max_radius` - Maximum filtration radius
/// * `max_dimension` - Maximum homology dimension
///
/// # Returns
/// Feature vector for machine learning or statistical analysis
pub fn topological_features(
    population: &NeuralPopulation,
    q: f64,
    max_radius: f64,
    max_dimension: usize,
) -> Result<Vec<f64>> {
    let diagram = population.persistence_diagram_vp(q, max_radius, max_dimension)?;

    let mut features = Vec::new();

    // Count features by dimension
    for dim in 0..=max_dimension {
        let count = diagram.points_by_dimension(dim).len() as f64;
        features.push(count);
    }

    // Average persistence by dimension
    for dim in 0..=max_dimension {
        let points = diagram.points_by_dimension(dim);
        let avg_pers = if !points.is_empty() {
            points.iter().map(|(b, d)| d - b).sum::<f64>() / points.len() as f64
        } else {
            0.0
        };
        features.push(avg_pers);
    }

    // Persistence entropy
    let entropy = crate::persistence_diagram::persistence_entropy(&diagram);
    features.push(entropy);

    // Max persistence by dimension
    for dim in 0..=max_dimension {
        let points = diagram.points_by_dimension(dim);
        let max_pers = points
            .iter()
            .map(|(b, d)| d - b)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        features.push(max_pers);
    }

    Ok(features)
}

/// Analyze burst synchrony using topological methods.
///
/// Detects periods of synchronous bursting in neural populations.
///
/// # Arguments
/// * `population` - Neural population
/// * `burst_window` - Time window for defining bursts
/// * `sync_threshold` - Threshold for considering bursts synchronous
///
/// # Returns
/// Time points where synchronous bursts occur
pub fn burst_synchrony_analysis(
    population: &NeuralPopulation,
    burst_window: f64,
    sync_threshold: f64,
) -> Result<Vec<f64>> {
    // Find all spike times across population
    let mut all_spikes = Vec::new();
    for train in &population.spike_trains {
        all_spikes.extend(train.times.iter().copied());
    }
    all_spikes.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if all_spikes.is_empty() {
        return Ok(Vec::new());
    }

    // Identify burst times
    let mut burst_times = Vec::new();
    let mut i = 0;

    while i < all_spikes.len() {
        let burst_start = all_spikes[i];
        let burst_end = burst_start + burst_window;

        // Count spikes in this window across neurons
        let mut neurons_active = HashSet::new();

        for (neuron_idx, train) in population.spike_trains.iter().enumerate() {
            for &spike_time in &train.times {
                if spike_time >= burst_start && spike_time < burst_end {
                    neurons_active.insert(neuron_idx);
                }
            }
        }

        // Check if enough neurons participated
        let participation_ratio = neurons_active.len() as f64 / population.size() as f64;

        if participation_ratio >= sync_threshold {
            burst_times.push(burst_start);
        }

        // Move to next potential burst
        i += 1;
        while i < all_spikes.len() && all_spikes[i] < burst_end {
            i += 1;
        }
    }

    Ok(burst_times)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neural_population() {
        let train1 = SpikeTrain::new(vec![1.0, 2.0, 3.0]).unwrap();
        let train2 = SpikeTrain::new(vec![1.1, 2.1, 3.1]).unwrap();
        let train3 = SpikeTrain::new(vec![5.0, 6.0, 7.0]).unwrap();

        let pop = NeuralPopulation::new(vec![train1, train2, train3]).unwrap();
        assert_eq!(pop.size(), 3);
    }

    #[test]
    fn test_sliding_window() {
        let window = SlidingWindowAnalysis::new(1.0, 0.5, 0.0, 5.0).unwrap();

        let train = SpikeTrain::new(vec![0.5, 1.5, 2.5, 3.5, 4.5]).unwrap();
        let windowed = window.window_spike_train(&train, 1.0);

        // Should contain spikes in [1.0, 2.0)
        assert_eq!(windowed.len(), 1);
        assert!((windowed.times[0] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_spike_correlation() {
        let train1 = SpikeTrain::new(vec![1.0, 2.0, 3.0]).unwrap();
        let train2 = SpikeTrain::new(vec![1.0, 2.0, 3.0]).unwrap();

        let corr = compute_spike_correlation(&train1, &train2).unwrap();
        assert!(corr > 0.9); // Should be very similar
    }

    #[test]
    fn test_cell_assembly_detection() {
        // Create a simple population with correlated spike trains
        let train1 = SpikeTrain::new(vec![1.0, 2.0, 3.0]).unwrap();
        let train2 = SpikeTrain::new(vec![1.1, 2.1, 3.1]).unwrap();
        let train3 = SpikeTrain::new(vec![10.0, 11.0, 12.0]).unwrap();

        let pop = NeuralPopulation::new(vec![train1, train2, train3]).unwrap();

        let assemblies = detect_cell_assemblies(&pop, 0.5, 2).unwrap();

        // Should detect at least one assembly
        assert!(!assemblies.is_empty());
    }

    #[test]
    fn test_topological_features() {
        let train1 = SpikeTrain::new(vec![1.0, 2.0, 3.0]).unwrap();
        let train2 = SpikeTrain::new(vec![1.1, 2.1, 3.1]).unwrap();

        let pop = NeuralPopulation::new(vec![train1, train2]).unwrap();

        let features = topological_features(&pop, 1.0, 2.0, 1).unwrap();

        // Should return non-empty feature vector
        assert!(!features.is_empty());
    }

    #[test]
    fn test_burst_synchrony() {
        let train1 = SpikeTrain::new(vec![1.0, 1.1, 5.0]).unwrap();
        let train2 = SpikeTrain::new(vec![1.05, 5.1]).unwrap();
        let train3 = SpikeTrain::new(vec![1.08, 5.05]).unwrap();

        let pop = NeuralPopulation::new(vec![train1, train2, train3]).unwrap();

        let bursts = burst_synchrony_analysis(&pop, 0.2, 0.6).unwrap();

        // Should detect synchronous bursts around t=1.0 and t=5.0
        assert!(!bursts.is_empty());
    }
}
