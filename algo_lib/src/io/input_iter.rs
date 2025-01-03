use crate::io::input::{Input, Readable};
use crate::string::str::Str;
use std::marker::PhantomData;

pub struct InputIterator<'t, 's: 't, T: Readable + 't + 's> {
    input: &'t mut Input<'s>,
    phantom: PhantomData<T>,
}

impl<'t, 's: 't, T: Readable + 't + 's> Iterator for InputIterator<'t, 's, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.input.skip_whitespace();
        self.input.peek().map(|_| self.input.read())
    }
}

pub trait InputIterable<'t, 's: 't> {
    fn iter<T: Readable + 't + 's>(&'t mut self) -> InputIterator<'t, 's, T>;
    fn iter_int(&'t mut self) -> InputIterator<'t, 's, i32> {
        self.iter()
    }
    fn iter_long(&'t mut self) -> InputIterator<'t, 's, i64> {
        self.iter()
    }
    fn iter_size(&'t mut self) -> InputIterator<'t, 's, usize> {
        self.iter()
    }
    fn iter_str(&'t mut self) -> InputIterator<'t, 's, Str> {
        self.iter()
    }
}

impl<'t, 's: 't> InputIterable<'t, 's> for Input<'s> {
    fn iter<T: Readable + 't + 's>(&'t mut self) -> InputIterator<'t, 's, T> {
        InputIterator {
            input: self,
            phantom: PhantomData,
        }
    }
}
