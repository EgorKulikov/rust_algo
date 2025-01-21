use crate::graph::edges::edge_trait::BidirectionalEdgeTrait;
use crate::graph::Graph;
use crate::misc::recursive_function::{Callable2, RecursiveFunction2};
use std::ops::{Bound, RangeBounds};

pub struct HLDecomposition {
    pub paths: Vec<Vec<usize>>,
    pub id: Vec<usize>,
    pub pos: Vec<usize>,
    pub parent: Vec<usize>,
}

impl HLDecomposition {
    pub fn iter(&self, range: impl RangeBounds<usize>) -> HLIter<'_> {
        let Bound::Included(&source) = range.start_bound() else {
            panic!("HLDecomposition::iter: source must be included")
        };
        let (destination, included) = match range.end_bound() {
            Bound::Included(dest) => (*dest, true),
            Bound::Excluded(dest) => (*dest, false),
            Bound::Unbounded => panic!("HLDecomposition::iter: destination must be included"),
        };
        HLIter {
            decomposition: self,
            current_vert: source,
            dest_vert: destination,
            include_dest: included,
        }
    }
}

pub struct HLIter<'a> {
    decomposition: &'a HLDecomposition,
    current_vert: usize,
    dest_vert: usize,
    include_dest: bool,
}

pub struct HLPart {
    pub id: usize,
    pub pos_from: usize,
    pub pos_to: usize,
}

impl Iterator for HLIter<'_> {
    type Item = HLPart;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_vert == self.dest_vert && !self.include_dest {
            return None;
        }
        let id = self.decomposition.id[self.current_vert];
        if id == self.decomposition.id[self.dest_vert] {
            let pos_from =
                self.decomposition.pos[self.dest_vert] + if self.include_dest { 0 } else { 1 };
            let pos_to = self.decomposition.pos[self.current_vert];
            self.current_vert = self.dest_vert;
            self.include_dest = false;
            Some(HLPart {
                id,
                pos_from,
                pos_to,
            })
        } else {
            let pos_to = self.decomposition.pos[self.current_vert];
            if self.decomposition.parent[self.current_vert] == self.current_vert {
                panic!("HLIter: destination is not ancestor of source");
            }
            self.current_vert = self.decomposition.parent[self.decomposition.paths[id][0]];
            Some(HLPart {
                id,
                pos_from: 0,
                pos_to,
            })
        }
    }
}

pub trait HLDecompositionTrait {
    fn hl_decomposition_with_root(&self, root: usize) -> HLDecomposition;

    fn hl_decomposition(&self) -> HLDecomposition {
        self.hl_decomposition_with_root(0)
    }
}

impl<E: BidirectionalEdgeTrait> HLDecompositionTrait for Graph<E> {
    fn hl_decomposition_with_root(&self, root: usize) -> HLDecomposition {
        debug_assert!(self.is_tree());
        let n = self.vertex_count();
        let mut paths = Vec::new();
        let mut id = vec![0; n];
        let mut pos = vec![0; n];
        let mut size = vec![0u32; n];
        let mut parent = vec![root; n];
        let mut calc_size = RecursiveFunction2::new(|f, vert, last| {
            size[vert] = 1;
            parent[vert] = last;
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
        HLDecomposition {
            paths,
            id,
            pos,
            parent,
        }
    }
}
