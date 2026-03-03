//! DBSCAN clustering with KD-tree acceleration and rayon parallelism.

use crate::analysis::loader::StarPoint;
use crate::analysis::spatial::StarTree;
use kiddo::SquaredEuclidean;
use rayon::prelude::*;

/// Label for a star's cluster assignment.
const NOISE: i32 = -1;
const UNVISITED: i32 = -2;

/// Result of DBSCAN clustering.
#[derive(Debug)]
pub struct ClusterResult {
    /// Cluster label per star: -1 = noise, 0.. = cluster id.
    pub labels: Vec<i32>,
    /// Number of clusters found (not counting noise).
    pub num_clusters: usize,
}

impl ClusterResult {
    /// Return cluster summaries sorted by size (largest first).
    pub fn summaries(&self, stars: &[StarPoint]) -> Vec<ClusterSummary> {
        let mut clusters: std::collections::HashMap<i32, Vec<usize>> =
            std::collections::HashMap::new();
        for (i, &label) in self.labels.iter().enumerate() {
            if label >= 0 {
                clusters.entry(label).or_default().push(i);
            }
        }

        let mut summaries: Vec<ClusterSummary> = clusters
            .into_iter()
            .map(|(id, members)| {
                let n = members.len();
                let (mut cx, mut cy, mut cz) = (0.0, 0.0, 0.0);
                for &i in &members {
                    cx += stars[i].x;
                    cy += stars[i].y;
                    cz += stars[i].z;
                }
                cx /= n as f64;
                cy /= n as f64;
                cz /= n as f64;

                // Spread = RMS distance from centroid
                let spread: f64 = members
                    .iter()
                    .map(|&i| {
                        let dx = stars[i].x - cx;
                        let dy = stars[i].y - cy;
                        let dz = stars[i].z - cz;
                        dx * dx + dy * dy + dz * dz
                    })
                    .sum::<f64>()
                    / n as f64;

                ClusterSummary {
                    id,
                    size: n,
                    centroid: [cx, cy, cz],
                    spread_pc: spread.sqrt(),
                }
            })
            .collect();

        summaries.sort_by(|a, b| b.size.cmp(&a.size));
        summaries
    }

    /// Count of stars labeled as noise.
    pub fn noise_count(&self) -> usize {
        self.labels.iter().filter(|&&l| l == NOISE).count()
    }
}

/// Summary statistics for a single cluster.
#[derive(Debug)]
pub struct ClusterSummary {
    pub id: i32,
    pub size: usize,
    pub centroid: [f64; 3],
    pub spread_pc: f64,
}

/// DBSCAN clustering on 3D positions using a pre-built KD-tree.
///
/// - `eps`: neighborhood radius in parsecs
/// - `min_points`: minimum neighbors to form a core point
///
/// The neighborhood computation is parallelized with rayon.
pub fn dbscan(tree: &StarTree, stars: &[StarPoint], eps: f64, min_points: usize) -> ClusterResult {
    let n = stars.len();
    let eps_sq = eps * eps;

    // Phase 1: compute neighborhoods in parallel
    let neighborhoods: Vec<Vec<usize>> = stars
        .par_iter()
        .map(|s| {
            tree.within::<SquaredEuclidean>(&s.pos(), eps_sq)
                .into_iter()
                .map(|nb| nb.item as usize)
                .collect()
        })
        .collect();

    // Phase 2: sequential label assignment (DBSCAN core loop)
    let mut labels = vec![UNVISITED; n];
    let mut cluster_id: i32 = 0;

    for i in 0..n {
        if labels[i] != UNVISITED {
            continue;
        }

        let neighbors = &neighborhoods[i];
        if neighbors.len() < min_points {
            labels[i] = NOISE;
            continue;
        }

        // Start a new cluster
        labels[i] = cluster_id;
        let mut queue: Vec<usize> = neighbors.clone();
        let mut qi = 0;

        while qi < queue.len() {
            let j = queue[qi];
            qi += 1;

            if labels[j] == NOISE {
                labels[j] = cluster_id;
            }
            if labels[j] != UNVISITED {
                continue;
            }

            labels[j] = cluster_id;
            let j_neighbors = &neighborhoods[j];
            if j_neighbors.len() >= min_points {
                for &k in j_neighbors {
                    if labels[k] == UNVISITED || labels[k] == NOISE {
                        queue.push(k);
                    }
                }
            }
        }

        cluster_id += 1;
    }

    ClusterResult {
        labels,
        num_clusters: cluster_id as usize,
    }
}
