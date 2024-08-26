#[macro_export]
macro_rules! add_assign {
    ($t: ident) => {
        impl AddAssign for $t {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
    };
}

#[macro_export]
macro_rules! mul_assign {
    ($t: ident) => {
        impl MulAssign for $t {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }
    };
}

#[macro_export]
macro_rules! sub_assign {
    ($t: ident) => {
        impl SubAssign for $t {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
    };
}

#[macro_export]
macro_rules! div_assign {
    ($t: ident) => {
        impl DivAssign for $t {
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }
    };
}
