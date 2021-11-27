pub mod collections;
pub mod io;
pub mod numbers;

#[cfg(test)]
mod tests {
    use crate::numbers::mod_int::ModInt;
    use crate::numbers::value::Value;

    #[test]
    fn it_works() {
        let val = 10007;
        let cl = || val;
        struct Val;
        impl Value<i32> for Val {
            fn value() -> i32 {
                cl()
            }
        }
        type MI = ModInt<i32, Val>;
        let res = MI::new(345345);
    }
}
