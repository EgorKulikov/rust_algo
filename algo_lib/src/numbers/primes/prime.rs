use crate::misc::random::{RandomTrait, StaticRandom};
use crate::numbers::gcd::gcd;
use crate::numbers::mod_int::ModInt64;
use crate::numbers::num_traits::algebra::{One, Zero};
use crate::numbers::num_traits::primitive::Primitive;
use crate::numbers::number_ext::Power;
use crate::{dynamic_value, when};

pub fn is_prime(n: impl Primitive<u64>) -> bool {
    let n = n.to();
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
    dynamic_value!(IsPrimeModule: u64 = n);
    type Mod = ModInt64<IsPrimeModule>;
    for _ in 0..20 {
        let a = Mod::new(StaticRandom.gen_bound(n));
        if a == Mod::zero() {
            continue;
        }
        if a.power(d) == Mod::one() {
            continue;
        }
        let mut dd = d;
        let mut good = true;
        for _ in 0..s {
            if a.power(dd) == -Mod::one() {
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

pub fn next_prime(mut n: u64) -> u64 {
    if n <= 2 {
        return 2;
    }
    n += 1 - (n & 1);
    while !is_prime(n) {
        n += 2;
    }
    n
}

fn brent(n: u64, x0: u64, c: u64) -> u64 {
    dynamic_value!(ModVal: u64 = n);
    type Mod = ModInt64<ModVal>;
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

pub fn find_divisor(n: u64) -> u64 {
    when! {
        n == 1 => 1,
        n % 2 == 0 => 2,
        is_prime(n) => n,
        else => {
            loop {
                let res = brent(
                    n,
                    StaticRandom.gen_range(2..n),
                    StaticRandom.gen_range(1..n),
                );
                if res != n {
                    return res;
                }
            }
        },
    }
}
