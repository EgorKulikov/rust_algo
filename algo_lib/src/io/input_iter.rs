use crate::io::input::{Input, Readable};
use crate::string::str::Str;
use std::marker::PhantomData;

pub struct InputIterator<'t, T: Readable + 't> {
    input: &'t mut Input,
    phantom: PhantomData<T>,
}

impl<'t, T: Readable + 't> Iterator for InputIterator<'t, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.input.skip_whitespace();
        self.input.peek().map(|_| self.input.read())
    }
}

pub trait InputIterable<'t> {
    fn iter<T: Readable + 't>(&'t mut self) -> InputIterator<'t, T>;
    fn iter_int(&'t mut self) -> InputIterator<'t, i32> {
        self.iter()
    }
    fn iter_long(&'t mut self) -> InputIterator<'t, i64> {
        self.iter()
    }
    fn iter_size(&'t mut self) -> InputIterator<'t, usize> {
        self.iter()
    }
    fn iter_str(&'t mut self) -> InputIterator<'t, Str> {
        self.iter()
    }
}

impl<'t> InputIterable<'t> for Input {
    fn iter<T: Readable + 't>(&'t mut self) -> InputIterator<'t, T> {
        InputIterator {
            input: self,
            phantom: PhantomData,
        }
    }
}
