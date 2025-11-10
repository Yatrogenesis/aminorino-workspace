//! Network topology and connectivity pattern generation.
//!
//! This module implements various connectivity patterns including random,
//! small-world, scale-free, and spatial networks.

use crate::error::{NeuralDynamicsError, Result};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use rand::Rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Connection pattern for projections between populations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionPattern {
    /// All neurons in source connect to all in target
    AllToAll,
    /// One-to-one mapping (requires equal sizes)
    OneToOne,
    /// Each pair connects with fixed probability
    FixedProbability(f64),
    /// Each target receives exactly n inputs
    FixedNumber(usize),
    /// Small-world network (Watts-Strogatz)
    SmallWorld { k: usize, p: f64 },
    /// Scale-free network (Barabási-Albert)
    ScaleFree { m: usize },
    /// Gaussian distance-dependent connectivity
    Gaussian { sigma: f64 },
}

impl ConnectionPattern {
    /// Generate list of connections (source_idx, target_idx) based on pattern.
    pub fn generate<R: Rng>(
        &self,
        source_size: usize,
        target_size: usize,
        rng: &mut R,
    ) -> Result<Vec<(usize, usize)>> {
        match self {
            ConnectionPattern::AllToAll => {
                Ok(all_to_all_connections(source_size, target_size))
            }
            ConnectionPattern::OneToOne => {
                one_to_one_connections(source_size, target_size)
            }
            ConnectionPattern::FixedProbability(p) => {
                fixed_probability_connections(source_size, target_size, *p, rng)
            }
            ConnectionPattern::FixedNumber(n) => {
                fixed_number_connections(source_size, target_size, *n, rng)
            }
            ConnectionPattern::SmallWorld { k, p } => {
                small_world_connections(source_size, *k, *p, rng)
            }
            ConnectionPattern::ScaleFree { m } => {
                scale_free_connections(source_size, *m, rng)
            }
            ConnectionPattern::Gaussian { sigma } => {
                gaussian_connections(source_size, target_size, *sigma, rng)
            }
        }
    }
}

/// Generate all-to-all connections.
fn all_to_all_connections(source_size: usize, target_size: usize) -> Vec<(usize, usize)> {
    let mut connections = Vec::with_capacity(source_size * target_size);
    for i in 0..source_size {
        for j in 0..target_size {
            connections.push((i, j));
        }
    }
    connections
}

/// Generate one-to-one connections.
fn one_to_one_connections(source_size: usize, target_size: usize) -> Result<Vec<(usize, usize)>> {
    if source_size != target_size {
        return Err(NeuralDynamicsError::ConnectivityError {
            reason: format!(
                "OneToOne requires equal population sizes, got {} and {}",
                source_size, target_size
            ),
        });
    }

    Ok((0..source_size).map(|i| (i, i)).collect())
}

/// Generate connections with fixed probability.
fn fixed_probability_connections<R: Rng>(
    source_size: usize,
    target_size: usize,
    probability: f64,
    rng: &mut R,
) -> Result<Vec<(usize, usize)>> {
    if probability < 0.0 || probability > 1.0 {
        return Err(NeuralDynamicsError::InvalidParameter {
            parameter: "probability".to_string(),
            value: probability,
            reason: "must be in [0, 1]".to_string(),
        });
    }

    let mut connections = Vec::new();
    for i in 0..source_size {
        for j in 0..target_size {
            if rng.gen::<f64>() < probability {
                connections.push((i, j));
            }
        }
    }

    Ok(connections)
}

/// Generate connections with fixed number of inputs per target neuron.
fn fixed_number_connections<R: Rng>(
    source_size: usize,
    target_size: usize,
    n_inputs: usize,
    rng: &mut R,
) -> Result<Vec<(usize, usize)>> {
    if n_inputs > source_size {
        return Err(NeuralDynamicsError::ConnectivityError {
            reason: format!(
                "Cannot have {} inputs from population of size {}",
                n_inputs, source_size
            ),
        });
    }

    let mut connections = Vec::new();
    let mut source_indices: Vec<usize> = (0..source_size).collect();

    for target in 0..target_size {
        source_indices.shuffle(rng);
        for &source in source_indices.iter().take(n_inputs) {
            connections.push((source, target));
        }
    }

    Ok(connections)
}

/// Generate small-world network (Watts-Strogatz model).
///
/// # Arguments
///
/// * `n` - Number of nodes
/// * `k` - Each node is connected to k nearest neighbors in ring topology
/// * `p` - Probability of rewiring each edge
/// * `rng` - Random number generator
///
/// # Returns
///
/// List of connections forming a small-world network
pub fn small_world_connections<R: Rng>(
    n: usize,
    k: usize,
    p: f64,
    rng: &mut R,
) -> Result<Vec<(usize, usize)>> {
    if k >= n {
        return Err(NeuralDynamicsError::ConnectivityError {
            reason: "k must be less than n".to_string(),
        });
    }

    if k % 2 != 0 {
        return Err(NeuralDynamicsError::ConnectivityError {
            reason: "k must be even".to_string(),
        });
    }

    // Start with ring lattice
    let mut edges: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..n {
        for j in 1..=k / 2 {
            let neighbor = (i + j) % n;
            edges.insert((i.min(neighbor), i.max(neighbor)));
        }
    }

    // Rewire with probability p
    let edges_to_rewire: Vec<_> = edges.iter().cloned().collect();

    for (u, v) in edges_to_rewire {
        if rng.gen::<f64>() < p {
            edges.remove(&(u, v));

            // Find new target that doesn't create self-loop or duplicate
            let mut new_target = rng.gen_range(0..n);
            let mut attempts = 0;
            while new_target == u || edges.contains(&(u.min(new_target), u.max(new_target))) {
                new_target = rng.gen_range(0..n);
                attempts += 1;
                if attempts > 100 {
                    // Give up and keep original edge
                    edges.insert((u, v));
                    break;
                }
            }

            if attempts <= 100 {
                edges.insert((u.min(new_target), u.max(new_target)));
            }
        }
    }

    // Convert to directed connections
    let mut connections = Vec::new();
    for (u, v) in edges {
        connections.push((u, v));
        connections.push((v, u));
    }

    Ok(connections)
}

/// Generate scale-free network (Barabási-Albert model).
///
/// # Arguments
///
/// * `n` - Final number of nodes
/// * `m` - Number of edges to attach from new node to existing nodes
/// * `rng` - Random number generator
pub fn scale_free_connections<R: Rng>(
    n: usize,
    m: usize,
    rng: &mut R,
) -> Result<Vec<(usize, usize)>> {
    if m >= n {
        return Err(NeuralDynamicsError::ConnectivityError {
            reason: "m must be less than n".to_string(),
        });
    }

    if m == 0 {
        return Ok(Vec::new());
    }

    let mut graph = Graph::<(), (), Undirected>::new_undirected();
    let mut nodes: Vec<NodeIndex> = Vec::new();
    let mut degrees: Vec<usize> = Vec::new();

    // Start with m+1 nodes in a complete graph
    for _ in 0..=m {
        nodes.push(graph.add_node(()));
        degrees.push(0);
    }

    for i in 0..=m {
        for j in i + 1..=m {
            graph.add_edge(nodes[i], nodes[j], ());
            degrees[i] += 1;
            degrees[j] += 1;
        }
    }

    // Add remaining nodes with preferential attachment
    for _ in (m + 1)..n {
        let new_node = graph.add_node(());
        nodes.push(new_node);
        degrees.push(0);

        let total_degree: usize = degrees.iter().sum();
        let mut targets = HashSet::new();

        // Select m targets using preferential attachment
        while targets.len() < m {
            let threshold = rng.gen::<f64>() * total_degree as f64;
            let mut cumulative = 0.0;

            for (i, &deg) in degrees.iter().enumerate() {
                cumulative += deg as f64;
                if cumulative >= threshold && !targets.contains(&i) {
                    targets.insert(i);
                    break;
                }
            }
        }

        // Add edges to selected targets
        for &target in &targets {
            graph.add_edge(new_node, nodes[target], ());
            degrees[nodes.len() - 1] += 1;
            degrees[target] += 1;
        }
    }

    // Extract connections from graph edges
    let mut connections = Vec::new();
    for node_a in graph.node_indices() {
        for node_b in graph.neighbors(node_a) {
            connections.push((node_a.index(), node_b.index()));
        }
    }

    Ok(connections)
}

/// Generate Gaussian distance-dependent connections.
fn gaussian_connections<R: Rng>(
    source_size: usize,
    target_size: usize,
    sigma: f64,
    rng: &mut R,
) -> Result<Vec<(usize, usize)>> {
    if sigma <= 0.0 {
        return Err(NeuralDynamicsError::InvalidParameter {
            parameter: "sigma".to_string(),
            value: sigma,
            reason: "must be positive".to_string(),
        });
    }

    // Assume 1D arrangement for simplicity
    let mut connections = Vec::new();

    for i in 0..source_size {
        for j in 0..target_size {
            let distance = ((i as f64 / source_size as f64) - (j as f64 / target_size as f64)).abs();
            let probability = (-distance * distance / (2.0 * sigma * sigma)).exp();

            if rng.gen::<f64>() < probability {
                connections.push((i, j));
            }
        }
    }

    Ok(connections)
}


/// Calculate network statistics.
pub fn network_statistics(connections: &[(usize, usize)], n_nodes: usize) -> NetworkStats {
    let n_connections = connections.len();

    // Calculate degree distribution
    let mut in_degrees = vec![0; n_nodes];
    let mut out_degrees = vec![0; n_nodes];

    for &(source, target) in connections {
        if source < n_nodes && target < n_nodes {
            out_degrees[source] += 1;
            in_degrees[target] += 1;
        }
    }

    let mean_in_degree = in_degrees.iter().sum::<usize>() as f64 / n_nodes as f64;
    let mean_out_degree = out_degrees.iter().sum::<usize>() as f64 / n_nodes as f64;

    NetworkStats {
        n_nodes,
        n_connections,
        mean_in_degree,
        mean_out_degree,
        max_in_degree: *in_degrees.iter().max().unwrap_or(&0),
        max_out_degree: *out_degrees.iter().max().unwrap_or(&0),
    }
}

/// Network statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub n_nodes: usize,
    pub n_connections: usize,
    pub mean_in_degree: f64,
    pub mean_out_degree: f64,
    pub max_in_degree: usize,
    pub max_out_degree: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_all_to_all() {
        let connections = all_to_all_connections(3, 2);
        assert_eq!(connections.len(), 6);
    }

    #[test]
    fn test_one_to_one() {
        let connections = one_to_one_connections(5, 5).unwrap();
        assert_eq!(connections.len(), 5);
        assert_eq!(connections[0], (0, 0));
        assert_eq!(connections[4], (4, 4));

        // Unequal sizes should fail
        assert!(one_to_one_connections(3, 5).is_err());
    }

    #[test]
    fn test_fixed_probability() {
        let mut rng = rand::thread_rng();
        let connections = fixed_probability_connections(10, 10, 0.5, &mut rng).unwrap();

        // Should have approximately 50 connections (10*10*0.5)
        assert!(connections.len() > 30 && connections.len() < 70);
    }

    #[test]
    fn test_fixed_number() {
        let mut rng = rand::thread_rng();
        let connections = fixed_number_connections(20, 10, 5, &mut rng).unwrap();

        // Each of 10 targets gets 5 inputs
        assert_eq!(connections.len(), 50);
    }

    #[test]
    fn test_small_world() {
        let mut rng = rand::thread_rng();
        let connections = small_world_connections(20, 4, 0.3, &mut rng).unwrap();

        // Should have connections (bidirectional from ring)
        assert!(!connections.is_empty());
    }

    #[test]
    fn test_scale_free() {
        let mut rng = rand::thread_rng();
        let connections = scale_free_connections(50, 3, &mut rng).unwrap();

        // Should have connections
        assert!(!connections.is_empty());

        // Check that connections form a valid graph
        let stats = network_statistics(&connections, 50);
        assert_eq!(stats.n_nodes, 50);
        assert!(stats.mean_in_degree > 0.0);
    }

    #[test]
    fn test_gaussian_connections() {
        let mut rng = rand::thread_rng();
        let connections = gaussian_connections(20, 20, 0.2, &mut rng).unwrap();

        // Should favor nearby connections
        assert!(!connections.is_empty());
    }

    #[test]
    fn test_network_statistics() {
        let connections = vec![(0, 1), (0, 2), (1, 2), (2, 3)];
        let stats = network_statistics(&connections, 4);

        assert_eq!(stats.n_nodes, 4);
        assert_eq!(stats.n_connections, 4);
        assert_relative_eq!(stats.mean_out_degree, 1.0);
    }


    #[test]
    fn test_connection_pattern_generate() {
        let mut rng = rand::thread_rng();

        let pattern = ConnectionPattern::AllToAll;
        let connections = pattern.generate(3, 2, &mut rng).unwrap();
        assert_eq!(connections.len(), 6);

        let pattern = ConnectionPattern::OneToOne;
        let connections = pattern.generate(4, 4, &mut rng).unwrap();
        assert_eq!(connections.len(), 4);

        let pattern = ConnectionPattern::FixedProbability(1.0);
        let connections = pattern.generate(2, 3, &mut rng).unwrap();
        assert_eq!(connections.len(), 6); // p=1.0 means all connections
    }
}
