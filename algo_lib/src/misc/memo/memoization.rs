use crate::misc::recursive_function::Callable;
use crate::misc::recursive_function::Callable2;
use crate::misc::recursive_function::Callable3;
use crate::misc::recursive_function::Callable4;
use crate::misc::recursive_function::Callable5;
use crate::misc::recursive_function::Callable6;
use crate::misc::recursive_function::Callable7;
use crate::misc::recursive_function::Callable8;
use crate::misc::recursive_function::Callable9;
use std::collections::HashMap;
use std::hash::Hash;

macro_rules! memoization {
    ($name: ident, $trait: ident, ($($type: ident $arg: ident,)*)) => {
        pub struct $name<F, $($type, )*Output>
        where
            F: FnMut(&mut dyn $trait<$($type, )*Output>, $($type, )*) -> Output,
        {
            f: std::cell::UnsafeCell<F>,
            res: HashMap<($($type, )*), Output>,
        }

        impl<F, $($type, )*Output: Clone> $name<F, $($type, )*Output>
        where
            F: FnMut(&mut dyn $trait<$($type, )*Output>, $($type, )*) -> Output,
        {
            pub fn new(f: F) -> Self {
                Self {
                    f: std::cell::UnsafeCell::new(f),
                    res: HashMap::new(),
                }
            }
        }

        impl<F, $($type : Hash + Eq + Clone, )*Output: Clone> $trait<$($type, )*Output> for $name<F, $($type, )*Output>
        where
            F: FnMut(&mut dyn $trait<$($type, )*Output>, $($type, )*) -> Output,
        {
            fn call(&mut self, $($arg: $type, )*) -> Output {
                match self.res.get(&($($arg.clone(), )*)).cloned() {
                    None => {
                        let res = unsafe { (*self.f.get())(self, $($arg.clone(), )*) };
                        self.res.insert(($($arg, )*), res.clone());
                        res
                    }
                    Some(res) => res,
                }
            }
        }
    }
}

memoization!(Memoization, Callable, (Arg arg,));
memoization!(Memoization2, Callable2, (Arg1 arg1, Arg2 arg2,));
memoization!(Memoization3, Callable3, (Arg1 arg1, Arg2 arg2, Arg3 arg3,));
memoization!(Memoization4, Callable4, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4,));
memoization!(Memoization5, Callable5, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5,));
memoization!(Memoization6, Callable6, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5, Arg6 arg6,));
memoization!(Memoization7, Callable7, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5, Arg6 arg6, Arg7 arg7,));
memoization!(Memoization8, Callable8, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5, Arg6 arg6, Arg7 arg7, Arg8 arg8,));
memoization!(Memoization9, Callable9, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5, Arg6 arg6, Arg7 arg7, Arg8 arg8, Arg9 arg9,));
