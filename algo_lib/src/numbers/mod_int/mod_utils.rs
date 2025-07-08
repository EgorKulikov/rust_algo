use crate::numbers::mod_int::{BaseModInt, ModIntF};
use crate::numbers::num_traits::algebra::IntegerSemiRing;
use crate::numbers::num_utils::{factorial, factorials};
use std::marker::PhantomData;

pub fn inverses<T: IntegerSemiRing + Copy, M: BaseModInt<T>>(len: usize) -> Vec<M> {
    let mut res = Vec::new();
    if len > 0 {
        res.push(M::zero());
    }
    if len > 1 {
        res.push(M::one());
    }
    let mut res_len = T::one() + T::one();
    while res.len() < len {
        res.push(res[M::from(M::module() % res_len).into()] * (-M::from(M::module() / res_len)));
        res_len += T::one();
    }
    res
}

pub fn inverse_factorials<T: IntegerSemiRing + Copy, M: BaseModInt<T>>(len: usize) -> Vec<M> {
    let mut res = inverses(len);
    if len > 0 {
        res[0] = M::one();
    }
    for i in 1..len {
        let last = res[i - 1];
        res[i] *= last;
    }
    res
}

pub struct Combinations<M: BaseModInt<T> + From<usize> = ModIntF, T = u32> {
    fact: Vec<M>,
    inv_fact: Vec<M>,
    phantom_data: PhantomData<T>,
}

impl<T: IntegerSemiRing + Copy, M: BaseModInt<T> + From<usize>> Combinations<M, T> {
    pub fn new(len: usize) -> Self {
        Self {
            fact: factorials(len),
            inv_fact: inverse_factorials(len),
            phantom_data: PhantomData,
        }
    }

    pub fn c(&self, n: usize, k: usize) -> M {
        if n < k {
            M::zero()
        } else {
            self.fact[n] * self.inv_fact[k] * self.inv_fact[n - k]
        }
    }

    pub fn comb_with_rep(&self, n: usize, k: usize) -> M {
        self.c(n + k - 1, k)
    }

    pub fn c_inv(&self, n: usize, k: usize) -> M {
        self.inv_fact[n] * self.fact[k] * self.fact[n - k]
    }

    pub fn fact(&self, n: usize) -> M {
        self.fact[n]
    }

    pub fn inv_fact(&self, n: usize) -> M {
        self.inv_fact[n]
    }
}

pub fn combinations<T, M: BaseModInt<T>>(n: usize, mut k: usize) -> M {
    if k > n {
        return M::zero();
    }
    if k > n - k {
        k = n - k;
    }
    let mut res = M::one();
    for i in n - k + 1..=n {
        res *= M::from(i);
    }
    res /= factorial(k);
    res
}
