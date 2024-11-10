use crate::collections::bit_set::BitSet;
use crate::graph::edges::bi_edge_trait::BiEdgeTrait;
use crate::graph::Graph;
use crate::misc::recursive_function::{Callable, Callable2, RecursiveFunction, RecursiveFunction2};

pub trait Decompose<F: FnMut(usize, &BitSet)> {
    fn decompose(&self, f: F);
}

impl<E: BiEdgeTrait, F: FnMut(usize, &BitSet)> Decompose<F> for Graph<E> {
    fn decompose(&self, mut f: F) {
        let n = self.vertex_count();
        let mut sz = vec![0; n];
        let mut closed = BitSet::new(n);
        let mut center_decomp = RecursiveFunction::new(|center_decomp, vert: usize| {
            let mut dfs = RecursiveFunction2::new(|dfs, vert: usize, prev: usize| {
                if closed[vert] {
                    sz[vert] = 0;
                    return 0;
                }
                sz[vert] = 1;
                for e in &self[vert] {
                    if e.to() == prev {
                        continue;
                    }
                    sz[vert] += dfs.call(e.to(), vert);
                }
                sz[vert]
            });
            let total = dfs.call(vert, vert);
            let mut cur = vert;
            let mut last = vert;
            loop {
                let mut found = false;
                for e in &self[vert] {
                    if e.to() != last && 2 * sz[e.to()] >= total {
                        found = true;
                        last = cur;
                        cur = e.to();
                        break;
                    }
                }
                if !found {
                    break;
                }
            }
            f(cur, &closed);
            closed.set(cur);
            for e in &self[cur] {
                if !closed[e.to()] {
                    center_decomp.call(e.to());
                }
            }
        });
        center_decomp.call(0);
    }
}
