use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::iter::repeat;
use std::ops::{Deref, RangeBounds};

#[derive(Default)]
pub struct MultiHashSet<T> {
    map: HashMap<T, usize>,
    size: usize,
}

impl<T: Eq + Hash> MultiHashSet<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            size: 0,
        }
    }

    pub fn insert(&mut self, value: T) {
        *self.map.entry(value).or_insert(0) += 1;
        self.size += 1;
    }

    pub fn remove(&mut self, value: &T) -> bool {
        if let Some(count) = self.map.get_mut(value) {
            *count -= 1;
            self.size -= 1;
            if *count == 0 {
                self.map.remove(value);
            }
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.map
            .iter()
            .flat_map(|(value, count)| repeat(value).take(*count))
    }

    pub fn remove_all(&mut self, value: &T) -> bool {
        if let Some(count) = self.map.remove(value) {
            self.size -= count;
            true
        } else {
            false
        }
    }
}

impl<T> Deref for MultiHashSet<T> {
    type Target = HashMap<T, usize>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<T: Hash + Eq> FromIterator<T> for MultiHashSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();
        for value in iter {
            set.insert(value);
        }
        set
    }
}

#[derive(Default)]
pub struct MultiTreeSet<T> {
    map: BTreeMap<T, usize>,
    size: usize,
}

impl<T: Ord> MultiTreeSet<T> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            size: 0,
        }
    }

    pub fn insert(&mut self, value: T) {
        *self.map.entry(value).or_insert(0) += 1;
        self.size += 1;
    }

    pub fn remove(&mut self, value: &T) -> bool {
        if let Some(count) = self.map.get_mut(value) {
            *count -= 1;
            self.size -= 1;
            if *count == 0 {
                self.map.remove(value);
            }
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.map
            .iter()
            .flat_map(|(value, count)| repeat(value).take(*count))
    }

    pub fn remove_all(&mut self, value: &T) -> bool {
        if let Some(count) = self.map.remove(value) {
            self.size -= count;
            true
        } else {
            false
        }
    }

    pub fn range(&self, range: impl RangeBounds<T>) -> impl Iterator<Item = &T> {
        self.map
            .range(range)
            .flat_map(|(value, count)| repeat(value).take(*count))
    }

    pub fn range_rev(&self, range: impl RangeBounds<T>) -> impl Iterator<Item = &T> {
        self.map
            .range(range)
            .rev()
            .flat_map(|(value, count)| repeat(value).take(*count))
    }

    pub fn first(&self) -> Option<&T> {
        self.map.iter().next().map(|(value, _)| value)
    }

    pub fn last(&self) -> Option<&T> {
        self.map.iter().next_back().map(|(value, _)| value)
    }
}

impl<T: Ord + Clone> MultiTreeSet<T> {
    pub fn pop_first(&mut self) -> Option<T> {
        if let Some((value, count)) = self.map.iter_mut().next() {
            *count -= 1;
            self.size -= 1;
            let res = value.clone();
            if *count == 0 {
                self.map.remove(&res);
            }
            Some(res)
        } else {
            None
        }
    }

    pub fn pop_last(&mut self) -> Option<T> {
        if let Some((value, count)) = self.map.iter_mut().next_back() {
            *count -= 1;
            self.size -= 1;
            let res = value.clone();
            if *count == 0 {
                self.map.remove(&res);
            }
            Some(res)
        } else {
            None
        }
    }
}

impl<T> Deref for MultiTreeSet<T> {
    type Target = BTreeMap<T, usize>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<T: Ord> FromIterator<T> for MultiTreeSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();
        for value in iter {
            set.insert(value);
        }
        set
    }
}
