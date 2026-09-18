//! Disjoint set union with additive potentials: `union(a, b, w)` records
//! `pot(b) - pot(a) = w`, and `diff(a, b)` returns `pot(b) - pot(a)` for
//! vertices in the same set.

use crate::numbers::num_traits::algebra::AdditionGroup;

#[derive(Clone)]
pub struct WeightedDsu<T> {
    /// Parent index, or the negated set size at a root.
    parent: Vec<i32>,
    /// Potential relative to the parent.
    to_parent: Vec<T>,
    sets: usize,
}

impl<T: AdditionGroup + Copy> WeightedDsu<T> {
    pub fn new(n: usize) -> Self {
        Self {
            parent: vec![-1; n],
            to_parent: vec![T::zero(); n],
            sets: n,
        }
    }

    pub fn len(&self) -> usize {
        self.parent.len()
    }

    pub fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }

    pub fn set_count(&self) -> usize {
        self.sets
    }

    /// Root of `x` and the potential of `x` relative to that root.
    pub fn find(&mut self, x: usize) -> (usize, T) {
        let mut v = x;
        let mut acc = T::zero();
        while self.parent[v] >= 0 {
            acc += self.to_parent[v];
            v = self.parent[v] as usize;
        }
        let root = v;
        // Compress the path, giving every vertex its potential to the root.
        let mut v = x;
        let mut rem = acc;
        while self.parent[v] >= 0 {
            let next = self.parent[v] as usize;
            let step = self.to_parent[v];
            self.parent[v] = root as i32;
            self.to_parent[v] = rem;
            rem -= step;
            v = next;
        }
        (root, acc)
    }

    pub fn size(&mut self, x: usize) -> usize {
        let (root, _) = self.find(x);
        (-self.parent[root]) as usize
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a).0 == self.find(b).0
    }

    /// `pot(b) - pot(a)` if `a` and `b` are connected.
    pub fn diff(&mut self, a: usize, b: usize) -> Option<T> {
        let (ra, pa) = self.find(a);
        let (rb, pb) = self.find(b);
        (ra == rb).then(|| pb - pa)
    }

    /// Records `pot(b) - pot(a) = w`. Returns `Ok(true)` if the sets were
    /// merged, `Ok(false)` if already connected consistently, and `Err(d)`
    /// with the existing difference if the constraint contradicts it.
    pub fn union(&mut self, a: usize, b: usize, w: T) -> Result<bool, T> {
        let (ra, pa) = self.find(a);
        let (rb, pb) = self.find(b);
        if ra == rb {
            let existing = pb - pa;
            return if existing == w {
                Ok(false)
            } else {
                Err(existing)
            };
        }
        let (mut small, mut large) = (rb, ra);
        // potential of rb relative to ra so that pot(b) - pot(a) = w
        let mut rel = pa + w - pb;
        if self.parent[small] < self.parent[large] {
            std::mem::swap(&mut small, &mut large);
            rel = -rel;
        }
        self.parent[large] += self.parent[small];
        self.parent[small] = large as i32;
        self.to_parent[small] = rel;
        self.sets -= 1;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::WeightedDsu;
    use crate::misc::random::{Random, RandomTrait};

    #[test]
    fn potentials_match_naive() {
        let mut rng = Random::new_with_seed(111);
        for n in [1usize, 2, 5, 30, 200] {
            let mut dsu = WeightedDsu::<i64>::new(n);
            let truth: Vec<i64> = (0..n).map(|_| rng.gen_range(-1000..=1000i64)).collect();
            let mut comp: Vec<usize> = (0..n).collect();
            for _ in 0..600 {
                let a = rng.gen_range(0..n);
                let b = rng.gen_range(0..n);
                match rng.gen_range(0..3u32) {
                    0 => {
                        let merged = dsu.union(a, b, truth[b] - truth[a]).unwrap();
                        assert_eq!(merged, comp[a] != comp[b]);
                        let (ca, cb) = (comp[a], comp[b]);
                        for c in comp.iter_mut() {
                            if *c == cb {
                                *c = ca;
                            }
                        }
                    }
                    1 => {
                        let expected = (comp[a] == comp[b]).then(|| truth[b] - truth[a]);
                        assert_eq!(dsu.diff(a, b), expected);
                        assert_eq!(dsu.same(a, b), comp[a] == comp[b]);
                    }
                    _ => {
                        if comp[a] == comp[b] {
                            let wrong = truth[b] - truth[a] + 1;
                            assert_eq!(dsu.union(a, b, wrong), Err(truth[b] - truth[a]));
                        }
                        let size = comp.iter().filter(|&&c| c == comp[a]).count();
                        assert_eq!(dsu.size(a), size);
                    }
                }
            }
            let distinct: std::collections::BTreeSet<usize> = comp.iter().copied().collect();
            assert_eq!(dsu.set_count(), distinct.len());
        }
    }
}
