//! Stellar stream detection via velocity-space clustering.
//!
//! Stars born in the same molecular cloud retain similar velocities even as
//! they disperse spatially. By clustering in velocity space (vx, vy, vz),
//! we find co-moving groups — potential stellar streams or dissolving clusters.

use crate::analysis::loader::StarPoint;
use kiddo::{KdTree, SquaredEuclidean};
use rayon::prelude::*;

/// A detected co-moving stellar group / stream candidate.
#[derive(Debug)]
pub struct Stream {
    pub id: usize,
    /// Indices of member stars.
    pub members: Vec<usize>,
    /// Mean velocity (km/s).
    pub mean_vel_kms: [f64; 3],
    /// Velocity dispersion (km/s).
    pub vel_dispersion_kms: f64,
    /// Mean position (pc).
    pub mean_pos_pc: [f64; 3],
    /// Spatial spread (pc).
    pub spatial_spread_pc: f64,
}

/// Detect stellar streams by DBSCAN in velocity space.
///
/// - `vel_eps_kms`: velocity-space neighborhood radius in km/s
/// - `min_members`: minimum stars to form a stream
pub fn detect_streams(
    stars: &[StarPoint],
    vel_eps_kms: f64,
    min_members: usize,
) -> Vec<Stream> {
    if stars.is_empty() {
        return vec![];
    }

    // Build KD-tree in velocity space (km/s)
    let mut vel_tree: KdTree<f64, 3> = KdTree::new();
    for (i, s) in stars.iter().enumerate() {
        vel_tree.add(&s.vel_kms(), i as u64);
    }

    let eps_sq = vel_eps_kms * vel_eps_kms;

    // Parallel neighborhood computation
    let neighborhoods: Vec<Vec<usize>> = stars
        .par_iter()
        .map(|s| {
            vel_tree
                .within::<SquaredEuclidean>(&s.vel_kms(), eps_sq)
                .into_iter()
                .map(|nb| nb.item as usize)
                .collect()
        })
        .collect();

    // DBSCAN label assignment
    const UNVISITED: i32 = -2;
    const NOISE: i32 = -1;
    let n = stars.len();
    let mut labels = vec![UNVISITED; n];
    let mut cluster_id: i32 = 0;

    for i in 0..n {
        if labels[i] != UNVISITED {
            continue;
        }
        let neighbors = &neighborhoods[i];
        if neighbors.len() < min_members {
            labels[i] = NOISE;
            continue;
        }
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
            let j_nb = &neighborhoods[j];
            if j_nb.len() >= min_members {
                for &k in j_nb {
                    if labels[k] == UNVISITED || labels[k] == NOISE {
                        queue.push(k);
                    }
                }
            }
        }
        cluster_id += 1;
    }

    // Build stream summaries
    let mut groups: std::collections::HashMap<i32, Vec<usize>> =
        std::collections::HashMap::new();
    for (i, &label) in labels.iter().enumerate() {
        if label >= 0 {
            groups.entry(label).or_default().push(i);
        }
    }

    let mut streams: Vec<Stream> = groups
        .into_iter()
        .map(|(id, members)| {
            let n = members.len() as f64;

            // Mean velocity
            let (mut vx, mut vy, mut vz) = (0.0, 0.0, 0.0);
            let (mut px, mut py, mut pz) = (0.0, 0.0, 0.0);
            for &i in &members {
                let v = stars[i].vel_kms();
                vx += v[0];
                vy += v[1];
                vz += v[2];
                px += stars[i].x;
                py += stars[i].y;
                pz += stars[i].z;
            }
            let mean_vel = [vx / n, vy / n, vz / n];
            let mean_pos = [px / n, py / n, pz / n];

            // Velocity dispersion
            let vel_var: f64 = members
                .iter()
                .map(|&i| {
                    let v = stars[i].vel_kms();
                    let dx = v[0] - mean_vel[0];
                    let dy = v[1] - mean_vel[1];
                    let dz = v[2] - mean_vel[2];
                    dx * dx + dy * dy + dz * dz
                })
                .sum::<f64>()
                / n;

            // Spatial spread
            let pos_var: f64 = members
                .iter()
                .map(|&i| {
                    let dx = stars[i].x - mean_pos[0];
                    let dy = stars[i].y - mean_pos[1];
                    let dz = stars[i].z - mean_pos[2];
                    dx * dx + dy * dy + dz * dz
                })
                .sum::<f64>()
                / n;

            Stream {
                id: id as usize,
                members,
                mean_vel_kms: mean_vel,
                vel_dispersion_kms: vel_var.sqrt(),
                mean_pos_pc: mean_pos,
                spatial_spread_pc: pos_var.sqrt(),
            }
        })
        .collect();

    streams.sort_by(|a, b| b.members.len().cmp(&a.members.len()));
    streams
}
