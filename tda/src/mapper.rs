//! Mapper algorithm for topological data visualization.
//!
//! The Mapper algorithm creates a graph representation of high-dimensional data
//! by applying a filter function, covering the range, clustering within each cover
//! element, and building a nerve complex (graph) based on overlaps.
//!
//! This is particularly useful for visualizing and exploring the shape of data.

use crate::error::{Result, TdaError};
use nalgebra::DMatrix;
use petgraph::graph::Graph;
use petgraph::Undirected;
use std::collections::{HashMap, HashSet};

/// A filter function maps data points to real values or vectors.
pub type FilterFunction = Box<dyn Fn(&DMatrix<f64>, usize) -> Vec<f64> + Send + Sync>;

/// Clustering method for Mapper algorithm.
#[derive(Debug, Clone, Copy)]
pub enum ClusteringMethod {
    /// Single-linkage clustering with a distance threshold
    SingleLinkage { threshold: f64 },
    /// DBSCAN-like clustering
    DbscanLike { eps: f64, min_points: usize },
}

/// Cover strategy for the filter space.
#[derive(Debug, Clone)]
pub enum CoverStrategy {
    /// Uniform cover with fixed number of intervals and overlap percentage
    Uniform {
        num_intervals: usize,
        overlap_percent: f64,
    },
    /// Adaptive cover based on data density
    Adaptive {
        num_intervals: usize,
        overlap_percent: f64,
    },
}

/// Represents a node in the Mapper graph.
#[derive(Debug, Clone)]
pub struct MapperNode {
    /// Indices of data points in this cluster
    pub points: Vec<usize>,
    /// Representative value (e.g., mean filter value)
    pub representative_value: Vec<f64>,
    /// Size of the cluster
    pub size: usize,
}

/// The Mapper graph structure.
pub type MapperGraph = Graph<MapperNode, (), Undirected>;

/// Builder for configuring and running the Mapper algorithm.
pub struct MapperBuilder {
    filter: Option<FilterFunction>,
    cover: CoverStrategy,
    clustering: ClusteringMethod,
}

impl MapperBuilder {
    /// Create a new MapperBuilder with default settings.
    pub fn new() -> Self {
        MapperBuilder {
            filter: None,
            cover: CoverStrategy::Uniform {
                num_intervals: 10,
                overlap_percent: 0.5,
            },
            clustering: ClusteringMethod::SingleLinkage { threshold: 0.5 },
        }
    }

    /// Set the filter function.
    pub fn filter(mut self, filter: FilterFunction) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Set the cover strategy.
    pub fn cover(mut self, cover: CoverStrategy) -> Self {
        self.cover = cover;
        self
    }

    /// Set the clustering method.
    pub fn clustering(mut self, clustering: ClusteringMethod) -> Self {
        self.clustering = clustering;
        self
    }

    /// Build the Mapper graph from data and distance matrix.
    ///
    /// # Arguments
    /// * `data` - Data matrix where each row is a data point
    /// * `distance_matrix` - Precomputed distance matrix
    ///
    /// # Returns
    /// A Mapper graph where nodes are clusters and edges connect overlapping clusters
    pub fn build(
        self,
        data: &DMatrix<f64>,
        distance_matrix: &DMatrix<f64>,
    ) -> Result<MapperGraph> {
        let n = data.nrows();
        if n == 0 {
            return Err(TdaError::EmptyDataset);
        }

        let filter = self.filter.as_ref().ok_or_else(|| {
            TdaError::InvalidParameter("Filter function must be set".to_string())
        })?;

        // Step 1: Apply filter function to all points
        let filter_values: Vec<Vec<f64>> = (0..n).map(|i| filter(data, i)).collect();

        // Step 2: Create cover of the filter space
        let cover_elements = self.create_cover(&filter_values)?;

        // Step 3: Cluster within each cover element
        let mut clusters = Vec::new();
        for element in cover_elements {
            let sub_clusters = self.cluster_points(&element, distance_matrix)?;
            clusters.extend(sub_clusters);
        }

        // Step 4: Build the nerve complex (Mapper graph)
        let graph = self.build_nerve(&clusters, &filter_values)?;

        Ok(graph)
    }

    /// Create cover elements based on the cover strategy.
    fn create_cover(&self, filter_values: &[Vec<f64>]) -> Result<Vec<Vec<usize>>> {
        if filter_values.is_empty() {
            return Ok(Vec::new());
        }

        let filter_dim = filter_values[0].len();
        if filter_dim == 0 {
            return Err(TdaError::InvalidParameter(
                "Filter values must be non-empty".to_string(),
            ));
        }

        // For simplicity, handle 1D filters
        if filter_dim != 1 {
            return Err(TdaError::InvalidParameter(
                "Multi-dimensional filters not yet supported".to_string(),
            ));
        }

        match self.cover {
            CoverStrategy::Uniform {
                num_intervals,
                overlap_percent,
            } => self.uniform_cover_1d(filter_values, num_intervals, overlap_percent),
            CoverStrategy::Adaptive {
                num_intervals,
                overlap_percent,
            } => self.adaptive_cover_1d(filter_values, num_intervals, overlap_percent),
        }
    }

    /// Create uniform cover for 1D filter.
    fn uniform_cover_1d(
        &self,
        filter_values: &[Vec<f64>],
        num_intervals: usize,
        overlap_percent: f64,
    ) -> Result<Vec<Vec<usize>>> {
        let values: Vec<f64> = filter_values.iter().map(|v| v[0]).collect();

        let min_val = values
            .iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max_val = values
            .iter()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let range = max_val - min_val;
        if range == 0.0 {
            return Ok(vec![vec![0; values.len()]]);
        }

        let interval_width = range / (num_intervals as f64);
        let overlap = interval_width * overlap_percent;

        let mut cover_elements = Vec::new();

        for i in 0..num_intervals {
            let start = min_val + (i as f64) * interval_width - overlap;
            let end = min_val + ((i + 1) as f64) * interval_width + overlap;

            let points_in_interval: Vec<usize> = values
                .iter()
                .enumerate()
                .filter(|(_, &v)| v >= start && v <= end)
                .map(|(idx, _)| idx)
                .collect();

            if !points_in_interval.is_empty() {
                cover_elements.push(points_in_interval);
            }
        }

        Ok(cover_elements)
    }

    /// Create adaptive cover for 1D filter based on data density.
    fn adaptive_cover_1d(
        &self,
        filter_values: &[Vec<f64>],
        num_intervals: usize,
        overlap_percent: f64,
    ) -> Result<Vec<Vec<usize>>> {
        // For simplicity, use uniform cover (adaptive would require density estimation)
        self.uniform_cover_1d(filter_values, num_intervals, overlap_percent)
    }

    /// Cluster points using the specified clustering method.
    fn cluster_points(
        &self,
        points: &[usize],
        distance_matrix: &DMatrix<f64>,
    ) -> Result<Vec<Vec<usize>>> {
        if points.is_empty() {
            return Ok(Vec::new());
        }

        match self.clustering {
            ClusteringMethod::SingleLinkage { threshold } => {
                self.single_linkage_clustering(points, distance_matrix, threshold)
            }
            ClusteringMethod::DbscanLike { eps, min_points } => {
                self.dbscan_clustering(points, distance_matrix, eps, min_points)
            }
        }
    }

    /// Single-linkage hierarchical clustering.
    fn single_linkage_clustering(
        &self,
        points: &[usize],
        distance_matrix: &DMatrix<f64>,
        threshold: f64,
    ) -> Result<Vec<Vec<usize>>> {
        let n = points.len();
        let mut parent: Vec<usize> = (0..n).collect();

        fn find(parent: &mut [usize], x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        fn union(parent: &mut [usize], x: usize, y: usize) {
            let root_x = find(parent, x);
            let root_y = find(parent, y);
            if root_x != root_y {
                parent[root_x] = root_y;
            }
        }

        // Connect points within threshold distance
        for i in 0..n {
            for j in (i + 1)..n {
                let dist = distance_matrix[(points[i], points[j])];
                if dist <= threshold {
                    union(&mut parent, i, j);
                }
            }
        }

        // Group points by cluster
        let mut clusters: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..n {
            let root = find(&mut parent, i);
            clusters.entry(root).or_insert_with(Vec::new).push(points[i]);
        }

        Ok(clusters.into_values().collect())
    }

    /// DBSCAN-like clustering.
    fn dbscan_clustering(
        &self,
        points: &[usize],
        distance_matrix: &DMatrix<f64>,
        eps: f64,
        min_points: usize,
    ) -> Result<Vec<Vec<usize>>> {
        let n = points.len();
        let mut visited = vec![false; n];
        let mut clusters = Vec::new();

        for i in 0..n {
            if visited[i] {
                continue;
            }

            visited[i] = true;

            // Find neighbors
            let mut neighbors: Vec<usize> = Vec::new();
            for j in 0..n {
                if i != j && distance_matrix[(points[i], points[j])] <= eps {
                    neighbors.push(j);
                }
            }

            if neighbors.len() < min_points {
                continue; // Noise point
            }

            // Expand cluster
            let mut cluster = vec![points[i]];
            let mut queue = neighbors.clone();

            while let Some(idx) = queue.pop() {
                if visited[idx] {
                    continue;
                }

                visited[idx] = true;
                cluster.push(points[idx]);

                // Find neighbors of this point
                let mut idx_neighbors: Vec<usize> = Vec::new();
                for j in 0..n {
                    if idx != j && distance_matrix[(points[idx], points[j])] <= eps {
                        idx_neighbors.push(j);
                    }
                }

                if idx_neighbors.len() >= min_points {
                    queue.extend(idx_neighbors);
                }
            }

            clusters.push(cluster);
        }

        if clusters.is_empty() && !points.is_empty() {
            // If no clusters formed, treat all as one cluster
            clusters.push(points.to_vec());
        }

        Ok(clusters)
    }

    /// Build the nerve complex (Mapper graph) from clusters.
    fn build_nerve(
        &self,
        clusters: &[Vec<usize>],
        filter_values: &[Vec<f64>],
    ) -> Result<MapperGraph> {
        let mut graph = Graph::new_undirected();
        let mut node_indices = Vec::new();

        // Create nodes
        for cluster in clusters {
            if cluster.is_empty() {
                continue;
            }

            // Compute representative filter value (mean)
            let mut mean_filter = vec![0.0; filter_values[0].len()];
            for &idx in cluster {
                for (i, &val) in filter_values[idx].iter().enumerate() {
                    mean_filter[i] += val;
                }
            }
            for val in &mut mean_filter {
                *val /= cluster.len() as f64;
            }

            let node = MapperNode {
                points: cluster.clone(),
                representative_value: mean_filter,
                size: cluster.len(),
            };

            let node_idx = graph.add_node(node);
            node_indices.push(node_idx);
        }

        // Create edges between overlapping clusters
        for i in 0..clusters.len() {
            for j in (i + 1)..clusters.len() {
                let set_i: HashSet<_> = clusters[i].iter().copied().collect();
                let set_j: HashSet<_> = clusters[j].iter().copied().collect();

                if set_i.intersection(&set_j).count() > 0 {
                    graph.add_edge(node_indices[i], node_indices[j], ());
                }
            }
        }

        Ok(graph)
    }
}

impl Default for MapperBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Common filter functions for Mapper.
pub mod filters {
    use nalgebra::DMatrix;

    /// First principal component projection.
    pub fn first_principal_component() -> Box<dyn Fn(&DMatrix<f64>, usize) -> Vec<f64> + Send + Sync>
    {
        Box::new(|_data: &DMatrix<f64>, _idx: usize| {
            // Simplified: would need PCA implementation
            // For now, return first coordinate
            vec![_data[(_idx, 0)]]
        })
    }

    /// Coordinate projection onto a specific axis.
    pub fn coordinate_projection(
        axis: usize,
    ) -> Box<dyn Fn(&DMatrix<f64>, usize) -> Vec<f64> + Send + Sync> {
        Box::new(move |data: &DMatrix<f64>, idx: usize| vec![data[(idx, axis)]])
    }

    /// L2 norm (distance from origin).
    pub fn l2_norm() -> Box<dyn Fn(&DMatrix<f64>, usize) -> Vec<f64> + Send + Sync> {
        Box::new(|data: &DMatrix<f64>, idx: usize| {
            let row = data.row(idx);
            vec![row.norm()]
        })
    }

    /// Density-based filter (average distance to k nearest neighbors).
    pub fn density_filter(
        k: usize,
    ) -> Box<dyn Fn(&DMatrix<f64>, usize) -> Vec<f64> + Send + Sync> {
        Box::new(move |data: &DMatrix<f64>, idx: usize| {
            let n = data.nrows();
            let mut distances: Vec<f64> = Vec::new();

            for i in 0..n {
                if i != idx {
                    let diff = data.row(idx) - data.row(i);
                    distances.push(diff.norm());
                }
            }

            distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let k_neighbors = k.min(distances.len());
            let avg_dist = distances.iter().take(k_neighbors).sum::<f64>() / k_neighbors as f64;

            vec![avg_dist]
        })
    }

    /// Eccentricity (maximum distance to all other points).
    pub fn eccentricity() -> Box<dyn Fn(&DMatrix<f64>, usize) -> Vec<f64> + Send + Sync> {
        Box::new(|data: &DMatrix<f64>, idx: usize| {
            let n = data.nrows();
            let mut max_dist: f64 = 0.0;

            for i in 0..n {
                if i != idx {
                    let diff = data.row(idx) - data.row(i);
                    max_dist = max_dist.max(diff.norm());
                }
            }

            vec![max_dist]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapper_builder() {
        let builder = MapperBuilder::new();
        assert!(builder.filter.is_none());
    }

    #[test]
    fn test_coordinate_filter() {
        let data = DMatrix::from_row_slice(3, 2, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let filter = filters::coordinate_projection(0);
        let value = filter(&data, 1);

        assert_eq!(value, vec![3.0]);
    }

    #[test]
    fn test_l2_norm_filter() {
        let data = DMatrix::from_row_slice(2, 2, &[3.0, 4.0, 0.0, 0.0]);

        let filter = filters::l2_norm();
        let value = filter(&data, 0);

        assert!((value[0] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_mapper_simple() {
        use crate::distances::euclidean_distance_matrix;

        // Create simple 1D data
        let data = DMatrix::from_row_slice(5, 1, &[0.0, 1.0, 2.0, 3.0, 4.0]);
        let dist = euclidean_distance_matrix(&data).unwrap();

        let mapper = MapperBuilder::new()
            .filter(filters::coordinate_projection(0))
            .cover(CoverStrategy::Uniform {
                num_intervals: 2,
                overlap_percent: 0.3,
            })
            .clustering(ClusteringMethod::SingleLinkage { threshold: 1.5 });

        let graph = mapper.build(&data, &dist).unwrap();

        // Should have at least one node
        assert!(graph.node_count() > 0);
    }
}
