use crate::collections::bit_set::BitSet;
use crate::graph::edges::bi_edge::BiEdgeWithId;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::Graph;
use crate::misc::extensions::option::OptionExt;

pub trait EulerPath {
    fn euler_path(&self) -> Option<Vec<usize>>;
}

impl<P: Clone> EulerPath for Graph<BiEdgeWithId<P>> {
    fn euler_path(&self) -> Option<Vec<usize>> {
        let mut start = 0;
        let mut odd_count = 0;
        for i in 0..self.vertex_count() {
            if self.adj(i).len() % 2 == 1 {
                odd_count += 1;
                start = i;
            }
        }
        if odd_count > 2 {
            return None;
        }
        let mut removed = BitSet::new(self.edge_count());
        // `id[v]` is the current edge id at vertex `v` (u32::MAX = exhausted).
        let mut id = vec![u32::MAX; self.vertex_count()];
        for v in 0..self.vertex_count() {
            id[v] = self.head_edge(v);
        }
        let mut st = vec![start];
        let mut ans = Vec::with_capacity(self.edge_count() + 1);
        while let Some(&v) = st.last() {
            while id[v] != u32::MAX && removed[self.edge(id[v] as usize).id()] {
                id[v] = self.next_edge(id[v] as usize);
            }
            if id[v] == u32::MAX {
                st.pop();
                ans.push(v);
            } else {
                let edge = self.edge(id[v] as usize);
                removed.set(edge.id());
                st.push(edge.to());
            }
        }
        let len = ans.len();
        ans.take_if(len == self.edge_count() + 1)
    }
}
