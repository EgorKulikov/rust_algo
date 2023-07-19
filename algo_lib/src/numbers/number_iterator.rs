use crate::numbers::num_traits::add_sub::Addable;
use crate::numbers::num_traits::from_u8::FromU8;
use crate::numbers::num_traits::mul_div_rem::MulDiv;
use crate::numbers::num_traits::ord::MinMax;
use crate::numbers::num_traits::zero_one::ZeroOne;

pub fn iterate<T: Copy + Addable + MulDiv + ZeroOne + FromU8 + Ord + MinMax>(
    from: T,
    to: T,
) -> Vec<(T, usize, T)> {
    iterate_with_base(from, to, T::from_u8(10))
}

pub fn iterate_with_base<T: Copy + Addable + MulDiv + ZeroOne + Ord + MinMax>(
    mut from: T,
    mut to: T,
    base: T,
) -> Vec<(T, usize, T)> {
    let mut pw = T::one();
    to += T::one();
    let mut res = Vec::new();
    let mut i = 0usize;
    loop {
        let end = T::max_val() / base < pw || from / (pw * base) == to / (pw * base);
        if end {
            let c_from = from / pw;
            let c_to = to / pw;
            let mut cur = c_from;
            while cur < c_to {
                res.push((cur, i, cur * pw));
                cur += T::one();
            }
            break;
        }
        let c_from = from / pw;
        let c_to = (from / (pw * base) + T::one()) * base;
        let mut cur = c_from;
        while cur < c_to {
            res.push((cur, i, cur * pw));
            cur += T::one();
        }
        from = c_to * pw;
        let c_from = to / (pw * base) * base;
        let c_to = to / pw;
        let mut cur = c_from;
        while cur < c_to {
            res.push((cur, i, cur * pw));
            cur += T::one();
        }
        i += 1;
        pw *= base;
    }
    res.sort_by(|a, b| a.2.cmp(&b.2));
    res
}
