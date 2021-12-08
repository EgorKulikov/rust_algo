use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Deref, Index, IndexMut, Range};
use std::vec::IntoIter;

pub enum Str<'s> {
    String(String, PhantomData<&'s str>),
    StringRef(&'s String),
    Str(&'s str),
    Vec(Vec<u8>, PhantomData<&'s str>),
    VecRef(&'s Vec<u8>),
    U8(&'s [u8]),
}

impl Str<'static> {
    pub fn new() -> Self {
        Str::Vec(Vec::new(), PhantomData::default())
    }

    pub fn with_capacity(cap: usize) -> Self {
        Str::Vec(Vec::with_capacity(cap), PhantomData::default())
    }
}

impl<'s> Str<'s> {
    pub fn push(&mut self, c: u8) {
        self.to_vec();
        self.as_vec().push(c)
    }

    pub fn as_slice(&self) -> &[u8] {
        match self {
            Str::String(s, _) => s.as_bytes(),
            Str::StringRef(s) => s.as_bytes(),
            Str::Str(s) => s.as_bytes(),
            Str::Vec(s, _) => s,
            Str::VecRef(s) => s,
            Str::U8(s) => s,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Str::String(s, _) => s.len(),
            Str::StringRef(s) => s.len(),
            Str::Str(s) => s.len(),
            Str::Vec(s, _) => s.len(),
            Str::VecRef(s) => s.len(),
            Str::U8(s) => s.len(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        match self {
            Str::String(s, _) => s.as_bytes().iter(),
            Str::StringRef(s) => s.as_bytes().iter(),
            Str::Str(s) => s.as_bytes().iter(),
            Str::Vec(s, _) => s.iter(),
            Str::VecRef(s) => s.iter(),
            Str::U8(s) => s.iter(),
        }
        .cloned()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut u8> {
        self.to_vec();
        self.as_vec().iter_mut()
    }

    pub fn sort(&mut self) {
        self.to_vec();
        self.as_vec().sort();
    }

    //noinspection RsSelfConvention
    fn to_vec(&mut self) {
        match self {
            Str::Vec(_, _) => {}
            _ => *self = self.clone(),
        }
    }

    fn as_vec(&mut self) -> &mut Vec<u8> {
        match self {
            Str::Vec(s, _) => s,
            _ => panic!("unreachable"),
        }
    }
}

impl IntoIterator for Str<'_> {
    type Item = u8;
    type IntoIter = IntoIter<u8>;

    fn into_iter(mut self) -> Self::IntoIter {
        self.to_vec();
        match self {
            Str::Vec(v, _) => v.into_iter(),
            _ => panic!("unreachable"),
        }
    }
}

impl From<String> for Str<'static> {
    fn from(s: String) -> Self {
        Str::String(s, PhantomData::default())
    }
}

impl<'s> From<&'s str> for Str<'s> {
    fn from(s: &'s str) -> Self {
        Str::Str(s)
    }
}

impl From<Vec<u8>> for Str<'static> {
    fn from(s: Vec<u8>) -> Self {
        Str::Vec(s, PhantomData::default())
    }
}

impl<'s> From<&'s [u8]> for Str<'s> {
    fn from(s: &'s [u8]) -> Self {
        Str::U8(s)
    }
}

impl<'s> From<&'s String> for Str<'s> {
    fn from(s: &'s String) -> Self {
        Str::StringRef(s)
    }
}

impl<'s> From<&'s Vec<u8>> for Str<'s> {
    fn from(s: &'s Vec<u8>) -> Self {
        Str::VecRef(s)
    }
}

impl Index<usize> for Str<'_> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Str::String(s, _) => &s.as_bytes()[index],
            Str::Str(s) => &s.as_bytes()[index],
            Str::Vec(s, _) => &s[index],
            Str::U8(s) => &s[index],
            Str::StringRef(s) => &s.as_bytes()[index],
            Str::VecRef(s) => &s[index],
        }
    }
}

impl IndexMut<usize> for Str<'_> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.to_vec();
        self.as_vec().index_mut(index)
    }
}

impl Index<Range<usize>> for Str<'_> {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        match self {
            Str::String(s, _) => &s.as_bytes()[index],
            Str::Str(s) => &s.as_bytes()[index],
            Str::Vec(s, _) => &s[index],
            Str::U8(s) => &s[index],
            Str::StringRef(s) => &s.as_bytes()[index],
            Str::VecRef(s) => &s[index],
        }
    }
}

impl IndexMut<Range<usize>> for Str<'_> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        self.to_vec();
        self.as_vec().index_mut(index)
    }
}

impl Clone for Str<'_> {
    fn clone(&self) -> Self {
        match self {
            Str::String(s, _) => Str::Vec(Vec::from(s.as_bytes()), PhantomData::default()),
            Str::Str(s) => Str::Vec(Vec::from(s.as_bytes()), PhantomData::default()),
            Str::Vec(s, _) => Str::Vec(s.clone(), PhantomData::default()),
            Str::U8(s) => Str::Vec(Vec::from(*s), PhantomData::default()),
            Str::StringRef(s) => Str::Vec(Vec::from(s.as_bytes()), PhantomData::default()),
            Str::VecRef(s) => Str::Vec(s.deref().clone(), PhantomData::default()),
        }
    }
}

impl<'r, 's, S: Into<Str<'r>>> AddAssign<S> for Str<'s> {
    fn add_assign(&mut self, rhs: S) {
        self.to_vec();
        self.as_vec().extend_from_slice(rhs.into().as_slice());
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
