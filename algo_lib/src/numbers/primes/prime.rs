use crate::misc::random::random;
use crate::misc::value::DynamicValue;
use crate::numbers::gcd::gcd;
use crate::numbers::mod_int::ModInt;
use crate::numbers::num_traits::primitive::Primitive;
use crate::numbers::num_traits::zero_one::ZeroOne;
use crate::numbers::number_ext::Power;
use crate::{dynamic_value, when};

pub fn is_prime(n: impl Primitive<i64>) -> bool {
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
    dynamic_value!(IsPrimeModule: i64 = n);
    type Mod = ModInt<i64, IsPrimeModule>;
    for _ in 0..20 {
        let a = Mod::new(random().next(n as u64) as i64);
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
                    random().next_bounds(2, n as u64 - 1) as i64,
                    random().next_bounds(1, n as u64 - 1) as i64,
                );
                if res != n {
                    return res;
                }
            }
        },
    }
}
