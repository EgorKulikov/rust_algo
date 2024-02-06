use crate::collections::md_arr::arr2d::Arr2d;
use crate::numbers::num_traits::algebra::Field;

pub fn gauss<T: Field + Copy>(mat: &mut Arr2d<T>) {
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

pub fn det<T: Field + Copy>(mat: &mut Arr2d<T>) -> T {
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

pub fn invert<T: Field + Copy>(mat: &Arr2d<T>) -> Option<Arr2d<T>> {
    let n = mat.d2();
    assert_eq!(mat.d1(), n);
    let mut m = Arr2d::generate(n, 2 * n, |i, j| {
        if j < n {
            mat[(i, j)]
        } else if i == j - n {
            T::one()
        } else {
            T::zero()
        }
    });
    gauss(&mut m);
    for i in 0..n {
        if m[(i, i)] == T::zero() {
            return None;
        }
        assert!(m[(i, i)] == T::one());
    }
    Some(Arr2d::generate(mat.d1(), n, |i, j| m[(i, j + n)]))
}
