#[macro_export]
macro_rules! add_assign {
    ($t: ident, $u: ident) => {
        impl AddAssign<$u> for $t {
            fn add_assign(&mut self, rhs: $u) {
                *self = *self + rhs;
            }
        }
    };
}

#[macro_export]
macro_rules! mul_assign {
    ($t: ident, $u: ident) => {
        impl MulAssign<$u> for $t {
            fn mul_assign(&mut self, rhs: $u) {
                *self = *self * rhs;
            }
        }
    };
}

#[macro_export]
macro_rules! sub_assign {
    ($t: ident, $u: ident) => {
        impl SubAssign<$u> for $t {
            fn sub_assign(&mut self, rhs: $u) {
                *self = *self - rhs;
            }
        }
    };
}

#[macro_export]
macro_rules! div_assign {
    ($t: ident, $u: ident) => {
        impl DivAssign<$u> for $t {
            fn div_assign(&mut self, rhs: $u) {
                *self = *self / rhs;
            }
        }
    };
}
