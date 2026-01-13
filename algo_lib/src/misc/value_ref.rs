pub trait ConstValueRef<T: ?Sized + 'static> {
    fn val() -> &'static T;
}

pub trait ValueRef<T: 'static> {
    fn with<R, F: FnOnce(&T) -> R>(f: F) -> R;
    fn set(t: T);
    fn with_mut<R, F: FnOnce(&mut T) -> R>(f: F) -> R;
    fn is_init() -> bool;
}

#[macro_export]
macro_rules! const_value_ref {
    ($v: vis $name: ident: $int_t: ty as $ext_t: ty = $base: expr) => {
        #[allow(non_upper_case_globals)]
        const $name: $int_t = $base;

        #[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
        $v struct $name {}

        impl $crate::misc::value_ref::ConstValueRef<$ext_t> for $name {
            fn val() -> &'static $ext_t {
                &$name
            }
        }
    };
    ($v: vis $name: ident: $int_t: ty = $base: expr) => {
        const_value_ref!($v $name: $int_t as $int_t = $base);
    };
}

#[macro_export]
macro_rules! value_ref {
    ($v: vis $name: ident: $t: ty) => {
        thread_local! {
            #[allow(non_upper_case_globals)]
            static $name: std::cell::RefCell<Option<$t>> = std::cell::RefCell::new(None);
        }

        #[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
        $v struct $name {}

        impl $crate::misc::value_ref::ValueRef<$t> for $name {
            fn with<R, F: FnOnce(&$t) -> R>(f: F) -> R {
                $name.with(|cell| f(cell.borrow().as_ref().unwrap()))
            }

            fn set(t: $t) {
                $name.with(|cell| *cell.borrow_mut() = Some(t));
            }

            fn with_mut<R, F: FnOnce(&mut $t) -> R>(f: F) -> R {
                $name.with(|cell| f(cell.borrow_mut().as_mut().unwrap()))
            }

            fn is_init() -> bool {
                $name.with(|cell| cell.borrow().is_some())
            }
        }
    };
    ($v: vis $name: ident: $t: ty = $init_val: expr) => {
        value_ref!($v $name: $t);
        <$name as $crate::misc::value_ref::ValueRef<$t>>::set($init_val);
    };
}
