use crate::collections::md_arr::arr2d::Arr2d;
use crate::numbers::num_traits::add_sub::AddSub;
use crate::numbers::num_traits::mul_div_rem::MulDiv;
use crate::numbers::num_traits::zero_one::ZeroOne;

pub fn gauss<T: ZeroOne + AddSub + MulDiv + Eq>(mat: &mut Arr2d<T>) {
    let mut skip = 0;
    for i in 0..mat.d2() {
        let mut good = false;
        for j in skip..mat.d1() {
            if mat[(j, i)] != T::zero() {
                good = true;
                for k in i..mat.d2() {
                    mat.swap(j, k, skip, k);
                }
                break;
            }
        }
        if !good {
            continue;
        }
        for j in (i..mat.d2()).rev() {
            let v = mat[(skip, i)];
            mat[(skip, j)] /= v;
        }
        for j in 0..mat.d1() {
            if j == skip {
                continue;
            }
            for k in (i..mat.d2()).rev() {
                let v = mat[(j, i)] * mat[(skip, k)];
                mat[(j, k)] -= v;
            }
        }
        skip += 1;
    }
}

pub fn det<T: ZeroOne + AddSub + MulDiv + Eq + Copy>(mat: &mut Arr2d<T>) -> T {
    if mat.d1() != mat.d2() {
        return T::zero();
    }
    let mut skip = 0;
    let mut ans = T::one();
    let minus_one = T::zero() - T::one();
    for i in 0..mat.d2() {
        let mut good = false;
        for j in skip..mat.d1() {
            if mat[(j, i)] != T::zero() {
                good = true;
                if skip != j {
                    ans *= minus_one;
                    for k in i..mat.d2() {
                        mat.swap(j, k, skip, k);
                    }
                }
                break;
            }
        }
        if !good {
            return T::zero();
        }
        ans *= mat[(skip, i)];
        for j in (i..mat.d2()).rev() {
            let v = mat[(skip, i)];
            mat[(skip, j)] /= v;
        }
        for j in 0..mat.d1() {
            if j == skip {
                continue;
            }
            for k in (i..mat.d2()).rev() {
                let v = mat[(j, i)] * mat[(skip, k)];
                mat[(j, k)] -= v;
            }
        }
        skip += 1;
    }
    ans
}

pub fn invert<T: ZeroOne + AddSub + MulDiv + Eq>(mat: &Arr2d<T>) -> Option<Arr2d<T>> {
    assert_eq!(mat.d1(), mat.d2());
    let mut m = Arr2d::generate(mat.d1(), 2 * mat.d2(), |i, j| {
        if j < mat.d2() {
            mat[(i, j)]
        } else if i == j - mat.d2() {
            T::one()
        } else {
            T::zero()
        }
    });
    gauss(&mut m);
    for i in 0..mat.d2() {
        if m[(i, i)] == T::zero() {
            return None;
        }
    }
    let mut ans = Arr2d::generate(mat.d1(), mat.d2(), |i, j| m[(i, j + mat.d2())]);
    for i in 0..mat.d2() {
        let v = m[(i, i)];
        for j in 0..mat.d2() {
            ans[(i, j)] /= v;
        }
    }
    Some(ans)
}
