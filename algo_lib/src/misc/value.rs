use std::hash::Hash;

pub trait Value<T>: Copy + Ord + Hash + Default {
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
    ($v: vis $name: ident: $t: ty = $val: expr) => {
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
        $v struct $name {}

        impl $crate::misc::value::ConstValue<$t> for $name {
            const VAL: $t = $val;
        }
    };
}

pub trait DynamicValue<T>: Value<T> {
    fn set(t: T);
}

#[macro_export]
macro_rules! dynamic_value {
    ($v: vis $name: ident: $t: ty) => {
        thread_local! {
            #[allow(non_upper_case_globals)]
            static $name: std::cell::Cell<Option<$t>> = std::cell::Cell::new(None);
        }

        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
        $v struct $name {}

        impl $crate::misc::value::DynamicValue<$t> for $name {
            fn set(t: $t) {
                $name.with(|c| c.set(Some(t)));
            }
        }

        impl $crate::misc::value::Value<$t> for $name {
            fn val() -> $t {
                $name.with(|c| c.get().unwrap())
            }
        }
    };
    ($v: vis $name: ident: $t: ty = $val: expr) => {
        dynamic_value!($v $name: $t);

        <$name as $crate::misc::value::DynamicValue<$t>>::set($val);
    };
}
