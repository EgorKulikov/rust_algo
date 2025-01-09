use crate::collections::bit_set::BitSet;

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

pub fn primes(n: usize) -> Vec<usize> {
    primality_table(n).into_iter().collect()
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
