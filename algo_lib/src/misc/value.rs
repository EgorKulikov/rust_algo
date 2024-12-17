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
    ($name: ident: $t: ty = $val: expr) => {
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
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
    ($name: ident: $t: ty, $val: ident) => {
        static mut $val: Option<$t> = None;

        #[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
        struct $name {}

        impl $crate::misc::value::DynamicValue<$t> for $name {
            fn set_val(t: $t) {
                unsafe {
                    $val = Some(t);
                }
            }
        }

        impl $crate::misc::value::Value<$t> for $name {
            fn val() -> $t {
                unsafe { $val.unwrap() }
            }
        }
    };
    ($name: ident: $t: ty) => {
        dynamic_value!($name: $t, VAL);
    };
    ($name: ident: $t: ty = $val: expr) => {
        dynamic_value!($name: $t);

        <$name as $crate::misc::value::DynamicValue<$t>>::set_val($val);
    };
    ($name: ident: $t: ty = $val: expr, $val_static: ident) => {
        dynamic_value!($name: $t, $val_static);

        <$name as $crate::misc::value::DynamicValue<$t>>::set_val($val);
    };
}
