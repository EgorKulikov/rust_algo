#[macro_export]
macro_rules! transparent_wrapper {
    ($name: ident, $t: ty) => {
        pub struct $name($t);

        impl Deref for $name {
            type Target = $t;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
