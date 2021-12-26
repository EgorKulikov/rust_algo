use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Default, Clone, Eq, PartialEq)]
pub struct DefaultMap<K: Hash + Eq, V>(HashMap<K, V>, V);

impl<K: Hash + Eq, V> Deref for DefaultMap<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: Hash + Eq, V> DerefMut for DefaultMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K: Hash + Eq, V: Default> DefaultMap<K, V> {
    pub fn new() -> Self {
        Self(HashMap::new(), V::default())
    }

    pub fn get(&self, key: &K) -> &V {
        self.0.get(key).unwrap_or(&self.1)
    }

    pub fn get_mut(&mut self, key: K) -> &mut V {
        self.0.entry(key).or_insert_with(|| V::default())
    }
}

impl<K: Hash + Eq, V: Default> Index<K> for DefaultMap<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.get(&index)
    }
}

impl<K: Hash + Eq, V: Default> IndexMut<K> for DefaultMap<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl<K: Hash + Eq, V> IntoIterator for DefaultMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
