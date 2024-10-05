use crate::graph::edges::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::strongly_connected_components::StronglyConnectedComponents;
use crate::misc::recursive_function::{Callable2, RecursiveFunction2};

pub struct TwoSat {
    n: usize,
    graph: Graph<Edge<()>>,
}

impl TwoSat {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            graph: Graph::new(n * 2),
        }
    }

    pub fn add_or(&mut self, a: usize, a_val: bool, b: usize, b_val: bool) {
        self.add_implication(a, !a_val, b, b_val);
        self.add_implication(b, !b_val, a, a_val);
    }

    pub fn add_implication(&mut self, a: usize, a_val: bool, b: usize, b_val: bool) {
        self.graph
            .add_edge(Edge::new(Self::id(a, a_val), Self::id(b, b_val)));
    }

    pub fn set_true(&mut self, a: usize) {
        self.add_implication(a, false, a, true);
    }

    pub fn set_false(&mut self, a: usize) {
        self.add_implication(a, true, a, false);
    }

    pub fn max_one(&mut self, r: &[(usize, bool)]) {
        if r.is_empty() {
            return;
        }
        let mut rec = RecursiveFunction2::new(|rec, from: usize, to: usize| -> (usize, usize) {
            if from + 1 == to {
                return (
                    Self::id(r[from].0, r[from].1),
                    Self::id(r[from].0, !r[from].1),
                );
            }
            let mid = (from + to) / 2;
            let (left_root, left_sink) = rec.call(from, mid);
            let (right_root, right_sink) = rec.call(mid, to);
            self.graph.add_edge(Edge::new(left_sink, right_root));
            self.graph.add_edge(Edge::new(right_sink, left_root));
            if from != 0 || to != r.len() {
                self.graph.add_vertices(1);
                let root = self.graph.vertex_count() - 1;
                self.graph.add_edge(Edge::new(root, left_root));
                self.graph.add_edge(Edge::new(root, right_root));
                self.graph.add_vertices(1);
                let sink = self.graph.vertex_count() - 1;
                self.graph.add_edge(Edge::new(left_sink, sink));
                self.graph.add_edge(Edge::new(right_sink, sink));
                (root, sink)
            } else {
                (0, 0)
            }
        });
        rec.call(0, r.len());
    }

    fn id(a: usize, val: bool) -> usize {
        a * 2 + val as usize
    }

    pub fn solve(&self) -> Option<Vec<bool>> {
        let (color, _) = self.graph.strongly_connected_components();
        let n = self.n;
        for i in 0..n {
            if color[i * 2] == color[i * 2 + 1] {
                return None;
            }
        }
        let mut result = vec![false; n];
        for i in 0..n {
            result[i] = color[i * 2] > color[i * 2 + 1];
        }
        Some(result)
    }
}
