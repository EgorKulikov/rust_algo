use crate::collections::min_max::MinimMaxim;
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::graph::Graph;
use crate::numbers::num_traits::algebra::SemiRing;
use std::ops::Add;

pub trait NegativeDistances<W: SemiRing + Ord + Copy> {
    fn negative_distances_from(&self, source: usize) -> Vec<Distance<W>>;
}

impl<W: SemiRing + Ord + Copy, E: WeightedEdgeTrait<W>> NegativeDistances<W> for Graph<E> {
    fn negative_distances_from(&self, source: usize) -> Vec<Distance<W>> {
        let mut res = vec![Distance::None; self.vertex_count()];
        res[source].minim(Distance::Finite {
            distance: W::zero(),
            from: source,
            edge: self[source].len(),
        });
        for _ in 0..self.vertex_count() {
            for i in 0..self.vertex_count() {
                for (j, e) in self[i].iter().enumerate() {
                    let next = e.to();
                    let edge = Distance::Finite {
                        distance: e.weight(),
                        from: i,
                        edge: j,
                    };
                    let candidate = res[i] + edge;
                    res[next].minim(candidate);
                }
            }
        }
        let mut infinities = Vec::with_capacity(self.vertex_count());
        for i in 0..self.vertex_count() {
            for (j, e) in self[i].iter().enumerate() {
                let next = e.to();
                let edge = Distance::Finite {
                    distance: e.weight(),
                    from: i,
                    edge: j,
                };
                let candidate = res[i] + edge;
                if res[next].minim(candidate) {
                    infinities.push(next);
                    res[next] = Distance::Infinite;
                }
            }
        }
        while let Some(inf) = infinities.pop() {
            for e in &self[inf] {
                let next = e.to();
                if res[next] == Distance::Infinite {
                    continue;
                }
                infinities.push(next);
                res[next] = Distance::Infinite;
            }
        }
        res
    }
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Copy, Clone)]
pub enum Distance<W> {
    Infinite,
    Finite {
        distance: W,
        from: usize,
        edge: usize,
    },
    None,
}

impl<W: Ord + SemiRing> Add for Distance<W> {
    type Output = Distance<W>;

    fn add(self, other: Distance<W>) -> Distance<W> {
        match (self, other) {
            (Distance::None, _) | (_, Distance::None) => Distance::None,
            (Distance::Infinite, _) | (_, Distance::Infinite) => Distance::Infinite,
            (
                Distance::Finite { distance: a, .. },
                Distance::Finite {
                    distance: b,
                    from,
                    edge,
                },
            ) => Distance::Finite {
                distance: a + b,
                from,
                edge,
            },
        }
    }
}
