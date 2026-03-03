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
    let mut m = Arr2d::with_gen(n, 2 * n, |i, j| {
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
    Some(Arr2d::with_gen(mat.d1(), n, |i, j| m[(i, j + n)]))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::collections::md_arr::arr2d::Arr2d;
    use crate::numbers::matrix::Matrix;
    use crate::numbers::mod_int::ModInt7;

    #[test]
    fn det_2x2() {
        let mut mat = Arr2d::with_gen(2, 2, |i, j| {
            ModInt7::from([[3usize, 8], [4, 6]][i][j])
        });
        let d = det(&mut mat);
        // det = 3*6 - 8*4 = -14
        assert_eq!(d, ModInt7::from(999_999_993usize));
    }

    #[test]
    fn det_singular() {
        let mut mat = Arr2d::with_gen(2, 2, |i, j| {
            ModInt7::from([[1usize, 2], [2, 4]][i][j])
        });
        assert_eq!(det(&mut mat), ModInt7::from(0usize));
    }

    #[test]
    fn invert_2x2() {
        let mat = Arr2d::with_gen(2, 2, |i, j| {
            ModInt7::from([[1usize, 2], [3, 4]][i][j])
        });
        let inv = invert(&mat).unwrap();
        let a = Matrix::from(mat);
        let b = Matrix::from(inv);
        assert!(a.mult(&b) == Matrix::ident(2));
    }

    #[test]
    fn invert_singular_none() {
        let mat = Arr2d::with_gen(2, 2, |i, j| {
            ModInt7::from([[1usize, 2], [2, 4]][i][j])
        });
        assert!(invert(&mat).is_none());
    }
}
