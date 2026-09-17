use crate::graph::edges::edge::Edge;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::Graph;

pub struct StronglyConnectedComponents {
    pub color: Vec<usize>,
    pub condensed: Graph<Edge<()>>,
}

pub trait StronglyConnectedComponentsTrait {
    fn strongly_connected_components(&self) -> StronglyConnectedComponents;
    fn strongly_connected_component_colors(&self) -> Vec<usize>;
}

const NONE: u32 = u32::MAX;

/// Iterative Tarjan. Returns component ids numbered in topological order of
/// the condensation (every edge goes from a lower or equal id to a higher
/// or equal id) and the number of components.
fn tarjan<E: EdgeTrait>(graph: &Graph<E>) -> (Vec<usize>, usize) {
    let n = graph.vertex_count();
    let mut index = vec![NONE; n];
    let mut low = vec![0u32; n];
    let mut comp = vec![NONE; n];
    let mut stack: Vec<u32> = Vec::with_capacity(n);
    let mut frames: Vec<(u32, u32)> = Vec::with_capacity(n);
    let mut timer = 0u32;
    let mut comps = 0u32;
    for s in 0..n {
        if index[s] != NONE {
            continue;
        }
        index[s] = timer;
        low[s] = timer;
        timer += 1;
        stack.push(s as u32);
        frames.push((s as u32, graph.head_edge(s)));
        while let Some(frame) = frames.last_mut() {
            let v = frame.0 as usize;
            let cursor = frame.1;
            if cursor != NONE {
                frame.1 = graph.step_edge(v, cursor);
                let w = graph.edge_at(v, cursor).to();
                if index[w] == NONE {
                    index[w] = timer;
                    low[w] = timer;
                    timer += 1;
                    stack.push(w as u32);
                    frames.push((w as u32, graph.head_edge(w)));
                } else if comp[w] == NONE && index[w] < low[v] {
                    low[v] = index[w];
                }
            } else {
                frames.pop();
                if low[v] == index[v] {
                    loop {
                        let w = stack.pop().unwrap() as usize;
                        comp[w] = comps;
                        if w == v {
                            break;
                        }
                    }
                    comps += 1;
                }
                if let Some(parent) = frames.last() {
                    let p = parent.0 as usize;
                    if low[v] < low[p] {
                        low[p] = low[v];
                    }
                }
            }
        }
    }
    // Tarjan finishes sink components first; reverse to get topological order.
    let color = comp.into_iter().map(|c| (comps - 1 - c) as usize).collect();
    (color, comps as usize)
}

impl<E: EdgeTrait> StronglyConnectedComponentsTrait for Graph<E> {
    fn strongly_connected_components(&self) -> StronglyConnectedComponents {
        assert!(!E::REVERSABLE);
        let n = self.vertex_count();
        let (color, comps) = tarjan(self);
        // Vertices bucketed by color, so each source component's edges are
        // emitted together and deduplicated with one marker per target.
        let mut start = vec![0usize; comps + 1];
        for &c in &color {
            start[c + 1] += 1;
        }
        for c in 0..comps {
            start[c + 1] += start[c];
        }
        let mut order = vec![0usize; n];
        let mut pos = start.clone();
        for (v, &c) in color.iter().enumerate() {
            order[pos[c]] = v;
            pos[c] += 1;
        }
        let mut res = Graph::new_linked(comps);
        let mut seen = vec![usize::MAX; comps];
        for c in 0..comps {
            for &v in &order[start[c]..start[c + 1]] {
                for e in self.adj(v).iter() {
                    let to = color[e.to()];
                    if to != c && seen[to] != c {
                        seen[to] = c;
                        res.add_edge(Edge::new(c, to));
                    }
                }
            }
        }
        StronglyConnectedComponents {
            color,
            condensed: res,
        }
    }

    fn strongly_connected_component_colors(&self) -> Vec<usize> {
        assert!(!E::REVERSABLE);
        tarjan(self).0
    }
}

#[cfg(test)]
mod test {
    use super::StronglyConnectedComponentsTrait;
    use crate::graph::edges::edge_trait::EdgeTrait;
    use crate::graph::Graph;

    #[test]
    fn triangle_single_scc() {
        let graph = Graph::with_edges(3, &[(0, 1), (1, 2), (2, 0)]);
        let scc = graph.strongly_connected_components();
        assert_eq!(scc.color[0], scc.color[1]);
        assert_eq!(scc.color[1], scc.color[2]);
        assert_eq!(scc.condensed.vertex_count(), 1);
    }

    #[test]
    fn two_sccs() {
        // Cycle 0->1->2->0 plus pendant 2->3
        let graph = Graph::with_edges(4, &[(0, 1), (1, 2), (2, 0), (2, 3)]);
        let scc = graph.strongly_connected_components();
        assert_eq!(scc.color[0], scc.color[1]);
        assert_eq!(scc.color[0], scc.color[2]);
        assert_ne!(scc.color[0], scc.color[3]);
        assert_eq!(scc.condensed.vertex_count(), 2);
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn random_graphs_match_reachability() {
        use crate::misc::random::{Random, RandomTrait};
        let mut rng = Random::new_with_seed(31);
        for _ in 0..300 {
            let n = rng.gen_range(1..=12usize);
            let m = rng.gen_range(0..=3 * n);
            let edges: Vec<(usize, usize)> = (0..m)
                .map(|_| (rng.gen_range(0..n), rng.gen_range(0..n)))
                .collect();
            let graph = Graph::with_edges(n, &edges);
            let mut reach = vec![vec![false; n]; n];
            for i in 0..n {
                reach[i][i] = true;
            }
            for &(a, b) in &edges {
                reach[a][b] = true;
            }
            for k in 0..n {
                for i in 0..n {
                    for j in 0..n {
                        reach[i][j] |= reach[i][k] && reach[k][j];
                    }
                }
            }
            let scc = graph.strongly_connected_components();
            assert_eq!(scc.color, graph.strongly_connected_component_colors());
            for i in 0..n {
                for j in 0..n {
                    assert_eq!(scc.color[i] == scc.color[j], reach[i][j] && reach[j][i]);
                }
            }
            let comps = scc.color.iter().max().unwrap() + 1;
            assert!(scc.color.iter().all(|&c| c < comps));
            assert_eq!(scc.condensed.vertex_count(), comps);
            for &(a, b) in &edges {
                assert!(
                    scc.color[a] <= scc.color[b],
                    "edge {a}->{b} breaks topological order"
                );
            }
            let mut expected = std::collections::BTreeSet::new();
            for &(a, b) in &edges {
                if scc.color[a] != scc.color[b] {
                    expected.insert((scc.color[a], scc.color[b]));
                }
            }
            let mut got = std::collections::BTreeSet::new();
            let mut count = 0;
            for c in 0..comps {
                for e in scc.condensed.adj(c).iter() {
                    got.insert((c, e.to()));
                    count += 1;
                }
            }
            assert_eq!(got, expected);
            assert_eq!(count, got.len(), "duplicate condensed edges");
        }
    }

    #[test]
    fn dag_all_singletons() {
        let graph = Graph::with_edges(3, &[(0, 1), (1, 2)]);
        let scc = graph.strongly_connected_components();
        assert_ne!(scc.color[0], scc.color[1]);
        assert_ne!(scc.color[1], scc.color[2]);
        assert_ne!(scc.color[0], scc.color[2]);
        assert_eq!(scc.condensed.vertex_count(), 3);
    }
}
