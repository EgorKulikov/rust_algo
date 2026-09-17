use crate::collections::bit_set::BitSet;

/// Bytes per segment of the odd-only sieve (each byte is one odd number).
const SEGMENT: usize = 1 << 16;

/// Calls `f` with every prime below `n` in increasing order, using an
/// odd-only segmented sieve whose working set fits in cache.
pub fn for_each_prime(n: usize, mut f: impl FnMut(usize)) {
    if n <= 2 {
        return;
    }
    f(2);
    // Small primes up to sqrt(n) by a plain odd sieve.
    let limit = ((n as f64).sqrt() as usize + 2).min(n);
    let mut small_composite = vec![false; limit / 2 + 1];
    let mut small_primes: Vec<usize> = Vec::new();
    let mut i = 3;
    while i < limit {
        if !small_composite[i / 2] {
            small_primes.push(i);
            let mut j = i * i;
            while j < limit {
                small_composite[j / 2] = true;
                j += 2 * i;
            }
        }
        i += 2;
    }
    // Next odd multiple of each small prime to strike, as an index of odd numbers.
    let mut next: Vec<usize> = small_primes.iter().map(|&p| (p * p) / 2).collect();
    let mut segment = vec![false; SEGMENT];
    let total = n / 2; // odd numbers 1, 3, 5, ... below n have indices 0..total
    let mut start = 1; // index of 3
    while start < total {
        let end = (start + SEGMENT).min(total);
        segment[..end - start].fill(false);
        for (k, &p) in small_primes.iter().enumerate() {
            let mut j = next[k];
            if j >= end {
                continue;
            }
            while j < end {
                segment[j - start] = true;
                j += p;
            }
            next[k] = j;
        }
        for (idx, &composite) in segment[..end - start].iter().enumerate() {
            if !composite {
                f(2 * (start + idx) + 1);
            }
        }
        start = end;
    }
}

pub fn primes(n: usize) -> Vec<usize> {
    let mut res = Vec::with_capacity(if n < 100 {
        30
    } else {
        n / ((n as f64).ln() as usize - 1)
    });
    for_each_prime(n, |p| res.push(p));
    res
}

/// `true` exactly at the primes below `n`.
pub fn primality_table(n: usize) -> BitSet {
    let mut res = BitSet::new(n);
    for_each_prime(n, |p| res.set(p));
    res
}

pub fn divisor_table(n: usize) -> Vec<usize> {
    let mut res: Vec<_> = (0..n).collect();
    let mut i = 2;
    while i * i < n {
        if res[i] == i {
            for j in ((i * i)..n).step_by(i) {
                res[j] = i;
            }
        }
        i += 1;
    }
    res
}

/// Prefix sums over primes of a completely multiplicative function by the
/// Lucy_Hedgehog sieve: after construction `get(v)` returns `sum f(p)` over
/// primes `p <= v` for every `v` of the form `n / k`, in O(n^(3/4)) time.
pub struct PrimeSums<T> {
    n: u64,
    sqrt: u64,
    /// `small[v]` for `v <= sqrt`
    small: Vec<T>,
    /// `large[k]` for `v = n / k`, `k <= sqrt`
    large: Vec<T>,
}

impl<T: Copy + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>> PrimeSums<T> {
    /// `prefix(v)` must equal `sum_{i=2..=v} f(i)` and `f` must be completely
    /// multiplicative (`f(ab) = f(a) f(b)`).
    pub fn new(n: u64, prefix: impl Fn(u64) -> T, f: impl Fn(u64) -> T) -> Self {
        let sqrt = (n as f64).sqrt() as u64;
        let sqrt = if (sqrt + 1) * (sqrt + 1) <= n {
            sqrt + 1
        } else {
            sqrt
        };
        let mut small: Vec<T> = (0..=sqrt).map(|v| prefix(v.max(1))).collect();
        let mut large: Vec<T> = (0..=sqrt)
            .map(|k| prefix(n.checked_div(k).unwrap_or(1)))
            .collect();
        small[0] = prefix(1);
        let mut composite = vec![false; sqrt as usize + 1];
        for p in 2..=sqrt {
            if composite[p as usize] {
                continue;
            }
            let mut q = p * p;
            while q <= sqrt {
                composite[q as usize] = true;
                q += p;
            }
            let fp = f(p);
            let sp = small[p as usize - 1];
            let p2 = p * p;
            let mut k = 1u64;
            while k <= sqrt && n / k >= p2 {
                let v = n / k;
                let vp = v / p;
                let inner = if vp <= sqrt {
                    small[vp as usize]
                } else {
                    large[(k * p) as usize]
                };
                large[k as usize] = large[k as usize] - fp * (inner - sp);
                k += 1;
            }
            let mut v = sqrt;
            while v >= p2 {
                small[v as usize] = small[v as usize] - fp * (small[(v / p) as usize] - sp);
                v -= 1;
            }
        }
        Self {
            n,
            sqrt,
            small,
            large,
        }
    }

    /// Sum over primes up to `v`; `v` must be `n / k` for some `k >= 1`.
    pub fn get(&self, v: u64) -> T {
        if v <= self.sqrt {
            self.small[v as usize]
        } else {
            self.large[(self.n / v) as usize]
        }
    }
}

/// Number of primes up to `n`.
pub fn prime_pi(n: u64) -> u64 {
    if n < 2 {
        return 0;
    }
    PrimeSums::new(n, |v| v as i64 - 1, |_| 1).get(n) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive_primes(n: usize) -> Vec<usize> {
        (2..n)
            .filter(|&x| (2..x).take_while(|d| d * d <= x).all(|d| x % d != 0))
            .collect()
    }

    #[test]
    fn primes_match_naive() {
        for n in [
            0usize, 1, 2, 3, 4, 5, 10, 100, 1000, 65_536, 131_073, 200_000,
        ] {
            let expected = naive_primes(n);
            assert_eq!(primes(n), expected, "n={n}");
            let table = primality_table(n);
            let from_table: Vec<usize> = (0..n).filter(|&i| table[i]).collect();
            assert_eq!(from_table, expected, "table n={n}");
        }
        assert_eq!(primes(10_000_000).len(), 664_579);
    }

    #[test]
    fn prime_pi_and_sums() {
        let ps = primes(1_000_000);
        for &n in &[1u64, 2, 3, 10, 100, 1000, 999_983, 1_000_000] {
            assert_eq!(
                prime_pi(n),
                ps.iter().filter(|&&p| p as u64 <= n).count() as u64,
                "pi({n})"
            );
        }
        assert_eq!(prime_pi(10_000_000_000), 455_052_511);
        let n = 100_000u64;
        let sums = PrimeSums::new(n, |v| (v * (v + 1) / 2 - 1) as i128, |p| p as i128);
        for k in 1..=300u64 {
            let v = n / k;
            let expected: i128 = ps
                .iter()
                .map(|&p| p as u64)
                .filter(|&p| p <= v)
                .map(|p| p as i128)
                .sum();
            assert_eq!(sums.get(v), expected, "sum of primes <= {v}");
        }
    }
}
