use crate::collections::md_arr::arr2d::Arr2d;
use crate::collections::rmq::Rmq;
use crate::graph::edges::edge_trait::BidirectionalEdgeTrait;
use crate::graph::Graph;
use crate::misc::owned_cell::OwnedCell;
use crate::numbers::num_traits::bit_ops::BitOps;

pub struct LCA {
    /// Preorder position of each vertex.
    position: Vec<u32>,
    /// Vertex at each preorder position.
    order: Vec<u32>,
    /// Minimum over `parent_position[pos(u) + 1 ..= pos(v)]` locates the LCA.
    parent_position: Rmq<u32>,
    level: Vec<u32>,
    parent: Vec<u32>,
    ancestors: OwnedCell<Option<Arr2d<i32>>>,
}

impl LCA {
    pub fn level(&self, vert: usize) -> usize {
        self.level[vert] as usize
    }

    pub fn parent(&self, vert: usize) -> Option<usize> {
        if (self.parent[vert] as usize) == vert {
            None
        } else {
            Some(self.parent[vert] as usize)
        }
    }

    pub fn lca(&self, first: usize, second: usize) -> usize {
        if first == second {
            return first;
        }
        let (a, b) = (self.position[first], self.position[second]);
        let (from, to) = if a < b { (a, b) } else { (b, a) };
        let at = self.parent_position.argmin(from as usize + 1..=to as usize);
        self.order[self.parent_position.values()[at] as usize] as usize
    }

    pub fn position(&self, vertex: usize) -> usize {
        self.position[vertex] as usize
    }

    pub fn on_path(&self, a: usize, b: usize, c: usize) -> bool {
        let lca = self.lca(a, b);
        self.lca(a, c) == lca && self.lca(b, c) == c || self.lca(a, c) == c && self.lca(b, c) == lca
    }

    pub fn path_length(&self, first: usize, second: usize) -> usize {
        (self.level[first] + self.level[second] - 2 * self.level[self.lca(first, second)]) as usize
    }

    pub fn num_levels(&self) -> usize {
        self.build_steps();
        unsafe { self.ancestors.as_ref().as_ref().unwrap().d1() }
    }

    pub fn nth_ancestor(&self, mut vert: usize, index: usize) -> Option<usize> {
        self.build_steps();
        unsafe {
            let ancestors = self.ancestors.as_ref().as_ref().unwrap();
            let height = ancestors.d1();
            if index >= (1 << height) {
                return None;
            }
            for i in 0..height {
                if index.is_set(i) {
                    let pred = ancestors[(i, vert)];
                    if pred == -1 {
                        return None;
                    }
                    vert = pred as usize;
                }
            }
            Some(vert)
        }
    }

    pub fn nth_vert_on_path(&self, from: usize, to: usize, index: usize) -> usize {
        let len = self.path_length(from, to);
        assert!(index <= len);
        let lca = self.lca(from, to);
        if index <= self.level(from) - self.level(lca) {
            self.nth_ancestor(from, index).unwrap()
        } else {
            self.nth_ancestor(to, len - index).unwrap()
        }
    }

    pub fn path(&self, from: usize, to: usize) -> Vec<usize> {
        let lca_vertex = self.lca(from, to);
        let mut result = Vec::new();
        let mut v = from;
        while v != lca_vertex {
            result.push(v);
            v = self.parent[v] as usize;
        }
        result.push(lca_vertex);
        let desc_start = result.len();
        let mut v = to;
        while v != lca_vertex {
            result.push(v);
            v = self.parent[v] as usize;
        }
        result[desc_start..].reverse();
        result
    }

    fn build_steps(&self) {
        unsafe {
            if self.ancestors.as_ref().is_some() {
                return;
            }
        }
        let vertex_count = self.position.len();
        let len = (32 - (vertex_count as u32).leading_zeros()) as usize;
        let mut predecessors = Arr2d::new(len, vertex_count, -1);
        for i in 0..vertex_count {
            predecessors[(0, i)] = match self.parent(i) {
                None => -1,
                Some(v) => v as i32,
            };
        }
        for i in 1..len {
            for j in 0..vertex_count {
                let p = predecessors[(i - 1, j)];
                if p == -1 {
                    predecessors[(i, j)] = -1;
                } else {
                    predecessors[(i, j)] = predecessors[(i - 1, p as usize)];
                }
            }
        }
        unsafe {
            self.ancestors.replace(Some(predecessors));
        }
    }
}

pub trait LCATrait {
    fn lca_with_root(&self, root: usize) -> LCA;

    fn lca(&self) -> LCA {
        self.lca_with_root(0)
    }
}

impl<E: BidirectionalEdgeTrait> LCATrait for Graph<E> {
    fn lca_with_root(&self, root: usize) -> LCA {
        debug_assert!(self.is_tree());
        let n = self.vertex_count();
        let mut position = vec![0u32; n];
        let mut order = Vec::with_capacity(n);
        let mut parent_position = Vec::with_capacity(n);
        let mut level = vec![0u32; n];
        let mut parent = vec![0u32; n];
        parent[root] = root as u32;
        // Iterative preorder DFS over edge cursors.
        let mut stack: Vec<(u32, u32)> = vec![(root as u32, self.head_edge(root))];
        position[root] = 0;
        order.push(root as u32);
        parent_position.push(0);
        while let Some(frame) = stack.last_mut() {
            let v = frame.0 as usize;
            let cursor = frame.1;
            if cursor == u32::MAX {
                stack.pop();
                continue;
            }
            frame.1 = self.step_edge(v, cursor);
            let to = self.edge_at(v, cursor).to();
            if to == parent[v] as usize && v != root || to == root {
                continue;
            }
            parent[to] = v as u32;
            level[to] = level[v] + 1;
            position[to] = order.len() as u32;
            order.push(to as u32);
            parent_position.push(position[v]);
            stack.push((to as u32, self.head_edge(to)));
        }
        LCA {
            position,
            order,
            parent_position: Rmq::new(parent_position),
            level,
            parent,
            ancestors: OwnedCell::new(None),
        }
    }
}

#[cfg(test)]
mod test {
    use super::LCATrait;
    use crate::graph::edges::bi_edge::BiEdge;
    use crate::graph::Graph;

    //        0
    //       / \
    //      1   2
    //     / \
    //    3   4
    fn make_tree() -> Graph<BiEdge<()>> {
        Graph::with_biedges(5, &[(0, 1), (0, 2), (1, 3), (1, 4)])
    }

    #[test]
    fn lca_basic() {
        let lca = make_tree().lca();
        assert_eq!(lca.lca(3, 4), 1);
        assert_eq!(lca.lca(3, 2), 0);
    }

    #[test]
    fn path_length() {
        let lca = make_tree().lca();
        assert_eq!(lca.path_length(3, 4), 2);
        assert_eq!(lca.path_length(3, 2), 3);
    }

    #[test]
    fn root_parent_none() {
        let lca = make_tree().lca();
        assert_eq!(lca.parent(0), None);
        assert_eq!(lca.parent(1), Some(0));
    }

    #[test]
    fn on_path() {
        let lca = make_tree().lca();
        assert!(lca.on_path(3, 4, 1));
        assert!(lca.on_path(3, 2, 1));
        assert!(!lca.on_path(3, 4, 2));
    }

    #[test]
    fn random_trees_match_naive() {
        use crate::misc::random::{Random, RandomTrait};
        let mut rng = Random::new_with_seed(91);
        for n in [1usize, 2, 3, 10, 64, 65, 200, 1000] {
            let mut edges = Vec::new();
            for v in 1..n {
                edges.push((rng.gen_range(0..v), v));
            }
            let graph = Graph::with_biedges(n, &edges);
            let root = rng.gen_range(0..n);
            let lca = graph.lca_with_root(root);
            let naive = |mut a: usize, mut b: usize| {
                while lca.level(a) > lca.level(b) {
                    a = lca.parent(a).unwrap();
                }
                while lca.level(b) > lca.level(a) {
                    b = lca.parent(b).unwrap();
                }
                while a != b {
                    a = lca.parent(a).unwrap();
                    b = lca.parent(b).unwrap();
                }
                a
            };
            assert_eq!(lca.parent(root), None);
            for _ in 0..2000 {
                let a = rng.gen_range(0..n);
                let b = rng.gen_range(0..n);
                assert_eq!(lca.lca(a, b), naive(a, b), "n={n} root={root} {a} {b}");
            }
            let mut positions: Vec<usize> = (0..n).map(|v| lca.position(v)).collect();
            positions.sort();
            assert_eq!(positions, (0..n).collect::<Vec<_>>());
        }
    }

    #[test]
    fn path_vertices() {
        let lca = make_tree().lca();
        assert_eq!(lca.path(3, 4), vec![3, 1, 4]);
        assert_eq!(lca.path(3, 2), vec![3, 1, 0, 2]);
        assert_eq!(lca.path(0, 4), vec![0, 1, 4]);
        assert_eq!(lca.path(3, 3), vec![3]);
    }
}
