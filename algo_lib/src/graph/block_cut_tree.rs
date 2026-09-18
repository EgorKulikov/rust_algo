use crate::collections::bit_set::BitSet;
use crate::collections::min_max::MinimMaxim;
use crate::graph::edges::bi_edge::BiEdge;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::Graph;

pub struct BlockCutTree {
    // For each original vertex, which block-cut tree node it maps to.
    // Cut vertices get their own node; non-cut vertices belong to exactly one block.
    pub vertex_node: Vec<usize>,
    // Total number of nodes in the block-cut tree (blocks + cut vertices).
    pub node_count: usize,
    // The block-cut tree itself.
    pub tree: Graph<BiEdge<()>>,
    // Whether a block-cut tree node is a cut vertex node.
    pub is_cut: BitSet,
    // For block nodes: which original vertices belong to this block.
    // For cut nodes: contains just that cut vertex.
    pub node_vertices: Vec<Vec<usize>>,
}

pub trait BlockCutTreeBuild {
    fn block_cut_tree(&self) -> BlockCutTree;
}

impl<E: EdgeTrait> BlockCutTreeBuild for Graph<E> {
    fn block_cut_tree(&self) -> BlockCutTree {
        assert!(E::REVERSABLE);
        let n = self.vertex_count();
        let mut timer = 0;
        let mut tin = vec![0u32; n];
        let mut fup = vec![0u32; n];
        let mut used = BitSet::new(n);
        let mut is_cut_orig = BitSet::new(n);
        let mut stack = Vec::new();
        let mut blocks: Vec<Vec<usize>> = Vec::new();

        const NO_PARENT: u32 = u32::MAX;
        // Frame: (vertex, parent, edge cursor, tree children so far).
        let mut frames: Vec<(u32, u32, u32, u32)> = Vec::new();
        for root in 0..n {
            if used[root] {
                continue;
            }
            used.set(root);
            tin[root] = timer;
            fup[root] = timer;
            timer += 1;
            frames.push((root as u32, NO_PARENT, self.head_edge(root), 0));
            while let Some(frame) = frames.last_mut() {
                let (vert, prev, cursor) = (frame.0 as usize, frame.1, frame.2);
                if cursor == u32::MAX {
                    let children = frame.3;
                    frames.pop();
                    if prev == NO_PARENT {
                        if children == 0 {
                            // Isolated vertex (no edges)
                            blocks.push(vec![vert]);
                        }
                        continue;
                    }
                    let parent = frames.last().unwrap();
                    let (up, up_prev, up_children) = (parent.0 as usize, parent.1, parent.3);
                    let cand = fup[vert];
                    fup[up].minim(cand);
                    if up_prev == NO_PARENT {
                        if up_children > 1 {
                            is_cut_orig.set(up);
                        }
                    } else if fup[vert] >= tin[up] {
                        is_cut_orig.set(up);
                    }
                    if fup[vert] >= tin[up] {
                        let mut block = Vec::new();
                        while let Some((u, v)) = stack.pop() {
                            block.push(u);
                            block.push(v);
                            if u == up && v == vert {
                                break;
                            }
                        }
                        block.sort();
                        block.dedup();
                        blocks.push(block);
                    }
                    continue;
                }
                frame.2 = self.step_edge(vert, cursor);
                let to = self.edge_at(vert, cursor).to();
                if prev != NO_PARENT && to == prev as usize {
                    continue;
                }
                if used[to] {
                    let cand = tin[to];
                    fup[vert].minim(cand);
                } else {
                    stack.push((vert, to));
                    frame.3 += 1;
                    used.set(to);
                    tin[to] = timer;
                    fup[to] = timer;
                    timer += 1;
                    frames.push((to as u32, vert as u32, self.head_edge(to), 0));
                }
            }
        }

        // Build the block-cut tree.
        // First, assign node IDs to cut vertices.
        let mut vertex_node = vec![usize::MAX; n];
        let mut node_count = 0;
        let mut node_vertices: Vec<Vec<usize>> = Vec::new();

        for v in 0..n {
            if is_cut_orig[v] {
                vertex_node[v] = node_count;
                node_vertices.push(vec![v]);
                node_count += 1;
            }
        }

        // Then assign node IDs to blocks.
        let mut tree_edges = Vec::new();
        for block in &blocks {
            let block_node = node_count;
            node_vertices.push(block.clone());
            node_count += 1;

            for &v in block {
                if is_cut_orig[v] {
                    tree_edges.push((vertex_node[v], block_node));
                } else {
                    vertex_node[v] = block_node;
                }
            }
        }

        let mut is_cut = BitSet::new(node_count);
        for v in 0..n {
            if is_cut_orig[v] {
                is_cut.set(vertex_node[v]);
            }
        }

        let tree = Graph::with_biedges(node_count, &tree_edges);

        BlockCutTree {
            vertex_node,
            node_count,
            tree,
            is_cut,
            node_vertices,
        }
    }
}

#[cfg(test)]
mod test {
    use super::BlockCutTreeBuild;
    use crate::graph::Graph;

    #[test]
    fn triangle_single_block() {
        // Triangle: one biconnected component, no cut vertices
        let g = Graph::with_biedges(3, &[(0, 1), (1, 2), (2, 0)]);
        let bct = g.block_cut_tree();
        assert_eq!(bct.node_count, 1);
        assert!(!bct.is_cut[0]);
        // All vertices map to the same block
        assert_eq!(bct.vertex_node[0], bct.vertex_node[1]);
        assert_eq!(bct.vertex_node[1], bct.vertex_node[2]);
    }

    #[test]
    fn path_graph() {
        // 0-1-2-3: each edge is its own block, vertices 1 and 2 are cut points
        let g = Graph::with_biedges(4, &[(0, 1), (1, 2), (2, 3)]);
        let bct = g.block_cut_tree();
        // 3 blocks + 2 cut vertices = 5 nodes
        assert_eq!(bct.node_count, 5);
        assert!(bct.is_cut[bct.vertex_node[1]]);
        assert!(bct.is_cut[bct.vertex_node[2]]);
        assert!(!bct.is_cut[bct.vertex_node[0]]);
        assert!(!bct.is_cut[bct.vertex_node[3]]);
    }

    #[test]
    fn two_triangles_sharing_vertex() {
        // Triangle 0-1-2 and triangle 2-3-4, vertex 2 is cut point
        let g = Graph::with_biedges(5, &[(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 2)]);
        let bct = g.block_cut_tree();
        // 2 blocks + 1 cut vertex = 3 nodes
        assert_eq!(bct.node_count, 3);
        assert!(bct.is_cut[bct.vertex_node[2]]);
        assert!(!bct.is_cut[bct.vertex_node[0]]);
        assert!(!bct.is_cut[bct.vertex_node[3]]);
        // Vertices in same triangle share a block
        assert_eq!(bct.vertex_node[0], bct.vertex_node[1]);
        assert_eq!(bct.vertex_node[3], bct.vertex_node[4]);
        assert_ne!(bct.vertex_node[0], bct.vertex_node[3]);
    }

    #[test]
    fn isolated_vertex() {
        let g = Graph::with_biedges(1, &[]);
        let bct = g.block_cut_tree();
        assert_eq!(bct.node_count, 1);
        assert!(!bct.is_cut[0]);
    }

    #[test]
    fn tree_is_valid() {
        // The block-cut tree should itself be a tree (or forest)
        let g = Graph::with_biedges(6, &[(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 5), (5, 3)]);
        let bct = g.block_cut_tree();
        // 3 blocks (0-1-2, 2-3, 3-4-5) + 2 cut vertices (2, 3) = 5 nodes
        assert_eq!(bct.node_count, 5);
        assert!(bct.tree.is_tree());
    }

    #[test]
    fn random_graphs_satisfy_invariants() {
        use crate::graph::cut_points::CutPointSearch;
        use crate::graph::edges::edge_trait::EdgeTrait;
        use crate::misc::random::{Random, RandomTrait};
        let mut rng = Random::new_with_seed(153);
        for _ in 0..300 {
            let n = rng.gen_range(1..=9usize);
            let m = rng.gen_range(0..=12usize);
            let edges: Vec<(usize, usize)> = (0..m)
                .map(|_| (rng.gen_range(0..n), rng.gen_range(0..n)))
                .filter(|&(a, b)| a != b)
                .collect();
            let graph = Graph::with_biedges(n, &edges);
            let bct = graph.block_cut_tree();
            let mut cuts = graph.cut_points();
            cuts.sort();
            let from_tree: Vec<usize> =
                (0..n).filter(|&v| bct.is_cut[bct.vertex_node[v]]).collect();
            assert_eq!(from_tree, cuts, "n={n} edges={edges:?}");
            assert_eq!(bct.node_vertices.len(), bct.node_count);
            // Every edge lies inside exactly one block.
            for &(a, b) in &edges {
                let holding = (0..bct.node_count)
                    .filter(|&k| !bct.is_cut[k])
                    .filter(|&k| {
                        bct.node_vertices[k].contains(&a) && bct.node_vertices[k].contains(&b)
                    })
                    .count();
                assert_eq!(holding, 1, "edge {a}-{b} in n={n} edges={edges:?}");
            }
            // Non-cut vertices belong to exactly one block; the tree is a forest.
            for v in 0..n {
                let blocks = (0..bct.node_count)
                    .filter(|&k| !bct.is_cut[k] && bct.node_vertices[k].contains(&v))
                    .count();
                if cuts.contains(&v) {
                    assert!(blocks >= 2);
                } else {
                    assert_eq!(blocks, 1);
                }
            }
            let tree_edges: usize = (0..bct.node_count)
                .map(|k| bct.tree.adj(k).iter().count())
                .sum::<usize>()
                / 2;
            let mut comp: Vec<usize> = (0..bct.node_count).collect();
            for k in 0..bct.node_count {
                for e in bct.tree.adj(k).iter() {
                    let (ca, cb) = (comp[k], comp[e.to()]);
                    for c in comp.iter_mut() {
                        if *c == cb {
                            *c = ca;
                        }
                    }
                }
            }
            let mut distinct = comp.clone();
            distinct.sort();
            distinct.dedup();
            assert_eq!(tree_edges + distinct.len(), bct.node_count, "not a forest");
        }
    }

    #[test]
    fn long_path_needs_no_deep_stack() {
        let n = 200_000;
        let handle = std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(move || {
                let edges: Vec<(usize, usize)> = (1..n).map(|v| (v - 1, v)).collect();
                Graph::with_biedges(n, &edges).block_cut_tree().node_count
            })
            .unwrap();
        // n - 1 blocks plus n - 2 cut vertices
        assert_eq!(handle.join().unwrap(), 2 * n - 3);
    }
}
