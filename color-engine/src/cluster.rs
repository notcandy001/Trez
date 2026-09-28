use crate::color::OklabColor;
use rand::{Rng, SeedableRng, RngExt};
use rand_chacha::ChaCha8Rng;

#[derive(Clone)]
pub struct Cluster {
    pub centroid: OklabColor,
    pub weight: usize,
}

pub fn kmeans(colors: &[OklabColor], k: usize, max_iterations: usize) -> Vec<Cluster> {
    if colors.is_empty() {
        return vec![];
    }
    if k == 0 {
        return vec![];
    }
    
    // Deterministic random number generator
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    
    // K-means++ initialization
    let mut centroids = Vec::with_capacity(k);
    centroids.push(colors[rng.random_range(0..colors.len())]);

    for _ in 1..k {
        let mut distances: Vec<f32> = colors
            .iter()
            .map(|c| {
                centroids
                    .iter()
                    .map(|centroid| c.distance_squared(centroid))
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap()
            })
            .collect();
            
        let total_weight: f32 = distances.iter().sum();
        if total_weight <= 0.0 {
            // Fallback if all remaining points are identical to existing centroids
            centroids.push(colors[rng.random_range(0..colors.len())]);
            continue;
        }

        let mut target = rng.random_range(0.0..total_weight);
        let mut selected_idx = 0;
        for (i, d) in distances.iter().enumerate() {
            target -= d;
            if target <= 0.0 {
                selected_idx = i;
                break;
            }
        }
        centroids.push(colors[selected_idx]);
    }

    let mut clusters = vec![
        Cluster {
            centroid: OklabColor::new(0.0, 0.0, 0.0),
            weight: 0,
        };
        k
    ];
    let mut assignments = vec![0; colors.len()];
    
    for _ in 0..max_iterations {
        let mut changed = false;
        
        // Assign
        for (i, c) in colors.iter().enumerate() {
            let mut best_idx = 0;
            let mut best_dist = f32::MAX;
            for (j, centroid) in centroids.iter().enumerate() {
                let dist = c.distance_squared(centroid);
                if dist < best_dist {
                    best_dist = dist;
                    best_idx = j;
                }
            }
            if assignments[i] != best_idx {
                assignments[i] = best_idx;
                changed = true;
            }
        }
        
        if !changed {
            break;
        }
        
        // Update
        let mut sums = vec![(0.0, 0.0, 0.0); k];
        let mut counts = vec![0; k];
        
        for (i, c) in colors.iter().enumerate() {
            let idx = assignments[i];
            sums[idx].0 += c.l;
            sums[idx].1 += c.a;
            sums[idx].2 += c.b;
            counts[idx] += 1;
        }
        
        for j in 0..k {
            if counts[j] > 0 {
                centroids[j] = OklabColor::new(
                    sums[j].0 / counts[j] as f32,
                    sums[j].1 / counts[j] as f32,
                    sums[j].2 / counts[j] as f32,
                );
            }
        }
    }
    
    let mut counts = vec![0; k];
    for idx in assignments {
        counts[idx] += 1;
    }
    
    for j in 0..k {
        clusters[j] = Cluster {
            centroid: centroids[j],
            weight: counts[j],
        };
    }
    
    // Sort by weight descending
    clusters.sort_by(|a, b| b.weight.cmp(&a.weight));
    clusters
}
