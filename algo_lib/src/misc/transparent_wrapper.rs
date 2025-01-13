#[macro_export]
macro_rules! transparent_wrapper {
    ($name: ident $(<$($par: ident$(,)?)+>)? = $t: ty $(, derive $($d: ty$(,)?)+)?) => {
        $(#[derive($($d,)+)])?
            pub struct $name$(<$($par,)+>)?($t);

        impl$(<$($par,)+>)? Deref for $name$(<$($par,)+>)? {
            type Target = $t;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl$(<$($par,)+>)? DerefMut for $name$(<$($par,)+>)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
