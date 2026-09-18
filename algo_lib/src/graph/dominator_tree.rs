use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::Graph;

pub trait DominatorTree {
    /// Immediate dominator of every vertex reachable from `root` in a
    /// directed graph; `None` for the root and for unreachable vertices.
    /// Lengauer-Tarjan with path compression, O(m log n).
    fn dominator_tree(&self, root: usize) -> Vec<Option<usize>>;
}

const NONE: u32 = u32::MAX;

impl<E: EdgeTrait> DominatorTree for Graph<E> {
    fn dominator_tree(&self, root: usize) -> Vec<Option<usize>> {
        assert!(!E::REVERSABLE);
        let n = self.vertex_count();
        // DFS numbering; everything below is indexed by DFS number.
        let mut number = vec![NONE; n];
        let mut vertex: Vec<u32> = Vec::new();
        let mut parent: Vec<u32> = Vec::new();
        let mut stack: Vec<(u32, u32)> = vec![(root as u32, self.head_edge(root))];
        number[root] = 0;
        vertex.push(root as u32);
        parent.push(0);
        while let Some(frame) = stack.last_mut() {
            let (v, cursor) = (frame.0 as usize, frame.1);
            if cursor == NONE {
                stack.pop();
                continue;
            }
            frame.1 = self.step_edge(v, cursor);
            let to = self.edge_at(v, cursor).to();
            if number[to] == NONE {
                number[to] = vertex.len() as u32;
                vertex.push(to as u32);
                parent.push(number[v]);
                stack.push((to as u32, self.head_edge(to)));
            }
        }
        let count = vertex.len();
        // Predecessors among reachable vertices, as DFS numbers (CSR).
        let mut start = vec![0u32; count + 1];
        for &v in &vertex {
            for e in self.adj(v as usize).iter() {
                start[number[e.to()] as usize + 1] += 1;
            }
        }
        for i in 0..count {
            start[i + 1] += start[i];
        }
        let mut fill = start.clone();
        let mut preds = vec![0u32; start[count] as usize];
        for (i, &v) in vertex.iter().enumerate() {
            for e in self.adj(v as usize).iter() {
                let to = number[e.to()] as usize;
                preds[fill[to] as usize] = i as u32;
                fill[to] += 1;
            }
        }

        let mut semi: Vec<u32> = (0..count as u32).collect();
        let mut idom = vec![0u32; count];
        let mut ancestor = vec![NONE; count];
        let mut label: Vec<u32> = (0..count as u32).collect();
        // Buckets as intrusive lists: bucket_head[w] -> next_in_bucket[..]
        let mut bucket_head = vec![NONE; count];
        let mut bucket_next = vec![NONE; count];
        let mut path: Vec<u32> = Vec::new();

        // eval(v): vertex with minimal semi on the compressed path above v.
        let mut eval =
            |v: u32, ancestor: &mut Vec<u32>, label: &mut Vec<u32>, semi: &Vec<u32>| -> u32 {
                if ancestor[v as usize] == NONE {
                    return v;
                }
                path.clear();
                let mut cur = v;
                while ancestor[ancestor[cur as usize] as usize] != NONE {
                    path.push(cur);
                    cur = ancestor[cur as usize];
                }
                // cur's ancestor is a root of the forest; compress top-down.
                while let Some(x) = path.pop() {
                    let a = ancestor[x as usize];
                    if semi[label[a as usize] as usize] < semi[label[x as usize] as usize] {
                        label[x as usize] = label[a as usize];
                    }
                    ancestor[x as usize] = ancestor[a as usize];
                }
                label[v as usize]
            };

        for w in (1..count).rev() {
            for &v in &preds[start[w] as usize..start[w + 1] as usize] {
                let u = eval(v, &mut ancestor, &mut label, &semi);
                if semi[u as usize] < semi[w] {
                    semi[w] = semi[u as usize];
                }
            }
            let s = semi[w] as usize;
            bucket_next[w] = bucket_head[s];
            bucket_head[s] = w as u32;
            let p = parent[w] as usize;
            ancestor[w] = p as u32;
            let mut v = bucket_head[p];
            bucket_head[p] = NONE;
            while v != NONE {
                let u = eval(v, &mut ancestor, &mut label, &semi);
                idom[v as usize] = if semi[u as usize] < semi[v as usize] {
                    u
                } else {
                    p as u32
                };
                v = bucket_next[v as usize];
            }
        }
        for w in 1..count {
            if idom[w] != semi[w] {
                idom[w] = idom[idom[w] as usize];
            }
        }
        let mut res = vec![None; n];
        for w in 1..count {
            res[vertex[w] as usize] = Some(vertex[idom[w] as usize] as usize);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::DominatorTree;
    use crate::graph::Graph;
    use crate::misc::random::{Random, RandomTrait};

    fn reachable(
        n: usize,
        edges: &[(usize, usize)],
        root: usize,
        removed: Option<usize>,
    ) -> Vec<bool> {
        let mut seen = vec![false; n];
        if Some(root) == removed {
            return seen;
        }
        seen[root] = true;
        let mut stack = vec![root];
        while let Some(v) = stack.pop() {
            for &(a, b) in edges {
                if a == v && Some(b) != removed && !seen[b] {
                    seen[b] = true;
                    stack.push(b);
                }
            }
        }
        seen
    }

    #[test]
    fn matches_brute_force() {
        let mut rng = Random::new_with_seed(251);
        for _ in 0..400 {
            let n = rng.gen_range(1..=9usize);
            let m = rng.gen_range(0..=3 * n);
            let edges: Vec<(usize, usize)> = (0..m)
                .map(|_| (rng.gen_range(0..n), rng.gen_range(0..n)))
                .collect();
            let root = rng.gen_range(0..n);
            let base = reachable(n, &edges, root, None);
            // dominators[v] = vertices (other than v) whose removal disconnects v
            let dominators: Vec<Vec<usize>> = (0..n)
                .map(|v| {
                    (0..n)
                        .filter(|&u| u != v && base[v] && !reachable(n, &edges, root, Some(u))[v])
                        .collect()
                })
                .collect();
            let got = Graph::with_edges(n, &edges).dominator_tree(root);
            for v in 0..n {
                if !base[v] || v == root {
                    assert_eq!(got[v], None, "v={v} root={root} edges={edges:?}");
                    continue;
                }
                // the immediate dominator is the one dominated by all others
                let expected = dominators[v]
                    .iter()
                    .copied()
                    .find(|&d| {
                        dominators[v]
                            .iter()
                            .all(|&o| o == d || dominators[d].contains(&o))
                    })
                    .unwrap();
                assert_eq!(got[v], Some(expected), "v={v} root={root} edges={edges:?}");
            }
        }
    }

    #[test]
    fn long_chain_needs_no_deep_stack() {
        let n = 200_000;
        let handle = std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(move || {
                let edges: Vec<(usize, usize)> = (1..n).map(|v| (v - 1, v)).collect();
                let idom = Graph::with_edges(n, &edges).dominator_tree(0);
                (1..n).all(|v| idom[v] == Some(v - 1))
            })
            .unwrap();
        assert!(handle.join().unwrap());
    }
}
