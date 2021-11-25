use std::ops::SubAssign;
use std::fmt::Display;
use std::ops::RemAssign;
use std::ops::Mul;
use std::ops::Add;
use std::ops::IndexMut;
use std::ops::MulAssign;
use std::io::Read;
use std::ops::DivAssign;
use std::ops::Div;
use std::ops::Index;
use std::ops::Sub;
use std::io::Write;
use std::ops::AddAssign;
use std::ops::Rem;

pub struct Arr2d<T> {
    d1: usize,
    d2: usize,
    data: Vec<T>,
}

impl<T: Copy> Arr2d<T> {
    pub fn new(d1: usize, d2: usize, value: T) -> Self {
        Self {
            d1,
            d2,
            data: vec![value; d1 * d2],
        }
    }
}

impl<T> Arr2d<T> {
    pub fn generate<F>(d1: usize, d2: usize, mut gen: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut data = Vec::with_capacity(d1 * d2);
        for i in 0usize..d1 {
            for j in 0usize..d2 {
                data.push(gen(i, j));
            }
        }
        Self { d1, d2, data }
    }

    pub fn d1(&self) -> usize {
        self.d1
    }

    pub fn d2(&self) -> usize {
        self.d2
    }
}

impl<T> Index<(usize, usize)> for Arr2d<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[self.d2 * index.0 + index.1]
    }
}

impl<T> Index<usize> for Arr2d<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.d2 * index..self.d2 * (index + 1)]
    }
}

impl<T> IndexMut<(usize, usize)> for Arr2d<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.data[self.d2 * index.0 + index.1]
    }
}

impl<T> IndexMut<usize> for Arr2d<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        &mut self.data[self.d2 * index..self.d2 * (index + 1)]
    }
}

pub trait SignedInteger:
    Add<Output = Self>
    + AddAssign
    + Div<Output = Self>
    + DivAssign
    + Mul<Output = Self>
    + MulAssign
    + Rem<Output = Self>
    + RemAssign
    + Sub<Output = Self>
    + SubAssign
    + PartialEq
    + PartialOrd
    + From<i8>
    + Display
    + Clone
{
    type W: TryInto<Self> + TryFrom<Self> + Mul<Output = Self::W> + Rem<Output = Self::W>;

    fn wide_mul(lhs: Self, rhs: Self) -> Self::W;
}

macro_rules! signed_integer_impl {
    ($t: ident, $w: ident) => {
        impl SignedInteger for $t {
            type W = $w;

            fn wide_mul(lhs: Self, rhs: Self) -> Self::W {
                $w::try_from(lhs).unwrap() * $w::try_from(rhs).unwrap()
            }
        }
    };
}

signed_integer_impl!(i8, i16);
signed_integer_impl!(i16, i32);
signed_integer_impl!(i32, i64);
signed_integer_impl!(i64, i128);
signed_integer_impl!(i128, i128);
signed_integer_impl!(isize, i128);

// impl SignedInteger for i8 {
//     type W = i16;
//
//     fn wide_mul(lhs: Self, rhs: Self) -> W {
//         lhs.into() * rhs.into()
//     }
// }

// impl SignedInteger for i16 {}
// impl SignedInteger for i32 {}
// impl SignedInteger for i64 {}
// impl SignedInteger for i128 {}
// impl SignedInteger for isize {}

pub trait UnsignedInteger:
    Add<Output = Self>
    + AddAssign
    + Div<Output = Self>
    + DivAssign
    + Mul<Output = Self>
    + MulAssign
    + Rem<Output = Self>
    + RemAssign
    + Sub<Output = Self>
    + SubAssign
    + PartialEq
    + PartialOrd
    + From<u8>
    + Display
    + Clone
{
}

impl UnsignedInteger for u8 {}
impl UnsignedInteger for u16 {}
impl UnsignedInteger for u32 {}
impl UnsignedInteger for u64 {}
impl UnsignedInteger for u128 {}
impl UnsignedInteger for usize {}

pub struct Input<'s> {
    input: &'s mut dyn Read,
    buf: Vec<u8>,
    at: usize,
    buf_read: usize,
}

impl<'s> Input<'s> {
    const DEFAULT_BUF_SIZE: usize = 4096;

    pub fn new(input: &'s mut dyn Read) -> Self {
        Self {
            input,
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            buf_read: 0,
        }
    }

    pub fn new_with_size(input: &'s mut dyn Read, buf_size: usize) -> Self {
        Self {
            input,
            buf: vec![0; buf_size],
            at: 0,
            buf_read: 0,
        }
    }

    pub fn get(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            let res = self.buf[self.at];
            self.at += 1;
            Some(res)
        } else {
            None
        }
    }

    pub fn peek(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            Some(self.buf[self.at])
        } else {
            None
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(b) = self.peek() {
            if !char::from(b).is_whitespace() {
                return;
            }
            self.get();
        }
    }

    pub fn next_token(&mut self) -> Option<String> {
        self.skip_whitespace();
        let mut res = String::new();
        while let Some(c) = self.get() {
            let ch = char::from(c);
            if ch.is_whitespace() {
                break;
            }
            res.push(ch);
        }
        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }

    //noinspection RsSelfConvention
    pub fn is_exhausted(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn read<T: Readable>(&mut self) -> T {
        T::read(self)
    }

    pub fn read_vec<T: Readable>(&mut self, size: usize) -> Vec<T> {
        let mut res = Vec::with_capacity(size);
        for _ in 0usize..size {
            res.push(self.read());
        }
        res
    }

    pub fn read_table<T: Readable>(&mut self, d1: usize, d2: usize) -> Arr2d<T> {
        Arr2d::generate(d1, d2, |_, _| self.read())
    }

    pub fn read_line(&mut self) -> String {
        let mut res = String::new();
        while let Some(c) = self.get() {
            if c == b'\n' {
                break;
            }
            if c == b'\r' {
                if self.peek() == Some(b'\n') {
                    self.get();
                }
                break;
            }
            res.push(c.into());
        }
        res
    }

    fn read_signed_integer<T: SignedInteger>(&mut self) -> T {
        self.skip_whitespace();
        let mut c = self.get().unwrap();
        let sgn = if c == b'-' {
            c = self.get().unwrap();
            -1i8
        } else if c == b'+' {
            c = self.get().unwrap();
            1i8
        } else {
            1i8
        };
        let mut res: T = 0i8.into();
        loop {
            if !char::from(c).is_digit(10) {
                panic!(
                    "expected integer, found {}{}{}",
                    if sgn == 1 { "" } else { "-" },
                    res,
                    char::from(c)
                );
            }
            res *= 10i8.into();
            res += ((c - b'0') as i8).into();
            match self.get() {
                None => {
                    break;
                }
                Some(ch) => {
                    if char::from(ch).is_whitespace() {
                        break;
                    } else {
                        c = ch;
                    }
                }
            }
        }
        res *= sgn.into();
        res
    }

    fn read_unsigned_integer<T: UnsignedInteger>(&mut self) -> T {
        self.skip_whitespace();
        let mut c = self.get().unwrap();
        if c == b'+' {
            c = self.get().unwrap();
        }
        let mut res: T = 0u8.into();
        loop {
            if !char::from(c).is_digit(10) {
                panic!("expected integer, found {}{}", res, char::from(c));
            }
            res *= 10u8.into();
            res += (c - b'0').into();
            match self.get() {
                None => {
                    break;
                }
                Some(ch) => {
                    if char::from(ch).is_whitespace() {
                        break;
                    } else {
                        c = ch;
                    }
                }
            }
        }
        res
    }

    fn read_string(&mut self) -> String {
        match self.next_token() {
            None => {
                panic!("Input exhausted");
            }
            Some(res) => res,
        }
    }

    fn read_char(&mut self) -> char {
        self.skip_whitespace();
        self.get().unwrap().into()
    }

    fn read_float(&mut self) -> f64 {
        self.read_string().parse().unwrap()
    }

    fn refill_buffer(&mut self) -> bool {
        if self.at == self.buf_read {
            self.at = 0;
            self.buf_read = self.input.read(&mut self.buf).unwrap();
            self.buf_read != 0
        } else {
            true
        }
    }
}

pub trait Readable {
    fn read(input: &mut Input) -> Self;
}

impl Readable for String {
    fn read(input: &mut Input) -> Self {
        input.read_string()
    }
}

impl Readable for char {
    fn read(input: &mut Input) -> Self {
        input.read_char()
    }
}

impl Readable for f64 {
    fn read(input: &mut Input) -> Self {
        input.read_float()
    }
}

impl<T: Readable> Readable for Vec<T> {
    fn read(input: &mut Input) -> Self {
        let size = input.read();
        input.read_vec(size)
    }
}

impl<T: Readable> Readable for Arr2d<T> {
    fn read(input: &mut Input) -> Self {
        let d1 = input.read();
        let d2 = input.read();
        input.read_table(d1, d2)
    }
}

macro_rules! signed_integers {
    ($t:ident) => {
        impl Readable for $t {
            fn read(input: &mut Input) -> Self {
                input.read_signed_integer()
            }
        }
    };
}

signed_integers!(i8);
signed_integers!(i16);
signed_integers!(i32);
signed_integers!(i64);
signed_integers!(i128);
signed_integers!(isize);

macro_rules! unsigned_integers {
    ($t:ident) => {
        impl Readable for $t {
            fn read(input: &mut Input) -> Self {
                input.read_unsigned_integer()
            }
        }
    };
}

unsigned_integers!(u8);
unsigned_integers!(u16);
unsigned_integers!(u32);
unsigned_integers!(u64);
unsigned_integers!(u128);
unsigned_integers!(usize);

macro_rules! tuple_readable {
    ( $( $name:ident )+ ) => {
        impl<$($name: Readable), +> Readable for ($($name,)+) {
            fn read(input: &mut Input) -> Self {
                ($($name::read(input),)+)
            }
        }
    }
}

tuple_readable! {T}
tuple_readable! {T U}
tuple_readable! {T U V}
tuple_readable! {T U V X}
tuple_readable! {T U V X Y}
tuple_readable! {T U V X Y Z}
tuple_readable! {T U V X Y Z A}
tuple_readable! {T U V X Y Z A B}
tuple_readable! {T U V X Y Z A B C}
tuple_readable! {T U V X Y Z A B C D}
tuple_readable! {T U V X Y Z A B C D E}
tuple_readable! {T U V X Y Z A B C D E F}

pub struct Output<'a> {
    output: &'a mut dyn Write,
    buf: Vec<u8>,
    at: usize,
    autoflush: bool,
}

impl<'a> Output<'a> {
    const DEFAULT_BUF_SIZE: usize = 4096;

    pub fn new(output: &'a mut dyn Write) -> Self {
        Self {
            output,
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            autoflush: false,
        }
    }

    pub fn new_with_autoflush(output: &'a mut dyn Write) -> Self {
        Self {
            output,
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            autoflush: true,
        }
    }

    pub fn flush(&mut self) {
        if self.at != 0 {
            self.output.write(&self.buf[..self.at]).unwrap();
            self.at = 0;
        }
    }

    pub fn print(&mut self, s: &str) {
        for b in s.as_bytes() {
            self.put(*b);
        }
        if self.autoflush {
            self.flush();
        }
    }

    pub fn write<T: Display>(&mut self, s: &T) {
        self.print(format!("{}", s).as_str())
    }

    pub fn write_line<T: Display>(&mut self, s: &T) {
        self.line(format!("{}", s).as_str())
    }

    pub fn new_line(&mut self) {
        self.print("\n");
    }

    pub fn line(&mut self, s: &str) {
        self.print(s);
        self.new_line();
    }

    fn put(&mut self, b: u8) {
        self.buf[self.at] = b;
        self.at += 1;
        if self.at == self.buf.len() {
            self.flush();
        }
    }
}

fn solve(input: &mut Input, output: &mut Output, _test_case: usize) {}

fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

fn main() {
    let mut in_file = std::fs::File::open("input.txt").unwrap();
    let input = Input::new(&mut in_file);
    let mut out_file = std::fs::File::create("output.txt").unwrap();
    let output = Output::new(&mut out_file);
    run(input, output);
}
