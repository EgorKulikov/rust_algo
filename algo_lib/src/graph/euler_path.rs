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
        if self.vertex_count() == 0 {
            return Some(Vec::new());
        }
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
        if odd_count == 0 {
            // A cycle has to start at a vertex that has edges; vertex 0 may
            // have none.
            start = (0..self.vertex_count())
                .find(|&v| self.adj(v).len() > 0)
                .unwrap_or(0);
        }
        let mut removed = BitSet::new(self.edge_count());
        // `id[v]` is the current edge id at vertex `v` (u32::MAX = exhausted).
        let mut id: Vec<u32> = (0..self.vertex_count())
            .map(|v| self.head_edge(v))
            .collect();
        let mut st = vec![start];
        let mut ans = Vec::with_capacity(self.edge_count() + 1);
        while let Some(&v) = st.last() {
            while id[v] != u32::MAX && removed[self.edge_at(v, id[v]).id()] {
                id[v] = self.step_edge(v, id[v]);
            }
            if id[v] == u32::MAX {
                st.pop();
                ans.push(v);
            } else {
                let edge = self.edge_at(v, id[v]);
                removed.set(edge.id());
                st.push(edge.to());
            }
        }
        let len = ans.len();
        ans.take_if(len == self.edge_count() + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::EulerPath;
    use crate::graph::edges::bi_edge::BiEdgeWithId;
    use crate::graph::Graph;

    fn path(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
        let mut g: Graph<BiEdgeWithId<()>> = Graph::new_linked(n);
        for &(a, b) in edges {
            g.add_edge(BiEdgeWithId::new(a, b));
        }
        g.euler_path()
    }

    fn assert_uses_every_edge_once(edges: &[(usize, usize)], path: &[usize]) {
        let mut left: Vec<(usize, usize)> =
            edges.iter().map(|&(a, b)| (a.min(b), a.max(b))).collect();
        for w in path.windows(2) {
            let e = (w[0].min(w[1]), w[0].max(w[1]));
            let pos = left
                .iter()
                .position(|&x| x == e)
                .expect("edge not in graph");
            left.swap_remove(pos);
        }
        assert!(left.is_empty());
    }

    #[test]
    fn cycle_that_avoids_vertex_zero() {
        let edges = [(1, 2), (2, 3), (3, 1)];
        let p = path(4, &edges).expect("cycle 1-2-3-1 exists");
        assert_uses_every_edge_once(&edges, &p);
        let edges = [(1, 1)];
        let p = path(2, &edges).expect("a loop at vertex 1 is an euler cycle");
        assert_uses_every_edge_once(&edges, &p);
    }

    #[test]
    fn path_cycle_and_impossible_cases() {
        let edges = [(0, 1), (1, 2), (2, 0), (2, 3)];
        let p = path(4, &edges).unwrap();
        assert_uses_every_edge_once(&edges, &p);
        assert_eq!(path(4, &[(0, 1), (2, 3)]), None);
        assert_eq!(path(4, &[(0, 1), (0, 2), (0, 3)]), None);
        assert_eq!(path(3, &[]), Some(vec![0]));
    }

    #[test]
    fn graph_without_vertices() {
        assert_eq!(path(0, &[]), Some(vec![]));
    }
}
