// Copyright 2015 The Rust Project Developers. See the COPYRIGHT at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::misc::lazy_lock::LazyLock;
use std::convert::TryInto;
use std::time::SystemTime;
use std::{
    collections::{HashMap, HashSet},
    hash::{BuildHasherDefault, Hasher},
    ops::BitXor,
};

pub type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub type FxHashSet<V> = HashSet<V, BuildHasherDefault<FxHasher>>;

#[derive(Default)]
pub struct FxHasher {
    hash: usize,
}

static K: LazyLock<usize> = LazyLock::new(|| {
    ((SystemTime::UNIX_EPOCH
        .elapsed()
        .unwrap()
        .as_nanos()
        .wrapping_mul(2)
        + 1)
        & 0xFFFFFFFFFFFFFFFF) as usize
});

impl FxHasher {
    #[inline]
    fn add_to_hash(&mut self, i: usize) {
        self.hash = self.hash.rotate_left(5).bitxor(i).wrapping_mul(*K);
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        let read_usize = |bytes: &[u8]| u64::from_ne_bytes(bytes[..8].try_into().unwrap());

        let mut hash = FxHasher { hash: self.hash };
        while bytes.len() >= 8 {
            hash.add_to_hash(read_usize(bytes) as usize);
            bytes = &bytes[8..];
        }
        if bytes.len() >= 4 {
            hash.add_to_hash(u32::from_ne_bytes(bytes[..4].try_into().unwrap()) as usize);
            bytes = &bytes[4..];
        }
        if bytes.len() >= 2 {
            hash.add_to_hash(u16::from_ne_bytes(bytes[..2].try_into().unwrap()) as usize);
            bytes = &bytes[2..];
        }
        if !bytes.is_empty() {
            hash.add_to_hash(bytes[0] as usize);
        }
        self.hash = hash.hash;
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add_to_hash(i as usize);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.add_to_hash(i as usize);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add_to_hash(i as usize);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add_to_hash(i as usize);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add_to_hash(i);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash as u64
    }
}
