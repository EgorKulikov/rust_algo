use crate::io::input::{Input, Readable};
use crate::string::str::{Str, StrReader};
use std::ops::{Deref, DerefMut};

pub struct EolStr(Str);

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

impl Deref for EolStr {
    type Target = Str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EolStr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct EolVec<T>(Vec<T>);

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

impl<T> Deref for EolVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for EolVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
