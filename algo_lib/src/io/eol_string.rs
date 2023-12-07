use crate::io::input::{Input, Readable};
use crate::string::str::{Str, StrReader};
use std::ops::{Deref, DerefMut};

pub struct EolString(Str<'static>);

impl Readable for EolString {
    fn read(input: &mut Input) -> Self {
        Self(input.read_line())
    }
}

impl Deref for EolString {
    type Target = Str<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EolString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
