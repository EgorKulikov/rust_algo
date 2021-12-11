use crate::collections::arr2d::Arr2d;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::graph::graph::Graph;
use crate::numbers::num_traits::bit_ops::Bits;

pub struct LCA {
    position: Vec<u32>,
    lca_arr: Arr2d<u32>,
    level: Vec<u32>,
    parent: Vec<u32>,
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
        let from = self.position[first].min(self.position[second]) as usize;
        let to = self.position[first].max(self.position[second]) as usize;
        let lv = (u32::bits() - ((to - from) as u32).leading_zeros() - 1) as usize;
        self.lca_arr[(lv, from)].min(self.lca_arr[(lv, to + 1 - (1 << lv))]) as usize
    }

    pub fn path_length(&self, first: usize, second: usize) -> usize {
        (self.level[first] + self.level[second] - 2 * self.level[self.lca(first, second)]) as usize
    }
}

pub trait LCATrait {
    fn lca_with_root(&self, root: usize) -> LCA;

    fn lca(&self) -> LCA {
        self.lca_with_root(0)
    }
}

impl<E: EdgeTrait> LCATrait for Graph<E> {
    fn lca_with_root(&self, root: usize) -> LCA {
        debug_assert!(self.is_tree());
        let vertex_count = self.vertex_count();
        let mut order = vec![0u32; 2 * vertex_count - 1];
        let mut position = vec![vertex_count as u32; vertex_count];
        let mut level = vec![0; vertex_count];
        let mut index = vec![0u32; vertex_count];
        let mut parent = vec![0; vertex_count];
        let mut stack = vec![0u32; vertex_count];
        stack[0] = root as u32;
        let mut size = 1usize;
        let mut j = 0usize;
        parent[root] = root as u32;
        while size > 0 {
            size -= 1;
            let vertex = stack[size] as usize;
            if (position[vertex] as usize) == vertex_count {
                position[vertex] = j as u32;
            }
            order[j] = vertex as u32;
            j += 1;
            while (index[vertex] as usize) < self[vertex].len()
                && (parent[vertex] as usize) == self[vertex][index[vertex] as usize].to()
            {
                index[vertex] += 1;
            }
            if (index[vertex] as usize) < self[vertex].len() {
                stack[size] = vertex as u32;
                size += 1;
                let to = self[vertex][index[vertex] as usize].to();
                stack[size] = to as u32;
                size += 1;
                parent[to] = vertex as u32;
                level[to] = level[vertex] + 1;
                index[vertex] += 1;
            }
        }
        let mut lca_arr = Arr2d::new(
            (u32::bits() - (2 * vertex_count - 1).leading_zeros()) as usize,
            2 * vertex_count - 1,
            0,
        );
        for i in 0..(2 * vertex_count - 1) {
            lca_arr[(0, i)] = order[i];
        }

        for i in 1..lca_arr.d1() {
            for j in 0..lca_arr.d2() {
                let other = j + (1 << (i - 1));
                if other < lca_arr.d2() {
                    lca_arr[(i, j)] = lca_arr[(i - 1, j)].min(lca_arr[(i - 1, other)])
                } else {
                    lca_arr[(i, j)] = lca_arr[(i - 1, j)];
                }
            }
        }

        LCA {
            position,
            lca_arr,
            level,
            parent,
        }
    }
}
