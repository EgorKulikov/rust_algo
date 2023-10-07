use std::collections::btree_map::{IntoIter, IntoValues};
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Default, Clone, Eq, PartialEq)]
pub struct DefaultTreeMap<K: Ord + Eq, V>(BTreeMap<K, V>, V);

impl<K: Ord + Eq, V> Deref for DefaultTreeMap<K, V> {
    type Target = BTreeMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: Ord + Eq, V> DerefMut for DefaultTreeMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K: Ord + Eq, V: Default> DefaultTreeMap<K, V> {
    pub fn new() -> Self {
        Self(BTreeMap::new(), V::default())
    }

    pub fn get(&self, key: &K) -> &V {
        self.0.get(key).unwrap_or(&self.1)
    }

    pub fn get_mut(&mut self, key: K) -> &mut V {
        self.0.entry(key).or_insert_with(|| V::default())
    }

    pub fn into_values(self) -> IntoValues<K, V> {
        self.0.into_values()
    }
}

impl<K: Ord + Eq, V: Default> Index<K> for DefaultTreeMap<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.get(&index)
    }
}

impl<K: Ord + Eq, V: Default> IndexMut<K> for DefaultTreeMap<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl<K: Ord + Eq, V> IntoIterator for DefaultTreeMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K: Ord + Eq, V: Default> FromIterator<(K, V)> for DefaultTreeMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self(iter.into_iter().collect(), V::default())
    }
}
