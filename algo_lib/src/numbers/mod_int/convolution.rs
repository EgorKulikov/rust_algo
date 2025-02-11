use crate::collections::vec_ext::gen_vec::VecGen;
use crate::misc::value::Value;
use crate::numbers::gcd::remainder;
use crate::numbers::mod_int::prime_fft::PrimeFFT;
use crate::numbers::mod_int::BaseModInt;
use crate::numbers::mod_int::ModInt;
use crate::value;

pub fn convolution(a: &[i32], b: &[i32]) -> Vec<i128> {
    value!(Module1: u32 = 998244353);
    value!(Module2: u32 = 985661441);
    value!(Module3: u32 = 975175681);
    type Mod1 = ModInt<Module1>;
    type Mod2 = ModInt<Module2>;
    type Mod3 = ModInt<Module3>;
    let c1 = PrimeFFT::<Mod1>::new().multiply(
        &a.iter()
            .map(|&x| Mod1::new_from_wide(x as i64))
            .collect::<Vec<_>>(),
        &b.iter()
            .map(|&x| Mod1::new_from_wide(x as i64))
            .collect::<Vec<_>>(),
    );
    let c2 = PrimeFFT::<Mod2>::new().multiply(
        &a.iter()
            .map(|&x| Mod2::new_from_wide(x as i64))
            .collect::<Vec<_>>(),
        &b.iter()
            .map(|&x| Mod2::new_from_wide(x as i64))
            .collect::<Vec<_>>(),
    );
    let c3 = PrimeFFT::<Mod3>::new().multiply(
        &a.iter()
            .map(|&x| Mod3::new_from_wide(x as i64))
            .collect::<Vec<_>>(),
        &b.iter()
            .map(|&x| Mod3::new_from_wide(x as i64))
            .collect::<Vec<_>>(),
    );
    let mod12 = (Module1::val() as i64 * Module2::val() as i64) as i128;
    let mod123 = mod12 * Module3::val() as i128;
    Vec::with_gen_prefix(c1.len(), |i, _| {
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
        if x123 >= mod123 / 2 {
            x123 -= mod123;
        }
        x123
    })
}
