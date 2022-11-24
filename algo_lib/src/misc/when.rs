#[macro_export]
macro_rules! when {
    {$($cond: expr => $then: expr,)*} => {
        match () {
            $(_ if $cond => $then,)*
        }
    };
    {$($cond: expr => $then: expr,)* else $(=>)? $else: expr,} => {
        match () {
            $(_ if $cond => $then,)*
            _ => $else,
        }
    };
}
