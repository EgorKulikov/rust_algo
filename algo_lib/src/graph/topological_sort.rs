use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph::Graph;
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
