use crate::graph::edges::edge_trait::BidirectionalEdgeTrait;
use crate::graph::graph::Graph;
use crate::misc::recursive_function::{Callable2, RecursiveFunction2};

pub trait HLDecomposition {
    fn hl_decomposition_with_root(&self, root: usize) -> (Vec<Vec<usize>>, Vec<usize>, Vec<usize>);

    fn hl_decomposition(&self) -> (Vec<Vec<usize>>, Vec<usize>, Vec<usize>) {
        self.hl_decomposition_with_root(0)
    }
}
impl<E: BidirectionalEdgeTrait> HLDecomposition for Graph<E> {
    fn hl_decomposition_with_root(&self, root: usize) -> (Vec<Vec<usize>>, Vec<usize>, Vec<usize>) {
        debug_assert!(self.is_tree());
        let n = self.vertex_count();
        let mut paths = Vec::new();
        let mut id = vec![0; n];
        let mut pos = vec![0; n];
        let mut size = vec![0u32; n];
        let mut calc_size = RecursiveFunction2::new(|f, vert, last| {
            size[vert] = 1;
            for e in self[vert].iter() {
                let next = e.to();
                if next == last {
                    continue;
                }
                size[vert] += f.call(next, vert);
            }
            size[vert]
        });
        calc_size.call(root, root);
        paths.push(vec![root]);
        let mut build = RecursiveFunction2::new(|f, vert: usize, last| {
            if vert != root {
                if 2 * size[vert] >= size[last] {
                    id[vert] = id[last];
                    pos[vert] = pos[last] + 1;
                    paths[id[vert]].push(vert);
                } else {
                    id[vert] = paths.len();
                    paths.push(vec![vert]);
                }
            }
            for e in self[vert].iter() {
                let next = e.to();
                if next == last {
                    continue;
                }
                f.call(next, vert);
            }
        });
        build.call(root, root);
        (paths, id, pos)
    }
}
