use crate::collections::bit_set::BitSet;
use crate::collections::iter_ext::IterExt;
use crate::dynamic_value;
use crate::misc::random::random;
use crate::misc::value::DynamicValue;
use crate::numbers::mod_int::ModInt;
use crate::numbers::num_traits::as_index::AsIndex;
use crate::numbers::num_traits::primitive::Primitive;
use crate::numbers::num_traits::zero_one::ZeroOne;
use crate::numbers::number_ext::Power;

pub fn primality_table(n: usize) -> BitSet {
    let mut res = BitSet::new(n);
    res.fill(true);
    if n > 0 {
        res.set(0, false);
    }
    if n > 1 {
        res.set(1, false);
    }
    let mut i = 2;
    while i * i < n {
        if res[i] {
            for j in ((i * i)..n).step_by(i) {
                res.set(j, false);
            }
        }
        i += 1;
    }
    res
}

pub fn primes<T: AsIndex>(n: usize) -> Vec<T> {
    let table = primality_table(n);
    table.iter().map(|i| T::from_index(i)).collect_vec()
}

pub fn divisor_table<T: AsIndex + PartialEq>(n: usize) -> Vec<T> {
    let mut res = (0..n).map(|i| T::from_index(i)).collect_vec();
    let mut i = 2;
    while i * i < n {
        if res[i] == T::from_index(i) {
            for j in ((i * i)..n).step_by(i) {
                res[j] = T::from_index(i);
            }
        }
        i += 1;
    }
    res
}

dynamic_value!(IsPrimeModule, MOD, i64);

pub fn is_prime(n: impl Primitive) -> bool {
    let n = n.into_i64();
    if n <= 1 {
        return false;
    }
    let mut s = 0;
    let mut d = n - 1;
    while d % 2 == 0 {
        s += 1;
        d >>= 1;
    }
    if s == 0 {
        return n == 2;
    }
    IsPrimeModule::set_val(n);
    type Mod = ModInt<i64, IsPrimeModule>;
    for _ in 0..20 {
        let a = Mod::new(random().next(n.into_u64()).into_i64());
        if a == Mod::zero() {
            continue;
        }
        if a.power(d) == Mod::one() {
            continue;
        }
        let mut dd = d;
        let mut good = true;
        for _ in 0..s {
            if a.power(dd) + Mod::one() == Mod::zero() {
                good = false;
                break;
            }
            dd *= 2;
        }
        if good {
            return false;
        }
    }
    true
}

pub fn next_prime(mut n: i64) -> i64 {
    if n <= 2 {
        return 2;
    }
    n += 1 - (n & 1);
    while !is_prime(n) {
        n += 2;
    }
    n
}
