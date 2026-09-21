use crate::collections::md_arr::arr2d::Arr2d;
use crate::collections::min_max::MinimMaxim;
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::Graph;
use crate::numbers::num_traits::algebra::SemiRing;
use std::ops::Add;

pub trait AllDistances<W: SemiRing + Ord + Copy> {
    fn all_distances(&self) -> Arr2d<Distance<W>>;
}

impl<W: SemiRing + Ord + Copy, E: WeightedEdgeTrait<W>> AllDistances<W> for Graph<E> {
    fn all_distances(&self) -> Arr2d<Distance<W>> {
        let n = self.vertex_count();
        let mut res = Arr2d::new(n, n, Distance::None);
        let mut has_negative = false;
        for i in 0..n {
            res[(i, i)] = Distance::Finite(W::zero());
            for e in self.adj(i).iter() {
                res[(i, e.to())].minim(Distance::Finite(e.weight()));
                if e.weight() < W::zero() {
                    has_negative = true;
                }
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    let r1 = res[(i, k)];
                    let r2 = res[(k, j)];
                    res[(i, j)].minim(r1 + r2);
                }
            }
            if has_negative {
                for k in 0..n {
                    if let Distance::Finite(w) = res[(k, k)] {
                        if w < W::zero() {
                            res[(k, k)] = Distance::Infinite;
                            for i in 0..n {
                                if res[(i, k)] == Distance::None {
                                    continue;
                                }
                                for j in 0..n {
                                    if res[(k, j)] == Distance::None {
                                        continue;
                                    }
                                    res[(i, j)] = Distance::Infinite;
                                }
                            }
                        }
                    }
                }
            }
        }
        if has_negative {
            // The marking inside the loop only knows the reachability found so
            // far (it is there to keep the values from blowing up), so repeat
            // it once everything is known.
            for k in 0..n {
                if res[(k, k)] != Distance::Infinite {
                    continue;
                }
                for i in 0..n {
                    if res[(i, k)] == Distance::None {
                        continue;
                    }
                    for j in 0..n {
                        if res[(k, j)] != Distance::None {
                            res[(i, j)] = Distance::Infinite;
                        }
                    }
                }
            }
        }
        res
    }
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Copy, Clone)]
pub enum Distance<W> {
    Infinite,
    Finite(W),
    None,
}

impl<W> Distance<W> {
    pub fn to_finite(self) -> W {
        match self {
            Distance::Finite(w) => w,
            _ => unreachable!(),
        }
    }
}

impl<W: Ord + SemiRing> Add for Distance<W> {
    type Output = Distance<W>;

    fn add(self, other: Distance<W>) -> Distance<W> {
        match (self, other) {
            (Distance::None, _) | (_, Distance::None) => Distance::None,
            (Distance::Infinite, _) | (_, Distance::Infinite) => Distance::Infinite,
            (Distance::Finite(a), Distance::Finite(b)) => Distance::Finite(a + b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AllDistances, Distance};
    use crate::graph::edges::weighted_edge::WeightedEdge;
    use crate::graph::Graph;
    use crate::misc::random::{Random, RandomTrait};

    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Expected {
        Unreachable,
        MinusInfinity,
        Finite(i64),
    }

    /// Bellman-Ford from every source, run long enough to settle, then anything
    /// reachable through a still-relaxable vertex is minus infinity.
    fn brute(n: usize, edges: &[(usize, usize, i64)]) -> Vec<Vec<Expected>> {
        let mut res = Vec::new();
        for s in 0..n {
            let mut d = vec![None; n];
            d[s] = Some(0i64);
            for _ in 0..n {
                for &(a, b, w) in edges {
                    if let Some(da) = d[a] {
                        if d[b].map_or(true, |db| da + w < db) {
                            d[b] = Some(da + w);
                        }
                    }
                }
            }
            let mut bad = vec![false; n];
            for _ in 0..2 * n {
                for &(a, b, w) in edges {
                    if let Some(da) = d[a] {
                        if bad[a] || d[b].map_or(true, |db| da + w < db) {
                            bad[b] = true;
                            d[b] = Some(d[b].map_or(da + w, |db| db.min(da + w)));
                        }
                    }
                }
            }
            res.push(
                (0..n)
                    .map(|v| match d[v] {
                        None => Expected::Unreachable,
                        Some(_) if bad[v] => Expected::MinusInfinity,
                        Some(x) => Expected::Finite(x),
                    })
                    .collect(),
            );
        }
        res
    }

    fn check(n: usize, edges: &[(usize, usize, i64)]) {
        let mut g: Graph<WeightedEdge<i64, ()>> = Graph::new_linked(n);
        for &(a, b, w) in edges {
            g.add_edge(WeightedEdge::new(a, b, w));
        }
        let d = g.all_distances();
        let expected = brute(n, edges);
        for i in 0..n {
            for j in 0..n {
                let got = match d[(i, j)] {
                    Distance::None => Expected::Unreachable,
                    Distance::Infinite => Expected::MinusInfinity,
                    Distance::Finite(w) => Expected::Finite(w),
                };
                assert_eq!(got, expected[i][j], "({}, {}) in {:?}", i, j, edges);
            }
        }
    }

    #[test]
    fn negative_cycle_reaches_every_connected_pair() {
        // negative cycle 1 -> 2 -> 1 inside a strongly connected graph
        check(
            4,
            &[
                (3, 0, 1),
                (3, 1, 0),
                (1, 3, -1),
                (0, 3, 1),
                (1, 2, -1),
                (2, 1, 0),
            ],
        );
        // negative self-loop
        check(3, &[(1, 1, -1), (1, 2, 0), (0, 2, 0), (2, 0, 0), (2, 1, 0)]);
    }

    #[test]
    fn random_graphs_with_negative_edges() {
        let mut rng = Random::new_with_seed(239);
        for _ in 0..3000 {
            let n = rng.gen_range(1..=6usize);
            let m = rng.gen_range(0..=10usize);
            let edges: Vec<_> = (0..m)
                .map(|_| (rng.gen_bound(n), rng.gen_bound(n), rng.gen_range(-3..=6i64)))
                .collect();
            check(n, &edges);
        }
    }
}
