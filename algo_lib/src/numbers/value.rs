use std::hash::Hash;

pub trait Value<T>: Copy + Eq + Hash {
    const VAL: T;
}

#[macro_export]
macro_rules! value {
    ($name: ident, $t: ty, $val: expr) => {
        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $name {}

        impl Value<$t> for $name {
            const VAL: $t = $val;
        }
    };
}

pub trait DynamicValue<T>: Copy + Eq + Hash {
    fn set_val(t: T);
    fn val() -> T;
}

#[macro_export]
macro_rules! dynamic_value {
    ($name: ident, $val_name: ident, $t: ty, $base: expr) => {
        static mut $val_name: $t = $base;

        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $name {}

        impl DynamicValue<$t> for $name {
            fn set_val(t: $t) {
                unsafe {
                    $val_name = t;
                }
            }

            fn val() -> $t {
                unsafe { $val_name }
            }
        }
    };
}

dynamic_value!(DV1, VAL1, u32, 0u32);
