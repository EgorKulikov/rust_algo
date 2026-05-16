use crate::graph::edges::edge_trait::BidirectionalEdgeTrait;
use crate::graph::Graph;
use std::ops::Range;

pub struct DFSOrder {
    pub position: Vec<usize>,
    pub end: Vec<usize>,
}

impl DFSOrder {
    pub fn len(&self, vert: usize) -> usize {
        self.end[vert] - self.position[vert]
    }

    pub fn subtree(&self, vert: usize) -> Range<usize> {
        self.position[vert]..self.end[vert]
    }
}

pub trait DFSOrderTrait {
    fn dfs_order_with_root(&self, root: usize) -> DFSOrder;

    fn dfs_order(&self) -> DFSOrder {
        self.dfs_order_with_root(0)
    }
}

impl<E: BidirectionalEdgeTrait> DFSOrderTrait for Graph<E> {
    fn dfs_order_with_root(&self, root: usize) -> DFSOrder {
        debug_assert!(self.is_tree());
        let count = self.vertex_count();
        let mut position = vec![0; count];
        let mut end = vec![0; count];
        // `edge[v]` is the current edge id at vertex `v` (u32::MAX = exhausted).
        let mut edge = vec![u32::MAX; count];
        for v in 0..count {
            edge[v] = self.head_edge(v);
        }
        let mut stack = vec![0u32; count];
        let mut last = vec![0u32; count];
        let mut size = 1usize;
        last[root] = root as u32;
        stack[0] = root as u32;
        position[root] = 0;
        let mut index = 0usize;
        while size > 0 {
            let current = stack[size - 1] as usize;
            let c_edge = edge[current];
            if c_edge == u32::MAX {
                end[current] = index + 1;
                size -= 1;
            } else {
                let eid = c_edge as usize;
                let next = self.edge(eid).to();
                edge[current] = self.next_edge(eid);
                if next == (last[current] as usize) {
                    continue;
                }
                index += 1;
                position[next] = index;
                last[next] = current as u32;
                stack[size] = next as u32;
                size += 1;
            }
        }
        DFSOrder { position, end }
    }
}
