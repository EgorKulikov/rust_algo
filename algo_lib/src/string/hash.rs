use crate::misc::random::random;
use crate::misc::value::DynamicValue;
use crate::misc::value_ref::ValueRef;
use crate::numbers::mod_int::ModInt;
use crate::numbers::num_traits::invertable::Invertable;
use crate::numbers::num_traits::primitive::Primitive;
use crate::numbers::num_traits::zero_one::ZeroOne;
use crate::numbers::primes::prime::next_prime;
use crate::{dynamic_value, value_ref, when};
use std::collections::Bound;
use std::ops::RangeBounds;

dynamic_value!(HM: i64);
type HashMod = ModInt<i64, HM>;

value_ref!(HashBaseContainer HBCS: HashBase);

pub struct HashBase {
    multiplier: HashMod,
    inv_multiplier: HashMod,
    power: Vec<HashMod>,
    inv_power: Vec<HashMod>,
}

impl HashBase {
    pub fn init() {
        if unsafe { HBCS.is_some() } {
            return;
        }
        HM::set_val(next_prime(
            random().next_bounds(10u64.pow(18), 2 * 10u64.pow(18)).to(),
        ));
        let multiplier = HashMod::new(random().next_bounds(257, 5 * 10u64.pow(17)).to());
        let inv_multiplier = multiplier.inv().unwrap();
        HashBaseContainer::set_val(Self {
            multiplier,
            inv_multiplier,
            power: vec![HashMod::one()],
            inv_power: vec![HashMod::one()],
        });
    }

    pub fn ensure_capacity(&mut self, n: usize) {
        if self.power.len() < n {
            self.power.reserve(n - self.power.len());
            while self.power.len() < n {
                self.power
                    .push(*self.power.last().unwrap() * self.multiplier);
            }
            self.inv_power.reserve(n - self.inv_power.len());
            while self.inv_power.len() < n {
                self.inv_power
                    .push(*self.inv_power.last().unwrap() * self.inv_multiplier);
            }
        }
    }
}

#[allow(clippy::len_without_is_empty)]
pub trait StringHash {
    fn len(&self) -> usize;
    fn hash<R: RangeBounds<usize>>(&self, r: R) -> i64;
    fn sub_hash<R: RangeBounds<usize>>(&self, r: R) -> SubstrigHash<Self> {
        SubstrigHash::new(self, r)
    }
}

pub struct SimpleHash {
    hash: Vec<HashMod>,
}

impl SimpleHash {
    pub fn new(str: &[impl Primitive<i64>]) -> Self {
        HashBaseContainer::val_mut().ensure_capacity(str.len() + 1);
        let mut hash = Vec::with_capacity(str.len() + 1);
        hash.push(HashMod::zero());
        let multiplier = HashBaseContainer::val().multiplier;
        let mut power = HashMod::one();
        for c in str {
            let c = HashMod::new(c.to());
            let cur = *hash.last().unwrap() + c * power;
            hash.push(cur);
            power *= multiplier;
        }
        Self { hash }
    }
}

impl StringHash for SimpleHash {
    fn len(&self) -> usize {
        self.hash.len() - 1
    }

    fn hash<R: RangeBounds<usize>>(&self, r: R) -> i64 {
        let (from, to) = convert_bounds(r, self.len());
        let res = (self.hash[to] - self.hash[from]) * HashBaseContainer::val().inv_power[from];
        res.val()
    }
}

pub struct SubstrigHash<'s, BaseHash: StringHash + ?Sized> {
    base: &'s BaseHash,
    from: usize,
    to: usize,
}

impl<'s, BaseHash: StringHash + ?Sized> SubstrigHash<'s, BaseHash> {
    pub fn new<R: RangeBounds<usize>>(base: &'s BaseHash, r: R) -> Self {
        let (from, to) = convert_bounds(r, base.len());
        Self { base, from, to }
    }
}

impl<'s, BaseHash: StringHash + ?Sized> StringHash for SubstrigHash<'s, BaseHash> {
    fn len(&self) -> usize {
        self.to - self.from
    }

    fn hash<R: RangeBounds<usize>>(&self, r: R) -> i64 {
        let (from, to) = convert_bounds(r, self.len());
        self.base.hash(from + self.from..to + self.from)
    }
}

fn convert_bounds<R: RangeBounds<usize>>(r: R, len: usize) -> (usize, usize) {
    let from = match r.start_bound() {
        Bound::Included(f) => *f,
        Bound::Excluded(f) => *f + 1,
        Bound::Unbounded => 0,
    };
    let to = match r.end_bound() {
        Bound::Included(t) => *t + 1,
        Bound::Excluded(t) => *t,
        Bound::Unbounded => len,
    };
    assert!(from <= to);
    assert!(to <= len);
    (from, to)
}

pub struct CompositeHash<'s, Hash1: StringHash + ?Sized, Hash2: StringHash + ?Sized> {
    base1: &'s Hash1,
    base2: &'s Hash2,
}

impl<'s, Hash1: StringHash + ?Sized, Hash2: StringHash + ?Sized> CompositeHash<'s, Hash1, Hash2> {
    pub fn new(base1: &'s Hash1, base2: &'s Hash2) -> Self {
        Self { base1, base2 }
    }
}

impl<'s, Hash1: StringHash + ?Sized, Hash2: StringHash + ?Sized> StringHash
    for CompositeHash<'s, Hash1, Hash2>
{
    fn len(&self) -> usize {
        self.base1.len() + self.base2.len()
    }

    fn hash<R: RangeBounds<usize>>(&self, r: R) -> i64 {
        let (from, to) = convert_bounds(r, self.len());
        when! {
            to <= self.base1.len() => self.base1.hash(from..to),
            from >= self.base1.len() => self.base2.hash(from - self.base1.len()..to - self.base1.len()),
            else => {
                let h1 = self.base1.hash(from..);
                let h2 = self.base2.hash(..to - self.base1.len());
                (HashMod::new(h2) * HashBaseContainer::val().power[self.base1.len() - from]
                    + HashMod::new(h1))
                .val()
            },
        }
    }
}

pub trait Hashable {
    fn str_hash(&self) -> i64;
}

impl<T: Primitive<i64>> Hashable for [T] {
    fn str_hash(&self) -> i64 {
        HashBaseContainer::val_mut().ensure_capacity(self.len() + 1);
        let mut res = HashMod::zero();
        let multiplier = HashBaseContainer::val().multiplier;
        let mut power = HashMod::one();
        for c in self {
            let c = HashMod::new(c.to());
            res += c * power;
            power *= multiplier;
        }
        res.val()
    }
}
