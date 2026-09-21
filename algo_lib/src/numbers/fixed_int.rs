use crate::numbers::num_traits::algebra::{One, Zero};
use crate::numbers::num_traits::primitive::Primitive;
use std::cmp::Ordering;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

/// Fixed-width two's complement signed integer with `32 * N` bits,
/// stored as little-endian `i32` limbs. Arithmetic wraps on overflow,
/// division truncates toward zero (like the primitive integer types).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FixedInt<const N: usize>([i32; N]);

#[allow(non_camel_case_types)]
pub type i256 = FixedInt<8>;
#[allow(non_camel_case_types)]
pub type i512 = FixedInt<16>;
#[allow(non_camel_case_types)]
pub type i1024 = FixedInt<32>;

impl<const N: usize> FixedInt<N> {
    fn is_negative(&self) -> bool {
        self.0[N - 1] < 0
    }

    fn abs(self) -> Self {
        if self.is_negative() {
            -self
        } else {
            self
        }
    }

    /// Quotient and remainder of the magnitudes, treating limbs as unsigned.
    fn div_rem_mag(u: &[i32; N], v: &[i32; N]) -> ([i32; N], [i32; N]) {
        let len = |z: &[i32; N]| z.iter().rposition(|&x| x != 0).map_or(0, |p| p + 1);
        let (mut q, mut r) = ([0; N], [0; N]);
        let (m, n) = (len(u), len(v));
        assert!(n > 0, "division by zero");
        if m < n {
            return (q, *u);
        }
        if n == 1 {
            let (d, mut rem) = (v[0] as u32 as u64, 0u64);
            for i in (0..m).rev() {
                let cur = rem << 32 | u[i] as u32 as u64;
                q[i] = (cur / d) as u32 as i32;
                rem = cur % d;
            }
            r[0] = rem as u32 as i32;
            return (q, r);
        }
        // Knuth, TAOCP vol. 2, algorithm D (as in Hacker's Delight `divmnu`).
        let s = (v[n - 1] as u32).leading_zeros();
        let mut vn = [0u32; N];
        for i in (1..n).rev() {
            vn[i] = ((v[i] as u32 as u64) << s | (v[i - 1] as u32 as u64) >> (32 - s)) as u32;
        }
        vn[0] = (v[0] as u32) << s;
        let mut un = vec![0u32; m + 1];
        un[m] = ((u[m - 1] as u32 as u64) >> (32 - s)) as u32;
        for i in (1..m).rev() {
            un[i] = ((u[i] as u32 as u64) << s | (u[i - 1] as u32 as u64) >> (32 - s)) as u32;
        }
        un[0] = (u[0] as u32) << s;
        for j in (0..=m - n).rev() {
            let num = (un[j + n] as u64) << 32 | un[j + n - 1] as u64;
            let mut qhat = num / vn[n - 1] as u64;
            let mut rhat = num % vn[n - 1] as u64;
            while qhat >> 32 != 0 || qhat * vn[n - 2] as u64 > rhat << 32 | un[j + n - 2] as u64 {
                qhat -= 1;
                rhat += vn[n - 1] as u64;
                if rhat >> 32 != 0 {
                    break;
                }
            }
            let mut k = 0i64;
            for i in 0..n {
                let p = qhat * vn[i] as u64;
                let t = un[i + j] as i64 - k - (p as u32) as i64;
                un[i + j] = t as u32;
                k = (p >> 32) as i64 - (t >> 32);
            }
            let t = un[j + n] as i64 - k;
            un[j + n] = t as u32;
            q[j] = qhat as u32 as i32;
            if t < 0 {
                q[j] = q[j].wrapping_sub(1);
                let mut carry = 0u64;
                for i in 0..n {
                    carry += un[i + j] as u64 + vn[i] as u64;
                    un[i + j] = carry as u32;
                    carry >>= 32;
                }
                un[j + n] = un[j + n].wrapping_add(carry as u32);
            }
        }
        for i in 0..n {
            r[i] = ((un[i] as u64 | (un[i + 1] as u64) << 32) >> s) as u32 as i32;
        }
        (q, r)
    }

    fn div_rem(self, rhs: Self) -> (Self, Self) {
        let (q, r) = Self::div_rem_mag(&self.abs().0, &rhs.abs().0);
        let (q, r) = (Self(q), Self(r));
        (
            if self.is_negative() != rhs.is_negative() {
                -q
            } else {
                q
            },
            if self.is_negative() { -r } else { r },
        )
    }
}

/// Converts through `i128`, so the value must fit in it. Only `u128` can
/// exceed that: a `u128` of `2^127` and above wraps to a negative number, e.g.
/// `i256::from(u128::MAX)` is `-1`. Build such values from two halves instead.
impl<const N: usize, T: Primitive<i128>> From<T> for FixedInt<N> {
    fn from(v: T) -> Self {
        let mut v = v.to();
        let mut z = [0; N];
        for x in z.iter_mut() {
            *x = v as i32;
            v >>= 32;
        }
        Self(z)
    }
}

impl<const N: usize> Zero for FixedInt<N> {
    fn zero() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> One for FixedInt<N> {
    fn one() -> Self {
        Self::from(1)
    }
}

impl<const N: usize> AddAssign for FixedInt<N> {
    fn add_assign(&mut self, rhs: Self) {
        let mut carry = 0u64;
        for (a, b) in self.0.iter_mut().zip(rhs.0) {
            carry += *a as u32 as u64 + b as u32 as u64;
            *a = carry as u32 as i32;
            carry >>= 32;
        }
    }
}

impl<const N: usize> Add for FixedInt<N> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl<const N: usize> Neg for FixedInt<N> {
    type Output = Self;

    fn neg(mut self) -> Self {
        for a in self.0.iter_mut() {
            *a = !*a;
        }
        self + Self::one()
    }
}

impl<const N: usize> Sub for FixedInt<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl<const N: usize> SubAssign for FixedInt<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const N: usize> Mul for FixedInt<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut res = [0i32; N];
        for (i, &a) in self.0.iter().enumerate() {
            let (a, mut carry) = (a as u32 as u64, 0u64);
            for (r, &b) in res[i..].iter_mut().zip(rhs.0.iter()) {
                carry += a * (b as u32 as u64) + *r as u32 as u64;
                *r = carry as u32 as i32;
                carry >>= 32;
            }
        }
        Self(res)
    }
}

impl<const N: usize> MulAssign for FixedInt<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const N: usize> Div for FixedInt<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self.div_rem(rhs).0
    }
}

impl<const N: usize> DivAssign for FixedInt<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<const N: usize> Rem for FixedInt<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        self.div_rem(rhs).1
    }
}

impl<const N: usize> RemAssign for FixedInt<N> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

impl<const N: usize> Ord for FixedInt<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0[N - 1].cmp(&other.0[N - 1]).then_with(|| {
            self.0[..N - 1]
                .iter()
                .map(|&x| x as u32)
                .rev()
                .cmp(other.0[..N - 1].iter().map(|&x| x as u32).rev())
        })
    }
}

impl<const N: usize> PartialOrd for FixedInt<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
