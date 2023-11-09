use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Default, Clone, Eq, PartialEq)]
pub struct DefaultHashMap<K: Hash + Eq, V>(HashMap<K, V>, V);

impl<K: Hash + Eq, V> Deref for DefaultHashMap<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: Hash + Eq, V> DerefMut for DefaultHashMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K: Hash + Eq, V: Default> DefaultHashMap<K, V> {
    pub fn new() -> Self {
        Self(HashMap::new(), V::default())
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self(HashMap::with_capacity(cap), V::default())
    }

    pub fn get(&self, key: &K) -> &V {
        self.0.get(key).unwrap_or(&self.1)
    }

    pub fn get_mut(&mut self, key: K) -> &mut V {
        self.0.entry(key).or_insert_with(|| V::default())
    }

    pub fn into_values(self) -> std::collections::hash_map::IntoValues<K, V> {
        self.0.into_values()
    }
}

impl<K: Hash + Eq, V: Default> Index<K> for DefaultHashMap<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.get(&index)
    }
}

impl<K: Hash + Eq, V: Default> IndexMut<K> for DefaultHashMap<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl<K: Hash + Eq, V> IntoIterator for DefaultHashMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K: Hash + Eq, V: Default> FromIterator<(K, V)> for DefaultHashMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self(iter.into_iter().collect(), V::default())
    }
}
