use crate::graph::edges::edge_trait::BidirectionalEdgeTrait;
use crate::graph::graph::Graph;

pub struct DFSOrder {
    pub position: Vec<usize>,
    pub end: Vec<usize>,
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
        let mut edge = vec![0u32; count];
        let mut stack = vec![0u32; count];
        let mut last = vec![0u32; count];
        let mut size = 1usize;
        last[root] = root as u32;
        stack[0] = root as u32;
        position[root] = 0;
        let mut index = 0usize;
        while size > 0 {
            let current = stack[size - 1] as usize;
            let c_edge = &mut edge[current];
            if *c_edge == self[current].len() as u32 {
                end[current] = index + 1;
                size -= 1;
            } else {
                let next = self[current][*c_edge as usize].to();
                *c_edge += 1;
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
