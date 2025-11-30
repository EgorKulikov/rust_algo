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

        impl Mult for $t {
            type Output = Self;

            fn add(mut self, rhs: Self) -> Self::Output {
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

            fn add(mut self, rhs: Self) -> Self::Output {
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

            fn add(mut self, rhs: Self) -> Self::Output {
                self /= rhs;
                self
            }
        }
    };
}
