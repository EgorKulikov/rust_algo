use crate::io::input::{Input, Readable};
use crate::string::str::{Str, StrReader};
use crate::transparent_wrapper;
use std::ops::{Deref, DerefMut};

transparent_wrapper!(EolStr = Str);
impl EolStr {
    pub fn unwrap(self) -> Str {
        self.0
    }
}

impl Readable for EolStr {
    fn read(input: &mut Input) -> Self {
        Self(input.read_line())
    }
}

transparent_wrapper!(EolVec<T> = Vec<T>);
impl<T> EolVec<T> {
    pub fn unwrap(self) -> Vec<T> {
        self.0
    }
}

impl<T: Readable> Readable for EolVec<T> {
    fn read(input: &mut Input) -> Self {
        let mut vec = Vec::new();
        loop {
            let item = T::read(input);
            if input.is_eol() {
                break;
            }
            vec.push(item);
        }
        Self(vec)
    }
}
