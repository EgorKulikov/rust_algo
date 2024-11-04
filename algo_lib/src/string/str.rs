use crate::collections::iter_ext::collect::IterCollect;
use crate::collections::slice_ext::backward::Back;
use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::{Copied, FromIterator};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Deref, DerefMut, Index, IndexMut, RangeBounds};
use std::slice::{Iter, IterMut, SliceIndex};
use std::str::FromStr;
use std::vec::IntoIter;

pub enum Str<'s> {
    Extendable(Vec<u8>, PhantomData<&'s [u8]>),
    Owned(Box<[u8]>, PhantomData<&'s [u8]>),
    Ref(&'s [u8]),
}

impl<'s> Str<'s> {
    pub fn substr(&self, range: impl RangeBounds<usize>) -> Str {
        let from = match range.start_bound() {
            std::ops::Bound::Included(&i) => i,
            std::ops::Bound::Excluded(&i) => i + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let to = match range.end_bound() {
            std::ops::Bound::Included(&i) => i + 1,
            std::ops::Bound::Excluded(&i) => i,
            std::ops::Bound::Unbounded => self.len(),
        };
        Str::from(&self[from..to])
    }
}

impl Default for Str<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl Str<'static> {
    pub fn new() -> Self {
        Str::Extendable(Vec::new(), PhantomData)
    }

    pub fn with_capacity(cap: usize) -> Self {
        Str::Extendable(Vec::with_capacity(cap), PhantomData)
    }
}

impl<'s> Str<'s> {
    pub fn push(&mut self, c: u8) {
        self.transform_to_extendable();
        self.as_extendable().push(c)
    }

    pub fn pop(&mut self) -> Option<u8> {
        self.transform_to_extendable();
        self.as_extendable().pop()
    }

    pub fn as_slice(&self) -> &[u8] {
        match self {
            Str::Extendable(s, _) => s.as_ref(),
            Str::Owned(s, _) => s.as_ref(),
            Str::Ref(s) => s,
        }
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn resize(&mut self, new_len: usize, value: u8) {
        self.transform_to_extendable();
        self.as_extendable().resize(new_len, value);
    }

    pub fn iter(&self) -> Copied<Iter<u8>> {
        match self {
            Str::Extendable(v, _) => v.iter(),
            Str::Owned(v, _) => v.iter(),
            Str::Ref(v) => v.iter(),
        }
        .copied()
    }

    pub fn iter_mut(&mut self) -> IterMut<u8> {
        self.transform_to_owned();
        self.as_mut_slice().iter_mut()
    }

    pub fn sort(&mut self) {
        self.transform_to_owned();
        self.as_mut_slice().sort_unstable();
    }

    pub fn into_owned(mut self) -> Str<'static> {
        self.transform_to_owned();
        match self {
            Str::Extendable(v, _) => Str::Extendable(v, PhantomData),
            Str::Owned(v, _) => Str::Owned(v, PhantomData),
            _ => unreachable!(),
        }
    }

    fn transform_to_extendable(&mut self) {
        match self {
            Str::Extendable(_, _) => {}
            Str::Owned(_, _) => {
                let mut fake = Str::new();
                std::mem::swap(self, &mut fake);
                if let Str::Owned(s, _) = fake {
                    *self = Str::Extendable(s.into_vec(), PhantomData)
                } else {
                    unreachable!();
                }
            }
            Str::Ref(s) => *self = Str::Extendable(s.to_vec(), PhantomData),
        }
    }

    fn as_extendable(&mut self) -> &mut Vec<u8> {
        match self {
            Str::Extendable(s, _) => s,
            _ => panic!("unreachable"),
        }
    }

    fn transform_to_owned(&mut self) {
        if let Str::Ref(s) = self {
            *self = Str::Owned(s.to_vec().into_boxed_slice(), PhantomData)
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.transform_to_owned();
        match self {
            Str::Extendable(s, _) => s.as_mut_slice(),
            Str::Owned(s, _) => s.as_mut(),
            _ => panic!("unreachable"),
        }
    }

    pub fn into_string(self) -> String {
        match self {
            Str::Extendable(v, _) => unsafe { String::from_utf8_unchecked(v) },
            Str::Owned(v, _) => unsafe { String::from_utf8_unchecked(v.into_vec()) },
            Str::Ref(v) => String::from_utf8_lossy(v).into_owned(),
        }
    }

    pub fn reverse(&mut self) {
        self.as_mut_slice().reverse();
    }

    pub fn trim(&self) -> Str<'_> {
        let mut start = 0;
        let mut end = self.len();
        while start < end && (self[start] as char).is_whitespace() {
            start += 1;
        }
        while start < end && (self[end - 1] as char).is_whitespace() {
            end -= 1;
        }
        self[start..end].into()
    }

    pub fn split<'a, 'b>(&'a self, sep: impl Into<Str<'b>>) -> Vec<Str<'a>>
    where
        's: 'a,
    {
        let sep = sep.into();
        let mut res = Vec::new();
        let mut start = 0;
        for i in 0..self.len() {
            if self[i..].starts_with(sep.as_slice()) {
                res.push(self[start..i].into());
                start = i + sep.len();
            }
        }
        res.push(self[start..].into());
        res
    }

    pub fn parse<F: FromStr>(self) -> F
    where
        F::Err: Debug,
    {
        self.into_string().parse().unwrap()
    }

    pub fn parse_vec<T: Readable>(&self) -> Vec<T> {
        let mut bytes = self.as_slice();
        let mut input = Input::new(&mut bytes);
        let mut res = Vec::new();
        while !input.is_exhausted() {
            res.push(input.read());
        }
        res
    }

    pub fn qty(&self, from: u8, to: u8) -> Vec<usize> {
        let mut res = vec![0; (to - from + 1) as usize];
        for &c in self.as_slice() {
            res[(c - from) as usize] += 1;
        }
        res
    }

    pub fn qty_lower(&self) -> Vec<usize> {
        self.qty(b'a', b'z')
    }
}

impl<'s> IntoIterator for Str<'s> {
    type Item = u8;
    type IntoIter = IntoIter<u8>;

    #[allow(clippy::unnecessary_to_owned)]
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Str::Extendable(v, _) => v.into_iter(),
            Str::Owned(v, _) => v.into_vec().into_iter(),
            Str::Ref(v) => v.to_vec().into_iter(),
        }
    }
}

impl From<String> for Str<'static> {
    fn from(s: String) -> Self {
        Str::Extendable(s.into(), PhantomData)
    }
}

impl<'s> From<&'s str> for Str<'s> {
    fn from(s: &'s str) -> Self {
        Str::Ref(s.as_bytes())
    }
}

impl From<Vec<u8>> for Str<'static> {
    fn from(s: Vec<u8>) -> Self {
        Str::Extendable(s, PhantomData)
    }
}

impl<'s> From<&'s [u8]> for Str<'s> {
    fn from(s: &'s [u8]) -> Self {
        Str::Ref(s)
    }
}

impl<'s, const N: usize> From<&'s [u8; N]> for Str<'s> {
    fn from(s: &'s [u8; N]) -> Self {
        Str::Ref(s)
    }
}

impl<'s> From<&'s String> for Str<'s> {
    fn from(s: &'s String) -> Self {
        Str::Ref(s.as_bytes())
    }
}

impl<'s> From<&'s Vec<u8>> for Str<'s> {
    fn from(s: &'s Vec<u8>) -> Self {
        Str::Ref(s.as_slice())
    }
}

impl From<u8> for Str<'static> {
    fn from(c: u8) -> Self {
        Str::Owned(Box::new([c]), PhantomData)
    }
}

impl From<char> for Str<'static> {
    fn from(c: char) -> Self {
        Str::from(c as u8)
    }
}

impl<'s, 't: 's> From<&'s Str<'t>> for Str<'s> {
    fn from(value: &'s Str<'t>) -> Self {
        Str::Ref(value.as_slice())
    }
}

impl<R: SliceIndex<[u8]>> Index<R> for Str<'_> {
    type Output = R::Output;

    fn index(&self, index: R) -> &Self::Output {
        self.as_slice().index(index)
    }
}

impl<R: SliceIndex<[u8]>> IndexMut<R> for Str<'_> {
    fn index_mut(&mut self, index: R) -> &mut Self::Output {
        self.transform_to_owned();
        self.as_mut_slice().index_mut(index)
    }
}

impl Clone for Str<'_> {
    fn clone(&self) -> Self {
        match self {
            Str::Extendable(s, _) => s.clone().into(),
            Str::Owned(s, _) => s.to_vec().into(),
            Str::Ref(s) => Str::Ref(s),
        }
    }
}

impl<'r, 's, S: Into<Str<'r>>> AddAssign<S> for Str<'s> {
    fn add_assign(&mut self, rhs: S) {
        self.transform_to_extendable();
        self.as_extendable()
            .extend_from_slice(rhs.into().as_slice());
    }
}

impl<'r, 's, S: Into<Str<'r>>> Add<S> for Str<'s> {
    type Output = Str<'s>;

    fn add(mut self, rhs: S) -> Self::Output {
        self += rhs;
        self
    }
}

impl Readable for Str<'static> {
    fn read(input: &mut Input) -> Self {
        input.next_token().unwrap().into()
    }
}

impl Writable for Str<'_> {
    fn write(&self, output: &mut Output) {
        for c in self.as_slice() {
            output.put(*c);
        }
        output.maybe_flush();
    }
}

impl Display for Str<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <String as Display>::fmt(&String::from_utf8(self.as_slice().to_vec()).unwrap(), f)
    }
}

impl Hash for Str<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<'r> PartialEq<Str<'r>> for Str<'_> {
    fn eq(&self, other: &Str<'r>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl Eq for Str<'_> {}

impl<'r> PartialOrd<Str<'r>> for Str<'_> {
    fn partial_cmp(&self, other: &Str<'r>) -> Option<Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl Ord for Str<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl FromIterator<u8> for Str<'static> {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self::Extendable(iter.into_iter().collect_vec(), Default::default())
    }
}

impl<'r> FromIterator<&'r u8> for Str<'static> {
    fn from_iter<T: IntoIterator<Item = &'r u8>>(iter: T) -> Self {
        Self::Extendable(iter.into_iter().cloned().collect_vec(), Default::default())
    }
}

impl Deref for Str<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for Str<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

pub trait StrReader {
    fn read_str(&mut self) -> Str<'static>;
    fn read_str_vec(&mut self, n: usize) -> Vec<Str<'static>>;
    fn read_line(&mut self) -> Str<'static>;
    fn read_line_vec(&mut self, n: usize) -> Vec<Str<'static>>;
    fn read_lines(&mut self) -> Vec<Str<'static>>;
}

impl StrReader for Input<'_> {
    fn read_str(&mut self) -> Str<'static> {
        self.read()
    }

    fn read_str_vec(&mut self, n: usize) -> Vec<Str<'static>> {
        self.read_vec(n)
    }

    fn read_line(&mut self) -> Str<'static> {
        let mut res = Str::new();
        while let Some(c) = self.get() {
            if c == b'\n' {
                break;
            }
            res.push(c);
        }
        res
    }

    fn read_line_vec(&mut self, n: usize) -> Vec<Str<'static>> {
        let mut res = Vec::with_capacity(n);
        for _ in 0..n {
            res.push(self.read_line());
        }
        res
    }

    fn read_lines(&mut self) -> Vec<Str<'static>> {
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

impl Index<Back> for Str<'_> {
    type Output = u8;

    fn index(&self, index: Back) -> &Self::Output {
        &self[self.len() - index.0 - 1]
    }
}

impl IndexMut<Back> for Str<'_> {
    fn index_mut(&mut self, index: Back) -> &mut Self::Output {
        let len = self.len();
        &mut self[len - index.0 - 1]
    }
}

impl AsRef<[u8]> for Str<'_> {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}
