use crate::collections::bit_set::BitSet;
use crate::collections::min_max::MinimMaxim;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph::Graph;
use crate::misc::recursive_function::{Callable2, RecursiveFunction2};

pub trait CutPointSearch {
    fn cut_points(&self) -> Vec<usize>;
}

impl<E: EdgeTrait> CutPointSearch for Graph<E> {
    fn cut_points(&self) -> Vec<usize> {
        assert!(E::REVERSABLE);
        let n = self.vertex_count();
        let mut timer = 0;
        let mut tin = vec![0; n];
        let mut fup = vec![0; n];
        let mut used = BitSet::new(n);
        let mut ans = Vec::new();
        for i in 0..n {
            if !used[i] {
                let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                    let mut children = 0;
                    used.set(vert);
                    tin[vert] = timer;
                    fup[vert] = timer;
                    timer += 1;
                    for e in &self[vert] {
                        if e.to() == prev {
                            continue;
                        }
                        let to = e.to();
                        if used[to] {
                            fup[vert].minim(tin[to]);
                        } else {
                            f.call(to, vert);
                            let cand = fup[to];
                            fup[vert].minim(cand);
                            if fup[to] >= tin[vert] && prev != n {
                                ans.push(vert);
                            }
                            children += 1;
                        }
                    }
                    if prev == n && children > 1 {
                        ans.push(vert);
                    }
                });
                dfs.call(i, n);
            }
        }
        ans
    }
}
