use crate::collections::dsu::DSU;
use std::collections::BTreeMap;

/// Minimum spanning tree under the Manhattan metric in O(n log n): for each
/// point and each of eight octants only the nearest point matters, which
/// gives at most `4n` candidate edges, followed by Kruskal. Returns the
/// tree edges as `(length, a, b)`.
pub fn manhattan_mst(points: &[(i64, i64)]) -> Vec<(i64, usize, usize)> {
    let n = points.len();
    let mut pts: Vec<(i64, i64)> = points.to_vec();
    let mut ids: Vec<usize> = (0..n).collect();
    let mut candidates: Vec<(i64, usize, usize)> = Vec::with_capacity(4 * n);
    for _ in 0..2 {
        for flip_x in 0..2 {
            // Sweep by x + y. The map (keyed by -y) holds points still waiting
            // for their nearest neighbour in the octant above the diagonal.
            ids.sort_by_key(|&i| pts[i].0 + pts[i].1);
            let mut sweep: BTreeMap<i64, usize> = BTreeMap::new();
            for &i in &ids {
                let (x, y) = pts[i];
                let mut done = Vec::new();
                for (&key, &j) in sweep.range(-y..) {
                    let (xj, yj) = pts[j];
                    if x - xj < y - yj {
                        break;
                    }
                    candidates.push(((x - xj).abs() + (y - yj).abs(), i, j));
                    done.push(key);
                }
                for key in done {
                    sweep.remove(&key);
                }
                sweep.insert(-y, i);
            }
            // Next of the four orientations: mirror across x = y on the first
            // pass of each pair, negate x on the second.
            for p in pts.iter_mut() {
                if flip_x == 0 {
                    std::mem::swap(&mut p.0, &mut p.1);
                } else {
                    p.0 = -p.0;
                }
            }
        }
    }
    candidates.sort_unstable();
    let mut dsu = DSU::new(n);
    let mut tree = Vec::with_capacity(n.saturating_sub(1));
    for (w, a, b) in candidates {
        if dsu.union(a, b) {
            tree.push((w, a, b));
        }
    }
    tree
}

#[cfg(test)]
mod tests {
    use super::manhattan_mst;
    use crate::misc::random::{Random, RandomTrait};

    fn prim(points: &[(i64, i64)]) -> i64 {
        let n = points.len();
        let mut dist = vec![i64::MAX; n];
        let mut used = vec![false; n];
        dist[0] = 0;
        let mut total = 0;
        for _ in 0..n {
            let v = (0..n)
                .filter(|&v| !used[v])
                .min_by_key(|&v| dist[v])
                .unwrap();
            used[v] = true;
            total += dist[v];
            for u in 0..n {
                let d = (points[v].0 - points[u].0).abs() + (points[v].1 - points[u].1).abs();
                if !used[u] && d < dist[u] {
                    dist[u] = d;
                }
            }
        }
        total
    }

    #[test]
    fn weight_matches_prim() {
        let mut rng = Random::new_with_seed(351);
        for round in 0..300 {
            let n = rng.gen_range(1..=50usize);
            let range = if round % 3 == 0 { 4i64 } else { 1_000_000_000 };
            let points: Vec<(i64, i64)> = (0..n)
                .map(|_| (rng.gen_range(-range..=range), rng.gen_range(-range..=range)))
                .collect();
            let tree = manhattan_mst(&points);
            assert_eq!(tree.len(), n - 1);
            for &(w, a, b) in &tree {
                assert_eq!(
                    w,
                    (points[a].0 - points[b].0).abs() + (points[a].1 - points[b].1).abs()
                );
            }
            assert_eq!(
                tree.iter().map(|e| e.0).sum::<i64>(),
                prim(&points),
                "n={n}"
            );
        }
    }
}
