#[macro_export]
macro_rules! add {
    ($t: ident) => {
        use std::ops::Add;

        impl Add for $t {
            type Output = Self;

            fn add(mut self, rhs: Self) -> Self::Output {
                self += rhs;
                self
            }
        }
    };
}

#[macro_export]
macro_rules! mult {
    ($t: ident) => {
        use std::ops::Mul;

        impl Mul for $t {
            type Output = Self;

            fn mul(mut self, rhs: Self) -> Self::Output {
                self *= rhs;
                self
            }
        }
    };
}

#[macro_export]
macro_rules! sub {
    ($t: ident) => {
        use std::ops::Sub;

        impl Sub for $t {
            type Output = Self;

            fn sub(mut self, rhs: Self) -> Self::Output {
                self -= rhs;
                self
            }
        }
    };
}

#[macro_export]
macro_rules! div {
    ($t: ident) => {
        use std::ops::Div;

        impl Div for $t {
            type Output = Self;

            fn div(mut self, rhs: Self) -> Self::Output {
                self /= rhs;
                self
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

    #[derive(Copy, Clone, Debug, PartialEq)]
    struct W(i64);

    impl AddAssign for W {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
        }
    }
    impl SubAssign for W {
        fn sub_assign(&mut self, rhs: Self) {
            self.0 -= rhs.0;
        }
    }
    impl MulAssign for W {
        fn mul_assign(&mut self, rhs: Self) {
            self.0 *= rhs.0;
        }
    }
    impl DivAssign for W {
        fn div_assign(&mut self, rhs: Self) {
            self.0 /= rhs.0;
        }
    }

    crate::add!(W);
    crate::sub!(W);
    crate::mult!(W);
    crate::div!(W);

    #[test]
    fn operators_from_assign_operators() {
        assert_eq!(W(7) + W(3), W(10));
        assert_eq!(W(7) - W(3), W(4));
        assert_eq!(W(7) * W(3), W(21));
        assert_eq!(W(7) / W(3), W(2));
    }
}
