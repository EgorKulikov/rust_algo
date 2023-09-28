use crate::collections::md_arr::arr2d::Arr2d;
use crate::collections::min_max::MinimMaxim;
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::graph::Graph;
use crate::numbers::num_traits::add_sub::Addable;
use crate::numbers::num_traits::ord::MinMax;
use crate::numbers::num_traits::zero_one::ZeroOne;

pub trait AllDistances<W: Addable + PartialOrd + Copy + ZeroOne + MinMax> {
    fn all_distances(&self) -> Arr2d<W>;
}

impl<W: Addable + PartialOrd + Copy + ZeroOne + MinMax, E: WeightedEdgeTrait<W>> AllDistances<W>
    for Graph<E>
{
    fn all_distances(&self) -> Arr2d<W> {
        let n = self.vertex_count();
        let inf = W::max_val();
        let mut res = Arr2d::new(n, n, inf);
        for i in 0..n {
            res[(i, i)] = W::zero();
            for e in self[i].iter() {
                res[(i, e.to())].minim(e.weight());
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    let r1 = res[(i, k)];
                    let r2 = res[(k, j)];
                    if r1 != inf && r2 != inf {
                        res[(i, j)].minim(r1 + r2);
                    }
                }
            }
        }
        res
    }
}
