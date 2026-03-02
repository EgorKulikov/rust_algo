use crate::graph::edges::edge::Edge;
use crate::graph::strongly_connected_components::StronglyConnectedComponentsTrait;
use crate::graph::Graph;
use crate::misc::recursive_function::{Callable2, RecursiveFunction2};

#[derive(Clone)]
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
        self.add_edge(a, !a_val, b, b_val);
        self.add_edge(b, !b_val, a, a_val);
    }

    pub fn add_implication(&mut self, a: usize, a_val: bool, b: usize, b_val: bool) {
        self.add_or(a, !a_val, b, b_val);
    }

    fn add_edge(&mut self, a: usize, a_val: bool, b: usize, b_val: bool) {
        self.graph
            .add_edge(Edge::new(Self::id(a, a_val), Self::id(b, b_val)));
    }

    pub fn set_true(&mut self, a: usize) {
        self.add_edge(a, false, a, true);
    }

    pub fn set_false(&mut self, a: usize) {
        self.add_edge(a, true, a, false);
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
            self.graph.add_edge(Edge::new(left_root, right_sink));
            self.graph.add_edge(Edge::new(right_root, left_sink));
            if from != 0 || to != r.len() {
                self.graph.add_vertices(1);
                let root = self.graph.vertex_count() - 1;
                self.graph.add_edge(Edge::new(left_root, root));
                self.graph.add_edge(Edge::new(right_root, root));
                self.graph.add_vertices(1);
                let sink = self.graph.vertex_count() - 1;
                self.graph.add_edge(Edge::new(sink, left_sink));
                self.graph.add_edge(Edge::new(sink, right_sink));
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

    pub fn clear(&mut self) {
        self.graph.clear();
    }

    pub fn solve(&self) -> Option<Vec<bool>> {
        let color = self.graph.strongly_connected_component_colors();
        let n = self.n;
        for i in 0..n {
            if color[i * 2] == color[i * 2 + 1] {
                return None;
            }
        }
        let mut result = vec![false; n];
        for i in 0..n {
            result[i] = color[i * 2] < color[i * 2 + 1];
        }
        Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_set_true() {
        let mut ts = TwoSat::new(1);
        ts.set_true(0);
        let sol = ts.solve().unwrap();
        assert!(sol[0], "x_0 should be true after set_true");
    }

    #[test]
    fn simple_set_false() {
        let mut ts = TwoSat::new(1);
        ts.set_false(0);
        let sol = ts.solve().unwrap();
        assert!(!sol[0], "x_0 should be false after set_false");
    }

    #[test]
    fn simple_or() {
        // x_0 OR x_1
        let mut ts = TwoSat::new(2);
        ts.add_or(0, true, 1, true);
        let sol = ts.solve().unwrap();
        assert!(sol[0] || sol[1], "at least one should be true");
    }

    #[test]
    fn max_one_two_vars() {
        // At most one of x_0, x_1 can be true
        let mut ts = TwoSat::new(2);
        ts.max_one(&[(0, true), (1, true)]);
        // Should be satisfiable (e.g., both false)
        let sol = ts.solve().unwrap();
        let count = sol.iter().filter(|&&v| v).count();
        assert!(count <= 1, "max_one violated: {} are true", count);
    }

    #[test]
    fn max_one_two_vars_with_forced() {
        // At most one of x_0, x_1 true, and x_0 must be true
        let mut ts = TwoSat::new(2);
        ts.max_one(&[(0, true), (1, true)]);
        ts.set_true(0);
        let sol = ts.solve().unwrap();
        assert!(sol[0], "x_0 should be true");
        assert!(!sol[1], "x_1 should be false (max_one with x_0=true)");
    }

    #[test]
    fn max_one_two_vars_conflict() {
        // At most one of x_0, x_1 true, but both forced true -> unsatisfiable
        let mut ts = TwoSat::new(2);
        ts.max_one(&[(0, true), (1, true)]);
        ts.set_true(0);
        ts.set_true(1);
        assert!(ts.solve().is_none(), "should be unsatisfiable");
    }

    #[test]
    fn max_one_three_vars() {
        // At most one of x_0, x_1, x_2 can be true
        let mut ts = TwoSat::new(3);
        ts.max_one(&[(0, true), (1, true), (2, true)]);
        let sol = ts.solve().unwrap();
        let count = sol.iter().filter(|&&v| v).count();
        assert!(count <= 1, "max_one(3) violated: {} are true", count);
    }

    #[test]
    fn max_one_three_vars_with_forced() {
        // At most one of x_0, x_1, x_2 true, x_1 forced true
        let mut ts = TwoSat::new(3);
        ts.max_one(&[(0, true), (1, true), (2, true)]);
        ts.set_true(1);
        let sol = ts.solve().unwrap();
        assert!(!sol[0], "x_0 must be false");
        assert!(sol[1], "x_1 must be true");
        assert!(!sol[2], "x_2 must be false");
    }

    #[test]
    fn max_one_five_vars() {
        // At most one of 5 variables
        let n = 5;
        let mut ts = TwoSat::new(n);
        let lits: Vec<(usize, bool)> = (0..n).map(|i| (i, true)).collect();
        ts.max_one(&lits);
        let sol = ts.solve().unwrap();
        let count = sol.iter().filter(|&&v| v).count();
        assert!(count <= 1, "max_one(5) violated: {} are true", count);
    }

    #[test]
    fn max_one_five_vars_all_forced_conflict() {
        // At most one of 5, but force 2 true -> unsatisfiable
        let n = 5;
        let mut ts = TwoSat::new(n);
        let lits: Vec<(usize, bool)> = (0..n).map(|i| (i, true)).collect();
        ts.max_one(&lits);
        ts.set_true(0);
        ts.set_true(3);
        assert!(ts.solve().is_none(), "should be unsatisfiable");
    }

    #[test]
    fn max_one_with_or() {
        // At most one of x_0..x_3 true, and at least one must be true
        // (exactly one must be true)
        let n = 4;
        let mut ts = TwoSat::new(n);
        let lits: Vec<(usize, bool)> = (0..n).map(|i| (i, true)).collect();
        ts.max_one(&lits);
        // Add pairwise OR to force at least one: x_0 OR x_1 OR x_2 OR x_3
        // We can encode "at least one" by chaining: for 2-SAT we do it pairwise
        // with auxiliary. Simpler: add_or for consecutive pairs won't work for "at least one of 4".
        // Instead: just force x_2 true.
        ts.set_true(2);
        let sol = ts.solve().unwrap();
        assert!(!sol[0]);
        assert!(!sol[1]);
        assert!(sol[2]);
        assert!(!sol[3]);
    }
}
