pub trait ConstValueRef<T: ?Sized + 'static> {
    fn val() -> &'static T;
}

pub trait ValueRef<T: 'static> {
    fn val() -> &'static T;
    fn set_val(t: T);
    fn val_mut() -> &'static mut T;
    fn is_init() -> bool;
}

#[macro_export]
macro_rules! const_value_ref {
    ($name: ident: $int_t: ty as $ext_t: ty = $base: expr) => {
        #[allow(non_upper_case_globals)]
        const $name: $int_t = $base;

        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $name {}

        impl $crate::misc::value_ref::ConstValueRef<$ext_t> for $name {
            fn val() -> &'static $ext_t {
                &$name
            }
        }
    };
    ($name: ident: $int_t: ty = $base: expr) => {
        const_value_ref!($name: $int_t as $int_t = $base);
    };
}

#[macro_export]
macro_rules! value_ref {
    ($name: ident: $t: ty) => {
        #[allow(non_upper_case_globals)]
        static mut $name: Option<$t> = None;

        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        struct $name {}

        impl $crate::misc::value_ref::ValueRef<$t> for $name {
            fn val() -> &'static $t {
                unsafe { $name.as_ref().unwrap() }
            }

            fn set_val(t: $t) {
                unsafe { $name = Some(t); }
            }

            fn val_mut() -> &'static mut $t {
                unsafe { $name.as_mut().unwrap() }
            }

            fn is_init() -> bool {
                unsafe { $name.is_some() }
            }
        }
    };
    ($name: ident: $t: ty = $init_val: expr) => {
        value_ref!($name: $t);
        <$name as $crate::misc::value_ref::ValueRef<$t>>::set_val($init_val);
    }
}
