use crate::collections::payload::{OrdPayload, Payload};
pub struct MultiPayload<T, V = ()> {
    pub key: T,
    pub value: V,
    pub self_size: usize,
    pub total_size: usize,
}

impl<T, V> MultiPayload<T, V> {
    pub fn new(key: T, value: V) -> Self {
        Self {
            key,
            value,
            self_size: 1,
            total_size: 1,
        }
    }

    pub fn new_with_size(key: T, value: V, size: usize) -> Self {
        Self {
            key,
            value,
            self_size: size,
            total_size: size,
        }
    }
}

impl<T, V> Payload for MultiPayload<T, V> {
    const NEED_UPDATE: bool = true;

    #[inline]
    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        self.total_size =
            self.self_size + left.map_or(0, |l| l.total_size) + right.map_or(0, |r| r.total_size);
    }
}

impl<T: Ord, V: Eq> OrdPayload for MultiPayload<T, V> {
    type Key = T;

    fn key(&self) -> &Self::Key {
        &self.key
    }

    #[inline]
    fn union(a: Self, b: Self) -> Self {
        assert!(a.key == b.key);
        assert!(a.value == b.value);
        Self {
            key: a.key,
            value: a.value,
            self_size: a.self_size + b.self_size,
            total_size: a.self_size + b.self_size,
        }
    }
}
