use crate::collections::vec_ext::sorted::Sorted;
use crate::misc::recursive_function::{Callable2, RecursiveFunction2};
use crate::numbers::num_traits::primitive::Primitive;
use crate::numbers::primes::prime::find_divisor;
use crate::numbers::primes::sieve::divisor_table;
use std::cmp::Ordering;

pub trait Factorize {
    fn prime_divisors(self) -> Vec<(u64, usize)>;
    fn divisors(self) -> Vec<u64>;
    fn max_power(self, p: Self) -> usize;
}

impl<T: Primitive<u64>> Factorize for T {
    fn prime_divisors(self) -> Vec<(u64, usize)> {
        let n = self.to();
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

    fn divisors(self) -> Vec<u64> {
        let pd = self.prime_divisors();
        let mut res = Vec::new();
        let mut rec = RecursiveFunction2::new(|f, mut d: u64, step: usize| {
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
        res.sorted()
    }

    fn max_power(self, p: Self) -> usize {
        let mut res = 0;
        let mut cur = self.to();
        assert!(cur >= 1);
        let p = p.to();
        assert!(p >= 2);
        while cur % p == 0 {
            cur /= p;
            res += 1;
        }
        res
    }
}

pub fn all_divisors(n: usize, sorted: bool) -> Vec<Vec<usize>> {
    let d: Vec<usize> = divisor_table(n);
    let mut res = Vec::with_capacity(n);
    if n > 0 {
        res.push(Vec::new());
    }
    if n > 1 {
        res.push(vec![1]);
    }
    for (i, p) in d.into_iter().enumerate().skip(2) {
        let mut q = 0;
        let mut c = i;
        while c % p == 0 {
            c /= p;
            q += 1;
        }
        let mut cur = Vec::with_capacity(res[c].len() * (q + 1));
        let mut by = 1;
        for j in 0..=q {
            cur.extend(res[c].iter().map(|&x| x * by));
            if j != q {
                by *= p;
            }
        }
        if sorted {
            cur.sort();
        }
        res.push(cur);
    }
    res
}
