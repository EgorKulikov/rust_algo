pub trait ValueRef<T: ?Sized + 'static> {
    fn set_val(t: Box<T>);
    fn val() -> &'static T;
}

#[macro_export]
macro_rules! const_value_ref {
    ($name: ident, $val_name: ident, $int_t: ty, $ext_t: ty, $base: expr) => {
        const $val_name: $int_t = $base;

        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $name {}

        impl $crate::misc::value_ref::ValueRef<$ext_t> for $name {
            fn set_val(_: Box<$ext_t>) {
                panic!("this is const");
            }

            fn val() -> &'static $ext_t {
                &$val_name
            }
        }
    };
}

#[macro_export]
macro_rules! value_ref {
    ($name: ident, $val_name: ident, $t: ty, $base: expr) => {
        static mut $val_name: $t = $base;

        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $name {}

        impl $crate::misc::value_ref::ValueRef<$t> for $name {
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
