use crate::collections::fx_hash_map::FxHashMap;
use std::collections::BTreeMap;
use std::hash::Hash;

pub trait EdgeMap: Clone {
    type Key: Copy;
    fn new() -> Self;
    fn get(&self, key: &Self::Key) -> Option<usize>;
    fn contains_key(&self, key: &Self::Key) -> bool;
    fn insert(&mut self, key: Self::Key, value: usize);
    fn iter(&self) -> impl Iterator<Item = (Self::Key, usize)> + '_;
}

impl<T: Ord + Copy> EdgeMap for BTreeMap<T, usize> {
    type Key = T;
    fn new() -> Self {
        BTreeMap::new()
    }
    fn get(&self, key: &T) -> Option<usize> {
        BTreeMap::get(self, key).copied()
    }
    fn contains_key(&self, key: &T) -> bool {
        BTreeMap::contains_key(self, key)
    }
    fn insert(&mut self, key: T, value: usize) {
        BTreeMap::insert(self, key, value);
    }
    fn iter(&self) -> impl Iterator<Item = (T, usize)> + '_ {
        BTreeMap::iter(self).map(|(&k, &v)| (k, v))
    }
}

impl<T: Hash + Eq + Copy> EdgeMap for FxHashMap<T, usize> {
    type Key = T;
    fn new() -> Self {
        FxHashMap::default()
    }
    fn get(&self, key: &T) -> Option<usize> {
        FxHashMap::get(self, key).copied()
    }
    fn contains_key(&self, key: &T) -> bool {
        FxHashMap::contains_key(self, key)
    }
    fn insert(&mut self, key: T, value: usize) {
        FxHashMap::insert(self, key, value);
    }
    fn iter(&self) -> impl Iterator<Item = (T, usize)> + '_ {
        FxHashMap::iter(self).map(|(&k, &v)| (k, v))
    }
}

pub type SuffixAutomatonTree<T> = SuffixAutomaton<BTreeMap<T, usize>>;
pub type SuffixAutomatonHash<T> = SuffixAutomaton<FxHashMap<T, usize>>;

pub struct SuffixAutomaton<M: EdgeMap> {
    states: Vec<State<M>>,
    last: usize,
}

struct State<M: EdgeMap> {
    len: i32,
    link: i32,
    next: M,
}

impl<M: EdgeMap> SuffixAutomaton<M> {
    pub fn new(s: &[M::Key]) -> Self {
        let mut sa = Self {
            states: Vec::with_capacity(2 * s.len()),
            last: 0,
        };
        sa.states.push(State {
            len: 0,
            link: -1,
            next: M::new(),
        });
        for &c in s {
            sa.extend(c);
        }
        sa
    }

    pub fn extend(&mut self, c: M::Key) {
        let cur = self.states.len();
        self.states.push(State {
            len: self.states[self.last].len + 1,
            link: -1,
            next: M::new(),
        });
        let mut p = self.last as i32;

        while p != -1 && !self.states[p as usize].next.contains_key(&c) {
            self.states[p as usize].next.insert(c, cur);
            p = self.states[p as usize].link;
        }

        if p == -1 {
            self.states[cur].link = 0;
        } else {
            let q = self.states[p as usize].next.get(&c).unwrap();
            if self.states[p as usize].len + 1 == self.states[q].len {
                self.states[cur].link = q as i32;
            } else {
                let clone = self.states.len();
                self.states.push(State {
                    len: self.states[p as usize].len + 1,
                    next: self.states[q].next.clone(),
                    link: self.states[q].link,
                });
                while p != -1 && self.states[p as usize].next.get(&c) == Some(q) {
                    self.states[p as usize].next.insert(c, clone);
                    p = self.states[p as usize].link;
                }
                self.states[q].link = clone as i32;
                self.states[cur].link = clone as i32;
            }
        }

        self.last = cur;
    }

    pub fn state_count(&self) -> usize {
        self.states.len()
    }

    pub fn state_len(&self, state: usize) -> i32 {
        self.states[state].len
    }

    pub fn state_link(&self, state: usize) -> Option<usize> {
        let link = self.states[state].link;
        if link == -1 {
            None
        } else {
            Some(link as usize)
        }
    }

    pub fn state_next(&self, state: usize, c: M::Key) -> Option<usize> {
        self.states[state].next.get(&c)
    }

    pub fn state_edges(&self, state: usize) -> impl Iterator<Item = (M::Key, usize)> + '_ {
        self.states[state].next.iter()
    }

    pub fn distinct_substrings(&self) -> usize {
        let mut total = 0;
        for i in 1..self.states.len() {
            total += self.num_substrings(i);
        }
        total
    }

    pub fn num_substrings(&self, state: usize) -> usize {
        (self.states[state].len - self.states[self.states[state].link as usize].len) as usize
    }

    pub fn contains(&self, pattern: &[M::Key]) -> bool {
        let mut v = 0;
        for &c in pattern {
            match self.states[v].next.get(&c) {
                Some(next) => v = next,
                None => return false,
            }
        }
        true
    }

    pub fn terminal(&self) -> Vec<bool> {
        let mut res = vec![false; self.states.len()];
        let mut v = self.last as i32;
        while v != -1 {
            res[v as usize] = true;
            v = self.states[v as usize].link;
        }
        res
    }

    pub fn end_state(&self) -> usize {
        self.last
    }
}

#[cfg(test)]
mod test {
    use super::{SuffixAutomatonHash, SuffixAutomatonTree};

    #[test]
    fn distinct_substrings_ababa() {
        let sa = SuffixAutomatonTree::new(b"ABABA");
        assert_eq!(sa.distinct_substrings(), 9);
    }

    #[test]
    fn distinct_substrings_ccccc() {
        let sa = SuffixAutomatonTree::new(b"CCCCC");
        assert_eq!(sa.distinct_substrings(), 5);
    }

    #[test]
    fn distinct_substrings_empty() {
        let sa = SuffixAutomatonTree::new(b"");
        assert_eq!(sa.distinct_substrings(), 0);
    }

    #[test]
    fn distinct_substrings_single() {
        let sa = SuffixAutomatonTree::new(b"x");
        assert_eq!(sa.distinct_substrings(), 1);
    }

    #[test]
    fn contains_found() {
        let sa = SuffixAutomatonTree::new(b"abracadabra");
        assert!(sa.contains(b"abra"));
        assert!(sa.contains(b"cad"));
        assert!(sa.contains(b"a"));
        assert!(sa.contains(b"abracadabra"));
    }

    #[test]
    fn contains_not_found() {
        let sa = SuffixAutomatonTree::new(b"abracadabra");
        assert!(!sa.contains(b"xyz"));
        assert!(!sa.contains(b"abracadabrab"));
        assert!(!sa.contains(b"dd"));
    }

    #[test]
    fn contains_empty_pattern() {
        let sa = SuffixAutomatonTree::new(b"hello");
        assert!(sa.contains(b""));
    }

    #[test]
    fn state_count_bound() {
        let s = b"abcdefghij";
        let sa = SuffixAutomatonTree::new(s);
        assert!(sa.state_count() <= 2 * s.len());
    }

    #[test]
    fn state_edges_root() {
        let sa = SuffixAutomatonTree::new(b"abc");
        let edges: Vec<_> = sa.state_edges(0).collect();
        assert_eq!(edges.len(), 3);
        assert_eq!(edges[0].0, b'a');
        assert_eq!(edges[1].0, b'b');
        assert_eq!(edges[2].0, b'c');
    }

    #[test]
    fn terminal_states() {
        let sa = SuffixAutomatonTree::new(b"abc");
        let term = sa.terminal();
        assert!(term[0]);
        let mut v = 0;
        for &c in b"abc" {
            v = sa.state_next(v, c).unwrap();
        }
        assert!(term[v]);
        let mut v = 0;
        for &c in b"bc" {
            v = sa.state_next(v, c).unwrap();
        }
        assert!(term[v]);
        let mut v = 0;
        for &c in b"ab" {
            v = sa.state_next(v, c).unwrap();
        }
        assert!(!term[v]);
    }

    #[test]
    fn distinct_substrings_abcabc() {
        let sa = SuffixAutomatonTree::new(b"abcabc");
        assert_eq!(sa.distinct_substrings(), 15);
    }

    #[test]
    fn hash_map_variant() {
        let sa = SuffixAutomatonHash::new(b"ABABA");
        assert_eq!(sa.distinct_substrings(), 9);
        assert!(sa.contains(b"ABA"));
        assert!(!sa.contains(b"CC"));
    }
}
