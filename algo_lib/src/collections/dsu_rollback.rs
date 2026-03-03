pub struct DSURollback {
    parent: Vec<usize>,
    rank: Vec<u32>,
    count: usize,
    history: Vec<(usize, usize, u32)>,
}

impl DSURollback {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            count: n,
            history: Vec::new(),
        }
    }

    pub fn find(&self, mut v: usize) -> usize {
        while self.parent[v] != v {
            v = self.parent[v];
        }
        v
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            return false;
        }
        if self.rank[a] < self.rank[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.history.push((a, b, self.rank[a]));
        self.parent[b] = a;
        if self.rank[a] == self.rank[b] {
            self.rank[a] += 1;
        }
        self.count -= 1;
        true
    }

    pub fn save(&self) -> usize {
        self.history.len()
    }

    pub fn rollback(&mut self, checkpoint: usize) {
        while self.history.len() > checkpoint {
            let (a, b, rank_a) = self.history.pop().unwrap();
            self.parent[b] = b;
            self.rank[a] = rank_a;
            self.count += 1;
        }
    }

    pub fn size(&self, v: usize) -> usize {
        let root = self.find(v);
        let mut sz = 0;
        for i in 0..self.parent.len() {
            if self.find(i) == root {
                sz += 1;
            }
        }
        sz
    }

    pub fn set_count(&self) -> usize {
        self.count
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.parent.len()
    }
}

#[cfg(test)]
mod test {
    use super::DSURollback;

    #[test]
    fn basic_union_find() {
        let mut dsu = DSURollback::new(5);
        assert_eq!(dsu.set_count(), 5);
        assert!(dsu.union(0, 1));
        assert!(dsu.union(2, 3));
        assert_eq!(dsu.set_count(), 3);
        assert_eq!(dsu.find(0), dsu.find(1));
        assert_ne!(dsu.find(0), dsu.find(2));
    }

    #[test]
    fn union_same_set() {
        let mut dsu = DSURollback::new(3);
        assert!(dsu.union(0, 1));
        assert!(!dsu.union(0, 1));
        assert_eq!(dsu.set_count(), 2);
    }

    #[test]
    fn rollback_restores_state() {
        let mut dsu = DSURollback::new(4);
        dsu.union(0, 1);
        let cp = dsu.save();
        dsu.union(2, 3);
        dsu.union(0, 2);
        assert_eq!(dsu.set_count(), 1);
        assert_eq!(dsu.find(0), dsu.find(3));
        dsu.rollback(cp);
        assert_eq!(dsu.set_count(), 3);
        assert_eq!(dsu.find(0), dsu.find(1));
        assert_ne!(dsu.find(0), dsu.find(2));
        assert_ne!(dsu.find(2), dsu.find(3));
    }

    #[test]
    fn rollback_to_empty() {
        let mut dsu = DSURollback::new(3);
        let cp = dsu.save();
        dsu.union(0, 1);
        dsu.union(1, 2);
        assert_eq!(dsu.set_count(), 1);
        dsu.rollback(cp);
        assert_eq!(dsu.set_count(), 3);
        assert_ne!(dsu.find(0), dsu.find(1));
        assert_ne!(dsu.find(1), dsu.find(2));
    }

    #[test]
    fn multiple_checkpoints() {
        let mut dsu = DSURollback::new(5);
        dsu.union(0, 1);
        let cp1 = dsu.save();
        dsu.union(2, 3);
        let cp2 = dsu.save();
        dsu.union(0, 4);
        assert_eq!(dsu.set_count(), 2);
        dsu.rollback(cp2);
        assert_eq!(dsu.set_count(), 3);
        dsu.rollback(cp1);
        assert_eq!(dsu.set_count(), 4);
    }

    #[test]
    fn failed_union_no_history() {
        let mut dsu = DSURollback::new(3);
        dsu.union(0, 1);
        let cp = dsu.save();
        dsu.union(0, 1); // same set, no-op
        assert_eq!(dsu.save(), cp); // history unchanged
    }
}
