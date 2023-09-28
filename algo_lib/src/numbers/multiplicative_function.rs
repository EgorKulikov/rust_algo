use crate::numbers::number_ext::Power;
use crate::numbers::primes::factorize::Factorize;
use crate::numbers::primes::sieve::divisor_table;

pub struct MulitplicativeFunction(Box<dyn Fn(i64, usize, i64) -> i64>);

impl MulitplicativeFunction {
    pub fn new(f: impl Fn(i64, usize, i64) -> i64 + 'static) -> Self {
        Self(Box::new(f))
    }

    pub fn call(&self, arg: i64) -> i64 {
        let mut res = 1;
        let d = arg.prime_divisors();
        for (p, q) in d {
            res *= self.0(p, q, p.power(q));
        }
        res
    }

    pub fn calculate_up_to(&self, n: usize) -> Vec<i64> {
        let divisor_table = divisor_table::<usize>(n);
        let mut res = Vec::with_capacity(n);
        if n >= 1 {
            res.push(0);
        }
        if n <= 1 {
            return res;
        }
        res.push(1);
        for (i, d) in divisor_table.into_iter().enumerate().skip(2) {
            let mut j = i;
            let mut exp = 0;
            while j % d == 0 {
                j /= d;
                exp += 1;
            }
            res.push(res[j] * self.0(d as i64, exp, (i / j) as i64));
        }
        res
    }

    pub fn divisor_count() -> Self {
        Self::new(|_, exp, _| exp as i64 + 1)
    }

    pub fn divisor_sum() -> Self {
        Self::new(|p, _, pow| (pow * p - 1) / (p - 1))
    }

    pub fn phi() -> Self {
        Self::new(|p, _, pow| pow / p * (p - 1))
    }

    pub fn mobius() -> Self {
        Self::new(|_, exp, _| if exp == 1 { -1 } else { 0 })
    }
}
