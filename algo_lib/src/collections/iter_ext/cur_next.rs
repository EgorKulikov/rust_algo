use crate::zip;
use std::iter::once;

pub fn cur_next(n: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..n).zip((1..n).chain(once(0)))
}

pub fn prev_cur_next(n: usize) -> impl Iterator<Item = (usize, usize, usize)> {
    zip!(once(n - 1).chain(0..n - 1), 0..n, (1..n).chain(once(0)))
}

#[macro_export]
macro_rules! zip {
    ( @closure $p:pat => $tup:expr ) => {
        |$p| $tup
    };

    ( @closure $p:pat => ( $($tup:tt)* ) , $_iter:expr $( , $tail:expr )* ) => {
        zip!(@closure ($p, b) => ( $($tup)*, b ) $( , $tail )*)
    };

    ($first:expr $(,)*) => {
        std::iter::IntoIterator::into_iter($first)
    };

    // binary
    ($first:expr, $second:expr $(,)*) => {
        zip!($first).zip($second)
    };

    // n-ary where n > 2
    ( $first:expr $( , $rest:expr )* $(,)* ) => {
        zip!($first)
            $(
                .zip($rest)
            )*
            .map(
                zip!(@closure a => (a) $( , $rest )*)
            )
    };
}
