use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::transparent_wrapper;
use std::fmt::Display;
use std::io::Write;
use std::iter::FromIterator;
use std::ops::{AddAssign, Deref, DerefMut};
use std::str::from_utf8_unchecked;
use std::vec::IntoIter;

transparent_wrapper!(Str = Vec<u8>, derive Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Default);

impl Str {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn unwrap(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for Str {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl From<&[u8]> for Str {
    fn from(v: &[u8]) -> Self {
        Self(v.to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for Str {
    fn from(v: &[u8; N]) -> Self {
        Self(v.to_vec())
    }
}

impl Readable for Str {
    fn read(input: &mut Input) -> Self {
        let mut res = Vec::new();
        input.skip_whitespace();
        while let Some(c) = input.get() {
            if c.is_ascii_whitespace() {
                break;
            }
            res.push(c);
        }
        Self(res)
    }
}

impl Writable for Str {
    fn write(&self, output: &mut Output) {
        output.write_all(&self.0).unwrap()
    }
}

impl IntoIterator for Str {
    type Item = u8;
    type IntoIter = IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Str {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Str {
    type Item = &'a mut u8;
    type IntoIter = std::slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl FromIterator<u8> for Str {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl AsRef<[u8]> for Str {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AddAssign<&[u8]> for Str {
    fn add_assign(&mut self, rhs: &[u8]) {
        self.0.extend_from_slice(rhs)
    }
}

impl Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { f.write_str(from_utf8_unchecked(&self.0)) }
    }
}

pub trait StrReader {
    fn read_str(&mut self) -> Str;
    fn read_str_vec(&mut self, n: usize) -> Vec<Str>;
    fn read_line(&mut self) -> Str;
    fn read_line_vec(&mut self, n: usize) -> Vec<Str>;
    fn read_lines(&mut self) -> Vec<Str>;
}

impl StrReader for Input<'_> {
    fn read_str(&mut self) -> Str {
        self.read()
    }

    fn read_str_vec(&mut self, n: usize) -> Vec<Str> {
        self.read_vec(n)
    }

    fn read_line(&mut self) -> Str {
        let mut res = Str::new();
        while let Some(c) = self.get() {
            if self.is_eol() {
                break;
            }
            res.push(c);
        }
        res
    }

    fn read_line_vec(&mut self, n: usize) -> Vec<Str> {
        let mut res = Vec::with_capacity(n);
        for _ in 0..n {
            res.push(self.read_line());
        }
        res
    }

    fn read_lines(&mut self) -> Vec<Str> {
        let mut res = Vec::new();
        while !self.is_exhausted() {
            res.push(self.read_line());
        }
        if let Some(s) = res.last() {
            if s.is_empty() {
                res.pop();
            }
        }
        res
    }
}
