use crate::io::input::{Input, Readable};
use crate::string::str::{Str, StrReader};

pub struct EolString(pub Str<'static>);

impl Readable for EolString {
    fn read(input: &mut Input) -> Self {
        Self(input.read_line())
    }
}

pub struct EolVec<T>(pub Vec<T>);

impl<T: Readable> Readable for EolVec<T> {
    fn read(input: &mut Input) -> Self {
        let mut vec = Vec::new();
        let s = input.read_line();
        for mut t in s.as_slice().split(|c| *c == b' ') {
            let mut input = Input::new(&mut t);
            vec.push(T::read(&mut input));
            assert!(input.is_exhausted());
        }
        Self(vec)
    }
}
