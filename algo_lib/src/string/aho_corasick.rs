use crate::collections::slice_ext::indices::Indices;
use crate::collections::slice_ext::splits::Split;

const NULL: u32 = u32::MAX;

pub trait ACPayload {
    fn add_single(&mut self, id: usize);
    fn add_node(&mut self, other: &Self);
}

#[derive(Debug)]
pub struct Node<P, const K: usize> {
    next: [u32; K],
    parent: u32,
    parent_char: u8,
    link: u32,
    payload: P,
}

impl<P: Default, const K: usize> Default for Node<P, K> {
    fn default() -> Self {
        Self {
            next: [NULL; K],
            parent: NULL,
            parent_char: 0,
            link: NULL,
            payload: P::default(),
        }
    }
}

#[derive(Debug)]
pub struct AhoCorasick<P, const K: usize, const BASE: u8> {
    nodes: Vec<Node<P, K>>,
}

pub type AhoCorasickLowercase<P> = AhoCorasick<P, 26, b'a'>;
pub type AhoCorasickUppercase<P> = AhoCorasick<P, 26, b'A'>;
pub type AhoCorasickDigits<P> = AhoCorasick<P, 10, b'0'>;
pub type AhoCorasickAlphanumeric<P> = AhoCorasick<P, 75, b'0'>;
pub type AhoCorasickLetters<P> = AhoCorasick<P, 58, b'A'>;

pub struct Iter<'s, P, S, const K: usize, const BASE: u8> {
    ac: &'s AhoCorasick<P, K, BASE>,
    s: &'s S,
    pos: usize,
    node: usize,
}

impl<'s, P: Default + ACPayload, S: AsRef<[u8]>, const K: usize, const BASE: u8> Iterator
    for Iter<'s, P, S, K, BASE>
{
    type Item = &'s P;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.s.as_ref().len();
        let res = if self.pos <= len {
            Some(&self.ac.nodes[self.node].payload)
        } else {
            None
        };
        if self.pos < len {
            let c = self.s.as_ref()[self.pos];
            self.node = self.ac.advance(self.node, c);
        }
        self.pos += 1;
        res
    }
}

impl<P: Default + ACPayload, const K: usize, const BASE: u8> AhoCorasick<P, K, BASE> {
    pub fn new(s: &[impl AsRef<[u8]>]) -> Self {
        let mut res = Self {
            nodes: vec![Node::default()],
        };
        for (i, s) in s.iter().enumerate() {
            res.add(i, s.as_ref());
        }
        res.build();
        res
    }

    pub fn iterate<'a, S: AsRef<[u8]>>(&'a self, s: &'a S) -> Iter<'a, P, S, K, BASE> {
        Iter {
            ac: self,
            s,
            pos: 0,
            node: 0,
        }
    }

    pub fn advance(&self, node: usize, c: u8) -> usize {
        self.nodes[node].next[(c - BASE) as usize] as usize
    }

    fn add(&mut self, id: usize, s: &[u8]) {
        let mut node = 0;
        for &c in s {
            let c = c - BASE;
            if self.nodes[node as usize].next[c as usize] == NULL {
                self.nodes[node as usize].next[c as usize] = self.nodes.len() as u32;
                let next = Node {
                    parent: node,
                    parent_char: c,
                    ..Default::default()
                };
                self.nodes.push(next);
            }
            node = self.nodes[node as usize].next[c as usize];
        }
        self.nodes[node as usize].payload.add_single(id);
    }

    fn get_link(&mut self, node: usize) -> u32 {
        if self.nodes[node].link == NULL {
            let parent = self.nodes[node].parent;
            self.nodes[node].link = if node == 0 || parent == 0 {
                0
            } else {
                let parent_link = self.get_link(parent as usize) as usize;
                let link = self.go(parent_link, self.nodes[node].parent_char);
                self.get_link(link as usize);
                let (node_node, link_node) = self.nodes.two_mut(node, link as usize);
                node_node.payload.add_node(&link_node.payload);
                link
            };
        }
        self.nodes[node].link
    }

    fn go(&mut self, node: usize, c: u8) -> u32 {
        if self.nodes[node].next[c as usize] == NULL {
            let go = if node == 0 {
                0
            } else {
                let link = self.get_link(node) as usize;
                self.go(link, c)
            };
            self.nodes[node].next[c as usize] = go;
        }
        self.nodes[node].next[c as usize]
    }

    fn build(&mut self) {
        for node in self.nodes.indices() {
            self.get_link(node);
            for i in 0..K {
                self.go(node, i as u8);
            }
        }
    }
}
