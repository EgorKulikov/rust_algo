use crate::collections::dsu::DSU;
use crate::graph::edge_trait::EdgeTrait;
use std::ops::{Index, IndexMut};
// use std::slice::Iter;

pub struct Graph<E: EdgeTrait> {
    edges: Vec<Vec<E>>,
    edge_count: usize,
}

impl<E: EdgeTrait> Graph<E> {
    pub fn new(vertex_count: usize) -> Self {
        Self {
            edges: vec![Vec::new(); vertex_count],
            edge_count: 0,
        }
    }

    pub fn add_edge(&mut self, from: usize, mut edge: E) -> usize {
        let to = edge.to();
        assert!(to < self.edges.len());
        let direct_id = self.edges[from].len();
        edge.set_id(self.edge_count);
        self.edges[from].push(edge);
        if E::REVERSABLE {
            let rev_id = self.edges[to].len();
            self.edges[from][direct_id].set_reverse_id(rev_id);
            let mut rev_edge = self.edges[from][direct_id].reverse_edge(from);
            rev_edge.set_id(self.edge_count);
            rev_edge.set_reverse_id(direct_id);
            self.edges[to].push(rev_edge);
        }
        self.edge_count += 1;
        direct_id
    }

    pub fn add_vertices(&mut self, cnt: usize) {
        self.edges.resize(self.edges.len() + cnt, Vec::new());
    }

    pub fn clear(&mut self) {
        self.edge_count = 0;
        for ve in self.edges.iter_mut() {
            ve.clear();
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.edges.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn is_tree(&self) -> bool {
        if !E::BIDIRECTIONAL || self.edge_count + 1 != self.vertex_count() {
            false
        } else {
            let mut dsu = DSU::new(self.vertex_count());
            for i in 0..self.vertex_count() {
                for e in self[i].iter() {
                    dsu.join(i, e.to());
                }
            }
            dsu.count() == 1
        }
    }

    // pub fn dfs_with_root<F, Args, Output>(&self, f: F, root: usize, args: Args) -> Output
    // where
    //     F: Fn(
    //         &dyn Fn(usize, usize, Args) -> Output,
    //         usize,
    //         Args,
    //         &dyn Iterator<Item = &E>,
    //     ) -> Output,
    // {
    //     self.call_dfs(&f, root, self.vertex_count(), args)
    // }
    //
    // fn call_dfs<F, Args, Output>(&self, f: &F, root: usize, parent: usize, args: Args) -> Output
    // where
    //     F: Fn(
    //         &dyn Fn(usize, usize, Args) -> Output,
    //         usize,
    //         Args,
    //         &dyn Iterator<Item = &E>,
    //     ) -> Output,
    // {
    //     let iter = DFSIter {
    //         iter: self[root].iter(),
    //         skip: parent,
    //     };
    //     f(
    //         &|root, parent, args| self.call_dfs(f, root, parent, args),
    //         root,
    //         args,
    //         &iter,
    //     )
    // }
}

// struct DFSIter<'a, E: EdgeTrait> {
//     iter: Iter<'a, E>,
//     skip: usize,
// }
//
// impl<'a, E: EdgeTrait> Iterator for DFSIter<'a, E> {
//     type Item = &'a E;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         while let Some(e) = self.iter.next() {
//             if e.to() == self.skip {
//                 return Some(e);
//             }
//         }
//         None
//     }
// }

impl<E: EdgeTrait> Index<usize> for Graph<E> {
    type Output = [E];

    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
    }
}

impl<E: EdgeTrait> IndexMut<usize> for Graph<E> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.edges[index]
    }
}
