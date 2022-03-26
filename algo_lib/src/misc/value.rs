use std::hash::Hash;

pub trait Value<T>: Copy + Eq + Hash {
    fn val() -> T;
}

pub trait ConstValue<T>: Value<T> {
    const VAL: T;
}

impl<T, V: ConstValue<T>> Value<T> for V {
    fn val() -> T {
        Self::VAL
    }
}

#[macro_export]
macro_rules! value {
    ($name: ident, $t: ty, $val: expr) => {
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
        pub struct $name {}

        impl $crate::misc::value::ConstValue<$t> for $name {
            const VAL: $t = $val;
        }
    };
}

pub trait DynamicValue<T>: Value<T> {
    //noinspection RsSelfConvention
    fn set_val(t: T);
}

#[macro_export]
macro_rules! dynamic_value {
    ($name: ident, $val_name: ident, $t: ty) => {
        static mut $val_name: Option<$t> = None;

        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        struct $name {}

        impl $crate::misc::value::DynamicValue<$t> for $name {
            fn set_val(t: $t) {
                unsafe {
                    $val_name = Some(t);
                }
            }
        }

        impl $crate::misc::value::Value<$t> for $name {
            fn val() -> $t {
                unsafe { $val_name.unwrap() }
            }
        }
    };
}
