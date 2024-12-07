use crate::collections::treap::payload::{OrdPayload, Payload, Pushable};

pub struct MultiPayload<T: Unpin, V: Unpin = ()> {
    pub key: T,
    pub value: V,
    pub self_size: usize,
    pub total_size: usize,
}

impl<T: Unpin, V: Unpin> MultiPayload<T, V> {
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

impl<T: Unpin, V: Unpin> Payload for MultiPayload<T, V> {
    const NEED_UPDATE: bool = true;

    #[inline]
    fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
        self.total_size =
            self.self_size + left.map_or(0, |l| l.total_size) + right.map_or(0, |r| r.total_size);
    }
}

impl<T: Unpin, V: Unpin> Pushable<isize> for MultiPayload<T, V> {
    fn push(&mut self, delta: isize) {
        self.self_size = (self.self_size as isize + delta) as usize;
        self.total_size = (self.total_size as isize + delta) as usize;
    }
}

impl<T: Ord + Unpin, V: Eq + Unpin> OrdPayload for MultiPayload<T, V> {
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
