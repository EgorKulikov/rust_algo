pub trait ConstValueRef<T: ?Sized + 'static> {
    fn val() -> &'static T;
}

pub trait ValueRef<T: 'static> {
    fn val() -> &'static T;
    fn set_val(t: T);
    fn val_mut() -> &'static mut T;
}

#[macro_export]
macro_rules! const_value_ref {
    ($name: ident $val_name: ident: $int_t: ty as $ext_t: ty = $base: expr) => {
        const $val_name: $int_t = $base;

        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $name {}

        impl $crate::misc::value_ref::ConstValueRef<$ext_t> for $name {
            fn val() -> &'static $ext_t {
                &$val_name
            }
        }
    };
}

#[macro_export]
macro_rules! value_ref {
    ($name: ident $val_name: ident: $t: ty) => {
        static mut $val_name: Option<$t> = None;

        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        struct $name {}

        impl $crate::misc::value_ref::ValueRef<$t> for $name {
            fn val() -> &'static $t {
                unsafe { $val_name.as_ref().unwrap() }
            }

            fn set_val(t: $t) {
                unsafe {
                    $val_name = Some(t);
                }
            }

            fn val_mut() -> &'static mut $t {
                unsafe { $val_name.as_mut().unwrap() }
            }
        }
    };
}
