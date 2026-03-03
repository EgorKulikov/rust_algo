use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::Graph;
use std::collections::VecDeque;

pub trait TopologicalSort {
    fn topological_sort(&self) -> Option<Vec<usize>>;
}

impl<E: EdgeTrait> TopologicalSort for Graph<E> {
    fn topological_sort(&self) -> Option<Vec<usize>> {
        assert!(!E::REVERSABLE);
        let n = self.vertex_count();
        let mut res = Vec::with_capacity(n);
        let mut degree = vec![0u32; n];
        for i in 0..n {
            for e in self[i].iter() {
                degree[e.to()] += 1;
            }
        }
        let mut queue = VecDeque::new();
        for (i, deg) in degree.iter().enumerate() {
            if *deg == 0 {
                queue.push_back(i);
            }
        }
        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            res.push(cur);
            for e in self[cur].iter() {
                let to = e.to();
                degree[to] -= 1;
                if degree[to] == 0 {
                    queue.push_back(to);
                }
            }
        }
        if res.len() == n {
            Some(res)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::TopologicalSort;
    use crate::graph::Graph;

    #[test]
    fn dag_valid_order() {
        // Diamond DAG: 0->1, 0->2, 1->3, 2->3
        let graph = Graph::with_edges(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        let order = graph.topological_sort().unwrap();
        let mut pos = vec![0; 4];
        for (i, &v) in order.iter().enumerate() {
            pos[v] = i;
        }
        for &(u, v) in &[(0, 1), (0, 2), (1, 3), (2, 3)] {
            assert!(pos[u] < pos[v]);
        }
    }

    #[test]
    fn cycle_returns_none() {
        let graph = Graph::with_edges(3, &[(0, 1), (1, 2), (2, 0)]);
        assert!(graph.topological_sort().is_none());
    }

    #[test]
    fn single_vertex() {
        let graph = Graph::with_edges(1, &[]);
        assert_eq!(graph.topological_sort(), Some(vec![0]));
    }
}
