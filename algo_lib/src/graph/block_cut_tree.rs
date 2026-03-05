use crate::collections::bit_set::BitSet;
use crate::collections::min_max::MinimMaxim;
use crate::graph::edges::bi_edge::BiEdge;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::Graph;
use crate::misc::recursive_function::{Callable2, RecursiveFunction2};

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

        for i in 0..n {
            if !used[i] {
                let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                    used.set(vert);
                    tin[vert] = timer;
                    fup[vert] = timer;
                    timer += 1;
                    let mut children = 0u32;
                    for e in &self[vert] {
                        let to = e.to();
                        if to == prev {
                            continue;
                        }
                        if used[to] {
                            fup[vert].minim(tin[to]);
                        } else {
                            stack.push((vert, to));
                            children += 1;
                            f.call(to, vert);
                            let cand = fup[to];
                            fup[vert].minim(cand);

                            if prev == n {
                                if children > 1 {
                                    is_cut_orig.set(vert);
                                }
                            } else if fup[to] >= tin[vert] {
                                is_cut_orig.set(vert);
                            }

                            if fup[to] >= tin[vert] {
                                let mut block = Vec::new();
                                while let Some((u, v)) = stack.pop() {
                                    block.push(u);
                                    block.push(v);
                                    if u == vert && v == to {
                                        break;
                                    }
                                }
                                block.sort();
                                block.dedup();
                                blocks.push(block);
                            }
                        }
                    }
                    // Isolated vertex (no edges)
                    if prev == n && children == 0 {
                        blocks.push(vec![vert]);
                    }
                });
                dfs.call(i, n);
            }
        }

        // Build the block-cut tree.
        // First, assign node IDs to cut vertices.
        let mut vertex_node = vec![usize::MAX; n];
        let mut node_count = 0;
        let mut is_cut = BitSet::new(n + blocks.len());
        let mut node_vertices: Vec<Vec<usize>> = Vec::new();

        for v in 0..n {
            if is_cut_orig[v] {
                vertex_node[v] = node_count;
                is_cut.set(node_count);
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

        is_cut = BitSet::new(node_count);
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
}
