use crate::numbers::mod_int::BaseModInt;
use crate::numbers::num_traits::as_index::AsIndex;
use crate::numbers::num_utils::factorials;

pub fn inverses<M: BaseModInt>(len: usize) -> Vec<M>
where
    M::T: AsIndex,
{
    let mut res = Vec::new();
    if len > 0 {
        res.push(M::zero());
    }
    if len > 1 {
        res.push(M::one());
    }
    while res.len() < len {
        res.push(
            res[M::module().to_index() % res.len()]
                * (M::from(M::module() / (M::T::from_index(res.len()))).neg()),
        );
    }
    res
}

pub fn inverse_factorials<M: BaseModInt>(len: usize) -> Vec<M>
where
    M::T: AsIndex,
{
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

pub struct Combinations<M: BaseModInt>
where
    M::T: AsIndex,
{
    fact: Vec<M>,
    inv_fact: Vec<M>,
}

impl<M: BaseModInt + AsIndex> Combinations<M>
where
    M::T: AsIndex,
{
    pub fn new(len: usize) -> Self {
        Self {
            fact: factorials(len),
            inv_fact: inverse_factorials(len),
        }
    }

    pub fn c(&self, n: usize, k: usize) -> M {
        if n < k {
            M::zero()
        } else {
            self.fact[n] * self.inv_fact[k] * self.inv_fact[n - k]
        }
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
