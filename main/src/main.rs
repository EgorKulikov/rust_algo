use std::ops::Div;
use std::ops::MulAssign;
use std::slice::Iter;
use std::ops::RemAssign;
use std::ops::SubAssign;
use std::ops::Index;
use std::io::Write;
use std::ops::IndexMut;
use std::marker::PhantomData;
use std::slice::IterMut;
use std::iter::Skip;
use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::Sub;
use std::io::Read;
use std::ops::Add;
use std::ops::Rem;
use std::hash::Hash;
use std::ops::Mul;
use std::ops::AddAssign;
use std::iter::StepBy;
use std::ops::DivAssign;


pub trait WeakInteger:
    Add<Output = Self>
    + AddAssign
    + Mul<Output = Self>
    + MulAssign
    + Sub<Output = Self>
    + SubAssign
    + PartialEq
    + Copy
    + Eq
    + Hash
{
    type W: From<Self> + WeakInteger;

    fn zero() -> Self;
    fn one() -> Self;
    fn from_u8(n: u8) -> Self;
    fn downcast(w: Self::W) -> Self;

    fn two() -> Self {
        Self::one() + Self::one()
    }

    fn power<T: Integer>(&self, exp: T) -> Self {
        if exp == T::zero() {
            Self::one()
        } else if exp % T::two() == T::zero() {
            let res = self.power(exp / T::two());
            res * res
        } else {
            self.power(exp - T::one()) * (*self)
        }
    }
}

pub trait Integer:
    WeakInteger + Ord + Div<Output = Self> + DivAssign + Rem<Output = Self> + RemAssign + 'static
{
    type W: From<Self> + Integer;

    const SIGNED: bool;

    fn max() -> Self;
    fn min() -> Self;
    fn downcast(w: <Self as Integer>::W) -> Self;
}

macro_rules! integer_impl {
    ($t: ident, $w: ident, $s: expr) => {
        impl WeakInteger for $t {
            type W = $w;

            fn zero() -> Self {
                0
            }

            fn one() -> Self {
                1
            }

            fn from_u8(n: u8) -> Self {
                n as $t
            }

            fn downcast(w: Self::W) -> Self {
                w as $t
            }
        }

        impl Integer for $t {
            type W = $w;

            const SIGNED: bool = $s;

            fn max() -> Self {
                $t::MAX
            }

            fn min() -> Self {
                $t::MIN
            }

            fn downcast(w: <Self as Integer>::W) -> Self {
                w as $t
            }
        }
    };
}

integer_impl!(i128, i128, true);
integer_impl!(i64, i128, true);
integer_impl!(i32, i64, true);
integer_impl!(i16, i32, true);
integer_impl!(i8, i16, true);
integer_impl!(isize, isize, true);
integer_impl!(u128, u128, false);
integer_impl!(u64, u128, false);
integer_impl!(u32, u64, false);
integer_impl!(u16, u32, false);
integer_impl!(u8, u16, false);
integer_impl!(usize, usize, false);

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

    fn read_integer<T: Integer + Display>(&mut self) -> T {
        self.skip_whitespace();
        let mut c = self.get().unwrap();
        let sgn = if c == b'-' {
            if !T::SIGNED {
                panic!("negative integer")
            }
            c = self.get().unwrap();
            true
        } else if c == b'+' {
            c = self.get().unwrap();
            false
        } else {
            false
        };
        let mut res = T::zero();
        loop {
            if !char::from(c).is_digit(10) {
                panic!(
                    "expected integer, found {}{}{}",
                    if sgn { "" } else { "-" },
                    res,
                    char::from(c)
                );
            }
            res *= T::from_u8(10);
            res += T::from_u8(c - b'0');
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
        if sgn {
            debug_assert!(T::SIGNED);
            res = T::zero() - res
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

impl<T: Readable, const N: usize> Readable for [T; N] {
    fn read(input: &mut Input) -> Self {
        let mut arr: [T; N] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            arr[i] = T::read(input);
        }
        arr
    }
}

macro_rules! read_integer {
    ($t:ident) => {
        impl Readable for $t {
            fn read(input: &mut Input) -> Self {
                input.read_integer()
            }
        }
    };
}

read_integer!(i8);
read_integer!(i16);
read_integer!(i32);
read_integer!(i64);
read_integer!(i128);
read_integer!(isize);
read_integer!(u8);
read_integer!(u16);
read_integer!(u32);
read_integer!(u64);
read_integer!(u128);
read_integer!(usize);

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

pub struct Output {
    output: Box<dyn Write>,
    buf: Vec<u8>,
    at: usize,
    autoflush: bool,
}

impl Output {
    const DEFAULT_BUF_SIZE: usize = 4096;

    pub fn new(output: Box<dyn Write>) -> Self {
        Self {
            output,
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            autoflush: false,
        }
    }

    pub fn new_with_autoflush(output: Box<dyn Write>) -> Self {
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

    pub fn print<T: Writable>(&mut self, s: &T) {
        s.write(self);
    }

    pub fn put(&mut self, b: u8) {
        self.buf[self.at] = b;
        self.at += 1;
        if self.at == self.buf.len() {
            self.flush();
        }
    }

    pub fn print_per_line<T: Writable>(&mut self, arg: &[T]) {
        for i in arg {
            i.write(self);
            self.put(b'\n');
        }
    }

    pub fn print_iter<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
        let mut first = true;
        for e in iter {
            if first {
                first = false;
            } else {
                self.put(b' ');
            }
            e.write(self);
        }
    }

    pub fn print_iter_ref<'a, T: 'a + Writable, I: Iterator<Item = &'a T>>(&mut self, iter: I) {
        let mut first = true;
        for e in iter {
            if first {
                first = false;
            } else {
                self.put(b' ');
            }
            e.write(self);
        }
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut start = 0usize;
        let mut rem = buf.len();
        while rem > 0 {
            let len = (self.buf.len() - self.at).min(rem);
            self.buf[self.at..self.at + len].copy_from_slice(&buf[start..start + len]);
            self.at += len;
            if self.at == self.buf.len() {
                self.flush();
            }
            start += len;
            rem -= len;
        }
        if self.autoflush {
            self.flush();
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.flush();
        Ok(())
    }
}

pub trait Writable {
    fn write(&self, output: &mut Output);
}

impl Writable for &str {
    fn write(&self, output: &mut Output) {
        output.write(&self.as_bytes()).unwrap();
    }
}

impl Writable for String {
    fn write(&self, output: &mut Output) {
        output.write(&self.as_bytes()).unwrap();
    }
}

impl Writable for char {
    fn write(&self, output: &mut Output) {
        output.put(*self as u8);
    }
}

impl<T: Writable> Writable for [T] {
    fn write(&self, output: &mut Output) {
        output.print_iter_ref(self.iter());
    }
}

impl<T: Writable> Writable for Vec<T> {
    fn write(&self, output: &mut Output) {
        self[..].write(output);
    }
}

macro_rules! write_to_string {
    ($t:ident) => {
        impl Writable for $t {
            fn write(&self, output: &mut Output) {
                self.to_string().write(output);
            }
        }
    };
}

write_to_string!(u8);
write_to_string!(u16);
write_to_string!(u32);
write_to_string!(u64);
write_to_string!(u128);
write_to_string!(usize);
write_to_string!(i8);
write_to_string!(i16);
write_to_string!(i32);
write_to_string!(i64);
write_to_string!(i128);
write_to_string!(isize);

impl<T: Writable, U: Writable> Writable for (T, U) {
    fn write(&self, output: &mut Output) {
        self.0.write(output);
        output.put(b' ');
        self.1.write(output);
    }
}

impl<T: Writable, U: Writable, V: Writable> Writable for (T, U, V) {
    fn write(&self, output: &mut Output) {
        self.0.write(output);
        output.put(b' ');
        self.1.write(output);
        output.put(b' ');
        self.2.write(output);
    }
}

pub static mut OUTPUT: Option<Output> = None;

pub fn output() -> &'static mut Output {
    unsafe {
        match &mut OUTPUT {
            None => {
                panic!("Panic");
            }
            Some(output) => output,
        }
    }
}

#[macro_export]
macro_rules! out {
    ($first: expr $(,$args:expr )*) => {
        output().print(&$first);
        $(output().put(b' ');
        output().print(&$args);
        )*
    }
}

#[macro_export]
macro_rules! out_line {
    ($first: expr $(, $args:expr )* ) => {
        out!($first $(,$args)*);
        output().put(b'\n');
    }
}

pub struct Arr2d<T> {
    d1: usize,
    d2: usize,
    data: Vec<T>,
}

impl<T: Clone> Arr2d<T> {
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

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    pub fn column(&self, col: usize) -> StepBy<Skip<Iter<T>>> {
        assert!(col < self.d2);
        self.data.iter().skip(col).step_by(self.d2)
    }

    pub fn column_mut(&mut self, col: usize) -> StepBy<Skip<IterMut<T>>> {
        assert!(col < self.d2);
        self.data.iter_mut().skip(col).step_by(self.d2)
    }
}

impl<T> Index<(usize, usize)> for Arr2d<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[self.d2 * row + col]
    }
}

impl<T> Index<usize> for Arr2d<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.d2 * index..self.d2 * (index + 1)]
    }
}

impl<T> IndexMut<(usize, usize)> for Arr2d<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        &mut self.data[self.d2 * row + col]
    }
}

impl<T> IndexMut<usize> for Arr2d<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        &mut self.data[self.d2 * index..self.d2 * (index + 1)]
    }
}

impl<T: Writable> Writable for Arr2d<T> {
    fn write(&self, output: &mut Output) {
        let mut at = 0usize;
        for i in 0usize..self.d1 {
            if i != 0 {
                output.put(b'\n');
            }
            for j in 0usize..self.d2 {
                if j != 0 {
                    output.put(b' ');
                }
                self.data[at].write(output);
                at += 1;
            }
        }
    }
}

pub trait Arr2dRead {
    fn read_table<T: Readable>(&mut self, d1: usize, d2: usize) -> Arr2d<T>;
}

impl Arr2dRead for Input<'_> {
    fn read_table<T: Readable>(&mut self, d1: usize, d2: usize) -> Arr2d<T> {
        Arr2d::generate(d1, d2, |_, _| self.read())
    }
}

impl<T: Readable> Readable for Arr2d<T> {
    fn read(input: &mut Input) -> Self {
        let d1 = input.read();
        let d2 = input.read();
        input.read_table(d1, d2)
    }
}

pub trait Value<T>: Copy + Eq + Hash {
    fn val() -> T;
}

pub trait ConstValue<T>: Value<T> {
    const VAL: T;
}

impl<T, V: ConstValue<T>> Value<T> for V {
    fn val() -> T {
        Self::VAL
    }
}

#[macro_export]
macro_rules! value {
    ($name: ident, $t: ty, $val: expr) => {
        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $name {}

        impl ConstValue<$t> for $name {
            const VAL: $t = $val;
        }
    };
}

pub trait DynamicValue<T>: Value<T> {
    //noinspection RsSelfConvention
    fn set_val(t: T);
}

#[macro_export]
macro_rules! dynamic_value {
    ($name: ident, $val_name: ident, $t: ty, $base: expr) => {
        static mut $val_name: $t = $base;

        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $name {}

        impl DynamicValue<$t> for $name {
            fn set_val(t: $t) {
                unsafe {
                    $val_name = t;
                }
            }
        }

        impl Value<$t> for $name {
            fn val() -> $t {
                unsafe { $val_name }
            }
        }
    };
}

pub struct Directions<V: Value<[(isize, isize); N]>, const N: usize> {
    phantom: PhantomData<V>,
}

impl<V: Value<[(isize, isize); N]>, const N: usize> Directions<V, N> {
    pub fn iter(row: usize, col: usize, n: usize, m: usize) -> DirectionsIter<V, N> {
        DirectionsIter {
            row,
            col,
            n,
            m,
            at: 0,
            phantom: Default::default(),
        }
    }
}

pub struct DirectionsIter<V: Value<[(isize, isize); N]>, const N: usize> {
    row: usize,
    col: usize,
    n: usize,
    m: usize,
    at: usize,
    phantom: PhantomData<V>,
}

impl<V: Value<[(isize, isize); N]>, const N: usize> Iterator for DirectionsIter<V, N> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.at < N {
            let nrow = (self.row as isize) + V::val()[self.at].0;
            let ncol = (self.col as isize) + V::val()[self.at].1;
            self.at += 1;
            if nrow >= 0 && (nrow as usize) < self.n && ncol >= 0 && (ncol as usize) < self.m {
                return Some((nrow as usize, ncol as usize));
            }
        }
        None
    }
}

value!(
    D4Dirs,
    [(isize, isize); 4],
    [
        (1isize, 0isize),
        (0isize, 1isize),
        (-1isize, 0isize),
        (0isize, -1isize)
    ]
);

pub type D4 = Directions<D4Dirs, 4>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let mut grid = input.read_table::<char>(n, m);

    let mut degree = Arr2d::new(n, m, 0u8);
    let mut li = n;
    let mut lj = m;
    for i in 0..n {
        for j in 0..m {
            if grid[(i, j)] == '#' {
                continue;
            }
            if grid[(i, j)] == 'L' {
                li = i;
                lj = j;
            }
            if i > 0 && grid[(i - 1, j)] != '#' {
                degree[(i - 1, j)] += 1;
                degree[(i, j)] += 1;
            }
            if j > 0 && grid[(i, j - 1)] != '#' {
                degree[(i, j - 1)] += 1;
                degree[(i, j)] += 1;
            }
        }
    }
    let mut q = VecDeque::new();
    q.push_back((li, lj));
    while !q.is_empty() {
        let (r, c) = q.pop_front().unwrap();
        for (nr, nc) in D4::iter(r, c, n, m) {
            if grid[(nr, nc)] == '.' {
                degree[(nr, nc)] -= 1;
                if degree[(nr, nc)] <= 1 {
                    q.push_back((nr, nc));
                    grid[(nr, nc)] = '+';
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            out!(grid[(i, j)]);
        }
        out_line!("");
    }
}


fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

fn main() {
    let mut sin = std::io::stdin();
    let input = Input::new(&mut sin);
    unsafe {
        OUTPUT = Some(Output::new(Box::new(std::io::stdout())));
    }
    run(input);
}
