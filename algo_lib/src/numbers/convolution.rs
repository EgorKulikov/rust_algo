use crate::collections::iter_ext::collect::IterCollect;
use crate::collections::vec_ext::gen::VecGen;
use crate::misc::value::Value;
use crate::numbers::gcd::remainder;
use crate::numbers::mod_int::{BaseModInt, ModInt};
use crate::numbers::prime_fft::PrimeFFT;
use crate::value;

pub fn convolution<T: BaseModInt<T = i32>>(a: &[T], b: &[T]) -> Vec<T> {
    value!(Module1: i32 = 998244353);
    value!(Module2: i32 = 985661441);
    value!(Module3: i32 = 975175681);
    type Mod1 = ModInt<i32, Module1>;
    type Mod2 = ModInt<i32, Module2>;
    type Mod3 = ModInt<i32, Module3>;
    let c1 = PrimeFFT::<Mod1>::new().multiply(
        &a.iter().map(|&x| Mod1::new(x.value())).collect_vec(),
        &b.iter().map(|&x| Mod1::new(x.value())).collect_vec(),
    );
    let c2 = PrimeFFT::<Mod2>::new().multiply(
        &a.iter().map(|&x| Mod2::new(x.value())).collect_vec(),
        &b.iter().map(|&x| Mod2::new(x.value())).collect_vec(),
    );
    let c3 = PrimeFFT::<Mod3>::new().multiply(
        &a.iter().map(|&x| Mod3::new(x.value())).collect_vec(),
        &b.iter().map(|&x| Mod3::new(x.value())).collect_vec(),
    );
    let mod12 = (Module1::val() as i64 * Module2::val() as i64) as i128;
    let mod123 = mod12 * Module3::val() as i128;
    Vec::gen(c1.len(), |i, _| {
        let x1 = c1[i].value();
        let x2 = c2[i].value();
        let x3 = c3[i].value();
        let x12 = remainder(
            x1 as i64,
            Module1::val() as i64,
            x2 as i64,
            Module2::val() as i64,
        )
        .unwrap();
        let mut x123 = remainder(x12 as i128, mod12, x3 as i128, Module3::val() as i128).unwrap();
        if x123 < 0 {
            x123 += mod123;
        }
        T::from((x123 % T::module() as i128) as i32)
    })
}
