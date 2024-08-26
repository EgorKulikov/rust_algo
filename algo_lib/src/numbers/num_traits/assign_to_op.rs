#[macro_export]
macro_rules! add {
    ($t: ident) => {
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
        impl Div for $t {
            type Output = Self;

            fn add(mut self, rhs: Self) -> Self::Output {
                self /= rhs;
                self
            }
        }
    };
}
