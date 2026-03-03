//! KD-tree spatial indexing for fast neighbor queries on star positions.

use crate::analysis::loader::StarPoint;
use kiddo::SquaredEuclidean;
use rayon::prelude::*;

/// 3D KD-tree over star positions (parsecs).
/// Bucket size 512 handles quantized datasets (STR3 i16) where many stars
/// share the same coordinate on one axis.
pub type StarTree = kiddo::float::kdtree::KdTree<f64, u64, 3, 512, u32>;

/// Build a KD-tree from star positions.
pub fn build_kdtree(stars: &[StarPoint]) -> StarTree {
    let mut tree = StarTree::with_capacity(stars.len());
    for (i, s) in stars.iter().enumerate() {
        tree.add(&s.pos(), i as u64);
    }
    tree
}

/// Result of a neighbor query.
#[derive(Debug, Clone)]
pub struct Neighbor {
    pub index: usize,
    pub distance_pc: f64,
}

/// Find the k nearest neighbors to a point.
pub fn nearest_neighbors(tree: &StarTree, point: &[f64; 3], k: usize) -> Vec<Neighbor> {
    tree.nearest_n::<SquaredEuclidean>(point, k)
        .into_iter()
        .map(|n| Neighbor {
            index: n.item as usize,
            distance_pc: (n.distance as f64).sqrt(),
        })
        .collect()
}

/// Find all stars within a radius (parsecs) of a point.
pub fn within_radius(tree: &StarTree, point: &[f64; 3], radius: f64) -> Vec<Neighbor> {
    let r_sq = radius * radius;
    tree.within::<SquaredEuclidean>(point, r_sq)
        .into_iter()
        .map(|n| Neighbor {
            index: n.item as usize,
            distance_pc: (n.distance as f64).sqrt(),
        })
        .collect()
}

/// Compute local density (neighbor count within radius) for every star, in parallel.
pub fn local_density(tree: &StarTree, stars: &[StarPoint], radius: f64) -> Vec<u32> {
    let r_sq = radius * radius;
    stars
        .par_iter()
        .map(|s| {
            // count includes the star itself, so subtract 1
            let count = tree.within::<SquaredEuclidean>(&s.pos(), r_sq).len();
            count.saturating_sub(1) as u32
        })
        .collect()
}
