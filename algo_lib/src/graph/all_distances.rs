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
            for e in self[i].iter() {
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
        res
    }
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Copy, Clone)]
pub enum Distance<W> {
    Infinite,
    Finite(W),
    None,
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
