//! Palindromic tree (eertree): one node per distinct palindromic substring,
//! built online in O(n log sigma)-free O(n * average degree) with children
//! kept as intrusive edge lists (12 bytes per edge instead of a table per
//! node).

const NONE: u32 = u32::MAX;

#[derive(Clone)]
pub struct PalindromicTree {
    text: Vec<u8>,
    /// Node 0 is the root of odd palindromes (length -1), node 1 of even ones.
    len: Vec<i32>,
    link: Vec<u32>,
    count: Vec<u32>,
    first_edge: Vec<u32>,
    edge_char: Vec<u8>,
    edge_to: Vec<u32>,
    edge_next: Vec<u32>,
    /// Node of the longest palindromic suffix of the text so far.
    last: u32,
}

impl Default for PalindromicTree {
    fn default() -> Self {
        Self::new()
    }
}

impl PalindromicTree {
    pub fn new() -> Self {
        Self {
            text: Vec::new(),
            len: vec![-1, 0],
            link: vec![0, 0],
            count: vec![0, 0],
            first_edge: vec![NONE, NONE],
            edge_char: Vec::new(),
            edge_to: Vec::new(),
            edge_next: Vec::new(),
            last: 1,
        }
    }

    pub fn from_slice(s: &[u8]) -> Self {
        let mut tree = Self::new();
        for &c in s {
            tree.push(c);
        }
        tree
    }

    fn child(&self, node: u32, c: u8) -> u32 {
        let mut e = self.first_edge[node as usize];
        while e != NONE {
            if self.edge_char[e as usize] == c {
                return self.edge_to[e as usize];
            }
            e = self.edge_next[e as usize];
        }
        NONE
    }

    /// Longest suffix palindrome `v` of the current text, starting the search
    /// at `node`, such that `c v c` ends at the new position.
    fn extendable(&self, mut node: u32, at: usize) -> u32 {
        loop {
            let l = self.len[node as usize];
            let before = at as i64 - l as i64 - 1;
            if before >= 0 && self.text[before as usize] == self.text[at] {
                return node;
            }
            node = self.link[node as usize];
        }
    }

    /// Appends a character; returns the node of the longest palindromic
    /// suffix of the new text.
    pub fn push(&mut self, c: u8) -> usize {
        let at = self.text.len();
        self.text.push(c);
        let parent = self.extendable(self.last, at);
        let mut node = self.child(parent, c);
        if node == NONE {
            node = self.len.len() as u32;
            let link = if self.len[parent as usize] == -1 {
                1
            } else {
                let up = self.extendable(self.link[parent as usize], at);
                self.child(up, c)
            };
            self.len.push(self.len[parent as usize] + 2);
            self.link.push(link);
            self.count.push(0);
            self.first_edge.push(NONE);
            self.edge_char.push(c);
            self.edge_to.push(node);
            self.edge_next.push(self.first_edge[parent as usize]);
            self.first_edge[parent as usize] = self.edge_char.len() as u32 - 1;
        }
        self.count[node as usize] += 1;
        self.last = node;
        node as usize
    }

    /// Number of nodes including the two roots (nodes 0 and 1).
    pub fn node_count(&self) -> usize {
        self.len.len()
    }

    /// Number of distinct nonempty palindromic substrings.
    pub fn distinct_palindromes(&self) -> usize {
        self.len.len() - 2
    }

    pub fn len(&self, node: usize) -> usize {
        self.len[node].max(0) as usize
    }

    /// Suffix link: the longest proper palindromic suffix of the node.
    pub fn link(&self, node: usize) -> usize {
        self.link[node] as usize
    }

    /// Occurrences of every node's palindrome in the text (nodes are created
    /// in an order where links point to earlier nodes).
    pub fn occurrences(&self) -> Vec<u64> {
        let mut occ: Vec<u64> = self.count.iter().map(|&c| c as u64).collect();
        for node in (2..occ.len()).rev() {
            let up = self.link[node] as usize;
            occ[up] += occ[node];
        }
        occ
    }
}

#[cfg(test)]
mod tests {
    use super::PalindromicTree;
    use crate::misc::random::{Random, RandomTrait};
    use std::collections::BTreeMap;

    #[test]
    fn matches_naive() {
        let mut rng = Random::new_with_seed(291);
        for _ in 0..300 {
            let n = rng.gen_range(0..=40usize);
            let alphabet = rng.gen_range(1..=3u8);
            let s: Vec<u8> = (0..n).map(|_| b'a' + rng.gen_range(0..alphabet)).collect();
            let mut naive: BTreeMap<&[u8], u64> = BTreeMap::new();
            for l in 0..n {
                for r in l + 1..=n {
                    let w = &s[l..r];
                    if w.iter().eq(w.iter().rev()) {
                        *naive.entry(w).or_default() += 1;
                    }
                }
            }
            let mut tree = PalindromicTree::new();
            for (i, &c) in s.iter().enumerate() {
                let node = tree.push(c);
                // longest palindromic suffix of s[..=i]
                let longest = (0..=i)
                    .find(|&l| s[l..=i].iter().eq(s[l..=i].iter().rev()))
                    .unwrap();
                assert_eq!(tree.len(node), i + 1 - longest);
            }
            assert_eq!(tree.distinct_palindromes(), naive.len(), "{s:?}");
            let occ = tree.occurrences();
            let mut by_len: Vec<(usize, u64)> = (2..tree.node_count())
                .map(|v| (tree.len(v), occ[v]))
                .collect();
            by_len.sort();
            let mut expected: Vec<(usize, u64)> =
                naive.iter().map(|(w, &c)| (w.len(), c)).collect();
            expected.sort();
            assert_eq!(by_len, expected, "{s:?}");
            for v in 2..tree.node_count() {
                assert!(tree.len(tree.link(v)) < tree.len(v));
            }
        }
    }
}
