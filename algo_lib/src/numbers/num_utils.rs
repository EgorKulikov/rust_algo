use crate::numbers::num_traits::as_index::AsIndex;
use crate::numbers::num_traits::mul_div_rem::Multable;
use crate::numbers::num_traits::zero_one::ZeroOne;

pub fn factorials<T: ZeroOne + Multable + AsIndex>(len: usize) -> Vec<T> {
    let mut res = Vec::new();
    if len > 0 {
        res.push(T::one());
    }
    while res.len() < len {
        res.push((*res.last().unwrap()) * T::from_index(res.len()));
    }
    res
}

pub fn factorial<T: ZeroOne + Multable + AsIndex>(n: usize) -> T {
    let mut res = T::one();
    for i in 1..=n {
        res *= T::from_index(i);
    }
    res
}
