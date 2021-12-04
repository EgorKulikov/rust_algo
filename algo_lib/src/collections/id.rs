use std::collections::HashMap;
use std::hash::Hash;

pub struct Id<T: Hash + Eq> {
    map: HashMap<T, usize>,
    next: usize,
}

impl<T: Hash + Eq> Id<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            next: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.next
    }

    pub fn get(&mut self, el: T) -> usize {
        match self.map.get(&el) {
            None => {
                let res = self.next;
                self.map.insert(el, res);
                self.next += 1;
                res
            }
            Some(res) => *res,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &usize)> {
        self.map.iter()
    }
}

impl<T: Hash + Eq + Clone> Id<T> {
    pub fn by_id(&self) -> Vec<T> {
        let mut res = Vec::with_capacity(self.next);
        unsafe {
            res.set_len(self.next);
        }
        for (val, i) in self.map.iter() {
            res[*i] = val.clone();
        }
        res
    }
}
