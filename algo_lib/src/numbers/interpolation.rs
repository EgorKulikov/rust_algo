use crate::numbers::mod_int::mod_utils::inverse_factorials;
use crate::numbers::mod_int::BaseModInt;
use crate::numbers::num_traits::algebra::IntegerSemiRing;
use std::marker::PhantomData;

pub struct Interpolation<T, Mod: BaseModInt<T> + From<usize> + Into<usize>> {
    values: Vec<Mod>,
    coef: Vec<Mod>,
    phantom_data: PhantomData<T>,
}

impl<T: IntegerSemiRing + Copy, Mod: BaseModInt<T> + From<usize> + Into<usize>>
    Interpolation<T, Mod>
{
    pub fn new(values: Vec<Mod>) -> Self {
        let n = values.len();
        Self::with_inverse_factorials(values, inverse_factorials(n).as_slice())
    }

    pub fn with_inverse_factorials(values: Vec<Mod>, inv_fact: &[Mod]) -> Self {
        let n = values.len();
        let mut coef = Vec::with_capacity(n);
        for (i, &v) in values.iter().enumerate() {
            coef.push(
                v * inv_fact[i]
                    * inv_fact[n - i - 1]
                    * if (n - i - 1) & 1 == 1 {
                        -Mod::one()
                    } else {
                        Mod::one()
                    },
            );
        }
        Self {
            values,
            coef,
            phantom_data: PhantomData,
        }
    }

    pub fn calculate(&self, x: Mod) -> Mod {
        let i = x.into();
        if i < self.values.len() {
            return self.values[i];
        }
        let mut product = Mod::one();
        for j in 0..self.values.len() {
            product *= x - Mod::from(j);
        }
        let mut res = Mod::zero();
        for (i, &c) in self.coef.iter().enumerate() {
            res += c * (x - Mod::from(i)).inv().unwrap();
        }
        res * product
    }

    pub fn degree(&self) -> usize {
        self.values.len() - 1
    }
}
