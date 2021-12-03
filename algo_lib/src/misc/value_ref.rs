use std::hash::Hash;

pub trait ValueRef<T: 'static>: Copy + Eq + Hash {
    fn val() -> T;
    //noinspection RsSelfConvention
    fn set_val(t: T);
}

#[macro_export]
macro_rules! value_ref {
    ($name: ident, $val_name: ident, $t: ty, $base: expr) => {
        static mut $val_name: $t = $base;

        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $name {}

        impl ValueRef<$t> for $name {
            fn set_val(t: $t) {
                unsafe {
                    $val_name = t;
                }
            }

            fn val() -> &'static $t {
                unsafe { &$val_name }
            }
        }
    };
}
