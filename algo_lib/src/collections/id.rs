use crate::collections::fx_hash_map::FxHashMap;
use std::hash::Hash;
use std::mem::MaybeUninit;
use std::ops::Deref;

#[derive(Default, Clone)]
pub struct Id<T> {
    map: FxHashMap<T, usize>,
    next: usize,
}

impl<T: Hash + Eq> Id<T> {
    pub fn new() -> Self {
        Self {
            map: FxHashMap::default(),
            next: 0,
        }
    }

    pub fn advance(&mut self, by: usize) {
        self.next += by;
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

    pub fn add(&mut self, arr: impl IntoIterator<Item = T>) {
        for el in arr {
            self.get(el);
        }
    }

    pub fn add_pairs(&mut self, arr: impl IntoIterator<Item = (T, T)>) {
        for (a, b) in arr {
            self.get(a);
            self.get(b);
        }
    }
}

impl<T> Deref for Id<T> {
    type Target = FxHashMap<T, usize>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<T: Hash + Eq + Clone> Id<T> {
    pub fn by_id(&self) -> Vec<T> {
        unsafe {
            let mut res = MaybeUninit::new(Vec::with_capacity(self.len()));
            (*res.as_mut_ptr()).set_len(self.len());
            for (val, i) in self.map.iter() {
                let ptr: *mut T = (*res.as_mut_ptr()).as_mut_ptr();
                ptr.add(*i).write(val.clone());
            }
            res.assume_init()
        }
    }
}
