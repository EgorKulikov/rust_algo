use crate::collections::fx_hash_map::FxHashMap;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref, DerefMut, Index, IndexMut};

macro_rules! default_map {
    ($name: ident, $inner: ident, $key_trait: path, $into_values: ident, $into_iter: ident) => {
        #[derive(Clone, Eq, PartialEq, Debug, Default)]
        pub struct $name<K: Eq + $key_trait, V> {
            inner: $inner<K, V>,
            default: V,
        }

        impl<K: Eq + $key_trait, V> Deref for $name<K, V> {
            type Target = $inner<K, V>;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<K: Eq + $key_trait, V> DerefMut for $name<K, V> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl<K: Eq + $key_trait, V: Clone> $name<K, V> {
            pub fn new(default: V) -> Self {
                Self {
                    inner: $inner::default(),
                    default,
                }
            }
        }

        impl<K: $key_trait + Eq, V: Clone> $name<K, V> {
            pub fn get(&self, key: &K) -> &V {
                self.inner.get(key).unwrap_or(&self.default)
            }

            pub fn get_mut(&mut self, key: K) -> &mut V {
                self.inner
                    .entry(key)
                    .or_insert_with(|| self.default.clone())
            }

            pub fn into_values(self) -> $into_values<K, V> {
                self.inner.into_values()
            }
        }

        impl<K: $key_trait + Eq, V: Clone> Index<K> for $name<K, V> {
            type Output = V;

            fn index(&self, index: K) -> &Self::Output {
                self.get(&index)
            }
        }

        impl<K: $key_trait + Eq, V: Clone> IndexMut<K> for $name<K, V> {
            fn index_mut(&mut self, index: K) -> &mut Self::Output {
                self.get_mut(index)
            }
        }

        impl<K: Eq + $key_trait, V> IntoIterator for $name<K, V> {
            type Item = (K, V);
            type IntoIter = $into_iter<K, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.inner.into_iter()
            }
        }

        impl<K: $key_trait + Eq, V: Default> FromIterator<(K, V)> for $name<K, V> {
            fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
                Self {
                    inner: $inner::from_iter(iter),
                    default: V::default(),
                }
            }
        }
    };
}

type HashMapIntoValues<K, V> = std::collections::hash_map::IntoValues<K, V>;
type HashMapIntoIter<K, V> = std::collections::hash_map::IntoIter<K, V>;

default_map!(
    DefaultHashMap,
    FxHashMap,
    Hash,
    HashMapIntoValues,
    HashMapIntoIter
);

type TreeMapIntoValues<K, V> = std::collections::btree_map::IntoValues<K, V>;
type TreeMapIntoIter<K, V> = std::collections::btree_map::IntoIter<K, V>;

default_map!(
    DefaultTreeMap,
    BTreeMap,
    Ord,
    TreeMapIntoValues,
    TreeMapIntoIter
);

pub fn qty<T: Eq + Hash + Clone>(arr: &[T]) -> DefaultHashMap<T, usize> {
    let mut map = DefaultHashMap::new(0);
    for item in arr {
        map[item.clone()] += 1;
    }
    map
}

pub fn by_index<T: Eq + Hash + Clone>(arr: &[T]) -> DefaultHashMap<T, Vec<usize>> {
    let mut map = DefaultHashMap::new(Vec::new());
    for (i, item) in arr.iter().enumerate() {
        map[item.clone()].push(i);
    }
    map
}
