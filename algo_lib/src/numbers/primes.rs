use crate::collections::bit_set::BitSet;
use crate::collections::iter_ext::IterExt;
use crate::misc::random::random;
use crate::misc::recursive_function::{Callable2, RecursiveFunction2};
use crate::misc::value::DynamicValue;
use crate::numbers::gcd::gcd;
use crate::numbers::mod_int::ModInt;
use crate::numbers::num_traits::as_index::AsIndex;
use crate::numbers::num_traits::primitive::Primitive;
use crate::numbers::num_traits::zero_one::ZeroOne;
use crate::numbers::number_ext::Power;
use crate::{dynamic_value, when};
use std::cmp::Ordering;
use std::ops::Mul;

pub fn primality_table(n: usize) -> BitSet {
    let mut res = BitSet::new(n);
    res.fill(true);
    if n > 0 {
        res.unset(0);
    }
    if n > 1 {
        res.unset(1);
    }
    let mut i = 2;
    while i * i < n {
        if res[i] {
            for j in ((i * i)..n).step_by(i) {
                res.unset(j);
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
    dynamic_value!(IsPrimeModule: i64 = n);
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

fn brent(n: i64, x0: i64, c: i64) -> i64 {
    dynamic_value!(ModVal: i64 = n);
    type Mod = ModInt<i64, ModVal>;
    let mut x = Mod::new(x0);
    let c = Mod::new(c);
    let mut g = 1;
    let mut q = Mod::one();
    let mut xs = Mod::zero();
    let mut y = Mod::zero();
    let m = 128i64;
    let mut l = 1;
    while g == 1 {
        y = x;
        for _ in 1..l {
            x = x * x + c;
        }
        let mut k = 0;
        while k < l && g == 1 {
            xs = x;
            for _ in 0..m.min(l - k) {
                x = x * x + c;
                q *= y - x;
            }
            g = gcd(q.val(), n);
            k += m;
        }
        l *= 2;
    }
    if g == n {
        loop {
            xs = xs * xs + c;
            g = gcd((xs - y).val(), n);
            if g != 1 {
                break;
            }
        }
    }
    g
}

pub fn find_divisor(n: i64) -> i64 {
    when! {
        n == 1 => 1,
        n % 2 == 0 => 2,
        is_prime(n) => n,
        else => {
            loop {
                let res = brent(
                    n,
                    random().next_bounds(2, n.into_u64() - 1).into_i64(),
                    random().next_bounds(1, n.into_u64() - 1).into_i64(),
                );
                if res != n {
                    return res;
                }
            }
        },
    }
}

pub trait Factorize {
    fn prime_divisors(self) -> Vec<(i64, usize)>;
    fn divisors(self) -> Vec<i64>;
    fn max_power(self, p: Self) -> usize;
}

impl<T: Primitive> Factorize for T {
    fn prime_divisors(self) -> Vec<(i64, usize)> {
        let n = self.into_i64();
        assert!(n >= 1);
        if n == 1 {
            return Vec::new();
        }
        let d = if n > 100 {
            find_divisor(n)
        } else {
            let mut res = n;
            let mut i = 2;
            while i * i <= n {
                if n % i == 0 {
                    res = i;
                    break;
                }
                i += 1;
            }
            res
        };
        if d == n {
            return vec![(d, 1)];
        }
        let left = d.prime_divisors();
        let right = (n / d).prime_divisors();
        let mut res = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while i < left.len() && j < right.len() {
            match left[i].0.cmp(&right[j].0) {
                Ordering::Less => {
                    res.push(left[i]);
                    i += 1;
                }
                Ordering::Equal => {
                    res.push((left[i].0, left[i].1 + right[j].1));
                    i += 1;
                    j += 1;
                }
                Ordering::Greater => {
                    res.push(right[j]);
                    j += 1;
                }
            }
        }
        res.extend_from_slice(&left[i..]);
        res.extend_from_slice(&right[j..]);
        res
    }

    fn divisors(self) -> Vec<i64> {
        let pd = self.prime_divisors();
        let mut res = Vec::new();
        let mut rec = RecursiveFunction2::new(|f, mut d: i64, step: usize| {
            if step == pd.len() {
                res.push(d);
            } else {
                let (p, e) = pd[step];
                for i in 0..=e {
                    f.call(d, step + 1);
                    if i < e {
                        d *= p;
                    }
                }
            }
        });
        rec.call(1, 0);
        res
    }

    fn max_power(self, p: Self) -> usize {
        let mut res = 0;
        let mut cur = self.into_i64();
        assert!(cur >= 1);
        let p = p.into_i64();
        assert!(p >= 2);
        while cur % p == 0 {
            cur /= p;
            res += 1;
        }
        res
    }
}

pub fn all_divisors<T: AsIndex + PartialEq + Copy + Mul<Output = T> + Ord>(
    n: usize,
    sorted: bool,
) -> Vec<Vec<T>> {
    let d: Vec<T> = divisor_table(n);
    let mut res = Vec::with_capacity(n);
    if n > 0 {
        res.push(Vec::new());
    }
    if n > 1 {
        res.push(vec![T::from_index(1)]);
    }
    for (i, p) in d.into_iter().enumerate().skip(2) {
        let mut q = 0;
        let mut c = i;
        while c % p.to_index() == 0 {
            c /= p.to_index();
            q += 1;
        }
        let mut cur = Vec::with_capacity(res[c].len() * (q + 1));
        let mut by = T::from_index(1);
        for j in 0..=q {
            cur.extend(res[c].iter().map(|&x| x * by));
            if j != q {
                by = by * p;
            }
        }
        if sorted {
            cur.sort();
        }
        res.push(cur);
    }
    res
}
