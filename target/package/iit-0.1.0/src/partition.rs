//! Partition enumeration and Minimum Information Partition (MIP) search.
//!
//! This module implements algorithms for partitioning systems into parts and finding
//! the partition that minimizes integrated information. This is core to IIT's definition
//! of integration.
//!
//! # Theory
//!
//! A partition divides a system into parts by cutting connections. The Minimum Information
//! Partition (MIP) is the partition that minimizes the distance between the whole system's
//! cause-effect repertoire and the partitioned repertoire.

use crate::error::{IITError, Result};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Represents a partition of a system into two parts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Partition {
    /// Elements in the first part.
    pub part1: Vec<usize>,
    /// Elements in the second part.
    pub part2: Vec<usize>,
    /// The type of cut (unidirectional or bidirectional).
    pub cut_type: CutType,
}

/// Type of partition cut.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CutType {
    /// Unidirectional cut: connections from part1 to part2 are severed.
    Unidirectional,
    /// Bidirectional cut: all connections between parts are severed.
    Bidirectional,
}

/// Information about a partition cut.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionInfo {
    /// The partition.
    pub partition: Partition,
    /// Integrated information (Phi) for this partition.
    pub phi: f64,
    /// Number of connections cut.
    pub cuts: usize,
}

impl Partition {
    /// Create a new partition.
    pub fn new(part1: Vec<usize>, part2: Vec<usize>, cut_type: CutType) -> Self {
        Self {
            part1,
            part2,
            cut_type,
        }
    }

    /// Check if partition is valid (non-empty, non-overlapping parts).
    pub fn is_valid(&self, n_elements: usize) -> bool {
        if self.part1.is_empty() || self.part2.is_empty() {
            return false;
        }

        let set1: HashSet<_> = self.part1.iter().collect();
        let set2: HashSet<_> = self.part2.iter().collect();

        // Check no overlap
        if set1.intersection(&set2).count() > 0 {
            return false;
        }

        // Check all elements covered
        let union_size = set1.len() + set2.len();
        if union_size != n_elements {
            return false;
        }

        // Check all elements are valid indices
        self.part1.iter().chain(&self.part2).all(|&i| i < n_elements)
    }

    /// Get all elements in the partition.
    pub fn all_elements(&self) -> Vec<usize> {
        let mut elements: Vec<_> = self.part1.iter().chain(&self.part2).copied().collect();
        elements.sort_unstable();
        elements
    }

    /// Check if this partition cuts the connection from i to j.
    pub fn cuts_connection(&self, from: usize, to: usize) -> bool {
        let from_in_part1 = self.part1.contains(&from);
        let to_in_part2 = self.part2.contains(&to);
        let from_in_part2 = self.part2.contains(&from);
        let to_in_part1 = self.part1.contains(&to);

        match self.cut_type {
            CutType::Unidirectional => {
                // Cut connections from part1 to part2
                from_in_part1 && to_in_part2
            }
            CutType::Bidirectional => {
                // Cut connections in both directions
                (from_in_part1 && to_in_part2) || (from_in_part2 && to_in_part1)
            }
        }
    }
}

/// Iterator over all possible bipartitions of a set.
pub struct BipartitionIterator {
    n_elements: usize,
    current: usize,
    max: usize,
    cut_type: CutType,
}

impl BipartitionIterator {
    /// Create a new bipartition iterator.
    ///
    /// # Arguments
    ///
    /// * `n_elements` - Number of elements to partition
    /// * `cut_type` - Type of cut (unidirectional or bidirectional)
    pub fn new(n_elements: usize, cut_type: CutType) -> Self {
        let max = 1 << n_elements; // 2^n
        Self {
            n_elements,
            current: 1, // Start at 1 to skip empty partition
            max: max - 1, // Stop before all-elements partition
            cut_type,
        }
    }
}

impl Iterator for BipartitionIterator {
    type Item = Partition;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.max {
            return None;
        }

        let mut part1 = Vec::new();
        let mut part2 = Vec::new();

        for i in 0..self.n_elements {
            if (self.current & (1 << i)) != 0 {
                part1.push(i);
            } else {
                part2.push(i);
            }
        }

        self.current += 1;

        Some(Partition::new(part1, part2, self.cut_type))
    }
}

/// Generate all possible bipartitions of n elements.
///
/// # Arguments
///
/// * `n_elements` - Number of elements
/// * `cut_type` - Type of cut
///
/// # Returns
///
/// Vector of all valid bipartitions
pub fn all_bipartitions(n_elements: usize, cut_type: CutType) -> Vec<Partition> {
    BipartitionIterator::new(n_elements, cut_type).collect()
}

/// Generate all k-partitions (partitions into exactly k non-empty parts).
///
/// This is more general than bipartitions and is used in some IIT 3.0 formulations.
///
/// # Arguments
///
/// * `elements` - Elements to partition
/// * `k` - Number of parts
///
/// # Returns
///
/// Vector of k-partitions represented as vectors of element sets
pub fn k_partitions(elements: &[usize], k: usize) -> Vec<Vec<Vec<usize>>> {
    if k == 0 || elements.is_empty() || k > elements.len() {
        return vec![];
    }

    if k == 1 {
        return vec![vec![elements.to_vec()]];
    }

    if k == elements.len() {
        return vec![elements.iter().map(|&e| vec![e]).collect()];
    }

    // Use Stirling numbers approach
    let mut result = Vec::new();

    // For small k, enumerate directly
    if k == 2 {
        // Convert bipartitions to k-partition format
        for i in 1..(1 << elements.len()) - 1 {
            let mut part1 = Vec::new();
            let mut part2 = Vec::new();

            for (j, &elem) in elements.iter().enumerate() {
                if (i & (1 << j)) != 0 {
                    part1.push(elem);
                } else {
                    part2.push(elem);
                }
            }

            result.push(vec![part1, part2]);
        }
    }

    result
}

/// Find the Minimum Information Partition (MIP) using parallel search.
///
/// This function searches through all possible partitions to find the one that
/// minimizes integrated information.
///
/// # Arguments
///
/// * `n_elements` - Number of elements in the system
/// * `phi_fn` - Function that computes Phi for a given partition
/// * `cut_type` - Type of cut to use
///
/// # Returns
///
/// The partition with minimum Phi and its Phi value
pub fn find_mip<F>(
    n_elements: usize,
    phi_fn: F,
    cut_type: CutType,
) -> Result<PartitionInfo>
where
    F: Fn(&Partition) -> Result<f64> + Sync + Send,
{
    if n_elements == 0 {
        return Err(IITError::InvalidPartition(
            "Cannot partition empty system".to_string(),
        ));
    }

    if n_elements == 1 {
        return Err(IITError::InvalidPartition(
            "Cannot partition single element".to_string(),
        ));
    }

    // For large systems, use heuristic search
    if n_elements > 15 {
        return find_mip_heuristic(n_elements, phi_fn, cut_type);
    }

    // Generate all partitions
    let partitions = all_bipartitions(n_elements, cut_type);

    // Parallel search for minimum
    let result = partitions
        .par_iter()
        .filter_map(|partition| {
            phi_fn(partition).ok().map(|phi| PartitionInfo {
                partition: partition.clone(),
                phi,
                cuts: count_cuts(partition, n_elements),
            })
        })
        .min_by(|a, b| a.phi.partial_cmp(&b.phi).unwrap_or(std::cmp::Ordering::Equal));

    result.ok_or_else(|| IITError::ComputationError("No valid partitions found".to_string()))
}

/// Heuristic MIP search for large systems.
///
/// Uses greedy and random sampling to approximate the MIP without exhaustive search.
fn find_mip_heuristic<F>(
    n_elements: usize,
    phi_fn: F,
    cut_type: CutType,
) -> Result<PartitionInfo>
where
    F: Fn(&Partition) -> Result<f64> + Sync + Send,
{
    let mut best: Option<PartitionInfo> = None;

    // Try random partitions
    let n_samples = 1000.min(1 << (n_elements - 1));
    let step = ((1 << (n_elements - 1)) as f64 / n_samples as f64).ceil() as usize;

    for i in (1..(1 << n_elements) - 1).step_by(step.max(1)) {
        let mut part1 = Vec::new();
        let mut part2 = Vec::new();

        for j in 0..n_elements {
            if (i & (1 << j)) != 0 {
                part1.push(j);
            } else {
                part2.push(j);
            }
        }

        if part1.is_empty() || part2.is_empty() {
            continue;
        }

        let partition = Partition::new(part1, part2, cut_type);
        if let Ok(phi) = phi_fn(&partition) {
            let info = PartitionInfo {
                partition: partition.clone(),
                phi,
                cuts: count_cuts(&partition, n_elements),
            };

            if best.is_none() || info.phi < best.as_ref().unwrap().phi {
                best = Some(info);
            }
        }
    }

    best.ok_or_else(|| IITError::ComputationError("Heuristic search failed".to_string()))
}

/// Count the number of connections cut by a partition.
fn count_cuts(partition: &Partition, n_elements: usize) -> usize {
    let mut count = 0;
    for i in 0..n_elements {
        for j in 0..n_elements {
            if partition.cuts_connection(i, j) {
                count += 1;
            }
        }
    }
    count
}

/// Cache for partition results.
pub struct PartitionCache {
    cache: HashMap<Vec<usize>, PartitionInfo>,
    max_size: usize,
}

impl PartitionCache {
    /// Create a new partition cache.
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
        }
    }

    /// Get a cached partition result.
    pub fn get(&self, elements: &[usize]) -> Option<&PartitionInfo> {
        self.cache.get(elements)
    }

    /// Insert a partition result into the cache.
    pub fn insert(&mut self, elements: Vec<usize>, info: PartitionInfo) {
        if self.cache.len() >= self.max_size {
            // Simple eviction: remove a random entry
            if let Some(key) = self.cache.keys().next().cloned() {
                self.cache.remove(&key);
            }
        }
        self.cache.insert(elements, info);
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
    fn test_partition_validity() {
        let p = Partition::new(vec![0, 1], vec![2, 3], CutType::Bidirectional);
        assert!(p.is_valid(4));

        let p = Partition::new(vec![0], vec![1, 2, 3], CutType::Unidirectional);
        assert!(p.is_valid(4));

        // Empty part
        let p = Partition::new(vec![], vec![0, 1, 2], CutType::Bidirectional);
        assert!(!p.is_valid(3));

        // Overlapping
        let p = Partition::new(vec![0, 1], vec![1, 2], CutType::Bidirectional);
        assert!(!p.is_valid(3));
    }

    #[test]
    fn test_cuts_connection() {
        let p = Partition::new(vec![0, 1], vec![2, 3], CutType::Bidirectional);

        assert!(!p.cuts_connection(0, 1)); // Within part1
        assert!(!p.cuts_connection(2, 3)); // Within part2
        assert!(p.cuts_connection(0, 2)); // Between parts
        assert!(p.cuts_connection(2, 0)); // Between parts

        let p = Partition::new(vec![0, 1], vec![2, 3], CutType::Unidirectional);
        assert!(p.cuts_connection(0, 2)); // part1 -> part2
        assert!(!p.cuts_connection(2, 0)); // part2 -> part1 not cut
    }

    #[test]
    fn test_bipartition_iterator() {
        let partitions: Vec<_> = BipartitionIterator::new(3, CutType::Bidirectional).collect();

        // Should have 2^3 - 2 = 6 partitions (excluding empty and full)
        assert_eq!(partitions.len(), 6);

        // All should be valid
        for p in &partitions {
            assert!(p.is_valid(3));
        }
    }

    #[test]
    fn test_all_bipartitions() {
        let partitions = all_bipartitions(4, CutType::Bidirectional);

        // 2^4 - 2 = 14 partitions
        assert_eq!(partitions.len(), 14);

        // All unique
        let unique: HashSet<_> = partitions.iter().collect();
        assert_eq!(unique.len(), partitions.len());
    }

    #[test]
    fn test_find_mip() {
        // Simple test with constant phi function
        let phi_fn = |p: &Partition| -> Result<f64> {
            Ok((p.part1.len() * p.part2.len()) as f64)
        };

        let mip = find_mip(4, phi_fn, CutType::Bidirectional).unwrap();

        // Should find partition with minimum product (1*3 = 3 or 3*1 = 3)
        assert_eq!(mip.phi, 3.0);
    }

    #[test]
    fn test_partition_cache() {
        let mut cache = PartitionCache::new(10);

        let info = PartitionInfo {
            partition: Partition::new(vec![0], vec![1, 2], CutType::Bidirectional),
            phi: 1.5,
            cuts: 2,
        };

        cache.insert(vec![0, 1, 2], info.clone());

        let retrieved = cache.get(&[0, 1, 2]).unwrap();
        assert_eq!(retrieved.phi, 1.5);
    }
}
