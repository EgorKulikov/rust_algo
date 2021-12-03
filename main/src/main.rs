use std::marker::PhantomData;
use std::ops::Add;
use std::convert::From;
use std::ops::SubAssign;
use std::ops::Mul;
use std::mem::swap;
use std::ops::Neg;
use std::ops::Div;
use std::fmt::Formatter;
use std::io::Read;
use std::io::Write;
use std::hash::Hasher;
use std::ops::Rem;
use std::ops::MulAssign;
use std::hash::Hash;
use std::ops::RemAssign;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::fmt::Display;
use std::ops::Sub;
use std::collections::HashMap;

pub trait MinimMaxim: PartialOrd + Sized {
    fn minim(&mut self, other: Self) -> bool {
        if other < *self {
            *self = other;
            true
        } else {
            false
        }
    }

    fn maxim(&mut self, other: Self) -> bool {
        if other > *self {
            *self = other;
            true
        } else {
            false
        }
    }
}

impl<T: PartialOrd + Sized> MinimMaxim for T {}

pub trait Addable: Add<Output = Self> + AddAssign + Copy {}
impl<T: Add<Output = Self> + AddAssign + Copy> Addable for T {}

pub trait AddSub: Addable + Sub<Output = Self> + SubAssign {}
impl<T: Addable + Sub<Output = Self> + SubAssign> AddSub for T {}
pub trait ZeroOne {
    fn zero() -> Self;
    fn one() -> Self;
}

macro_rules! zero_one_integer_impl {
    ($t: ident) => {
        impl ZeroOne for $t {
            fn zero() -> Self {
                0
            }

            fn one() -> Self {
                1
            }
        }
    };
}

zero_one_integer_impl!(i128);
zero_one_integer_impl!(i64);
zero_one_integer_impl!(i32);
zero_one_integer_impl!(i16);
zero_one_integer_impl!(i8);
zero_one_integer_impl!(isize);
zero_one_integer_impl!(u128);
zero_one_integer_impl!(u64);
zero_one_integer_impl!(u32);
zero_one_integer_impl!(u16);
zero_one_integer_impl!(u8);
zero_one_integer_impl!(usize);

macro_rules! zero_one_float_impl {
    ($t: ident) => {
        impl ZeroOne for $t {
            fn zero() -> Self {
                0.
            }

            fn one() -> Self {
                1.
            }
        }
    };
}

zero_one_float_impl!(f32);
zero_one_float_impl!(f64);

pub struct FenwickTree<T: AddSub + ZeroOne> {
    value: Vec<T>,
}

impl<T: AddSub + ZeroOne> FenwickTree<T> {
    pub fn new(size: usize) -> Self {
        Self {
            value: vec![T::zero(); size],
        }
    }

    pub fn get_to(&self, mut to: usize) -> T {
        to.minim(self.value.len());
        let mut result = T::zero();
        while to > 0 {
            to -= 1;
            result += self.value[to];
            to &= to + 1;
        }
        result
    }

    pub fn get(&self, from: usize, to: usize) -> T {
        if from >= to {
            T::zero()
        } else {
            self.get_to(to) - self.get_to(from)
        }
    }

    pub fn add(&mut self, mut at: usize, v: T) {
        while at < self.value.len() {
            self.value[at] += v;
            at |= at + 1;
        }
    }

    pub fn clear(&mut self) {
        self.value.fill(T::zero());
    }
}
pub trait FromU8 {
    fn from_u8(val: u8) -> Self;
}

macro_rules! from_u8_impl {
    ($t: ident) => {
        impl FromU8 for $t {
            fn from_u8(val: u8) -> Self {
                val as $t
            }
        }
    };
}

from_u8_impl!(i128);
from_u8_impl!(i64);
from_u8_impl!(i32);
from_u8_impl!(i16);
from_u8_impl!(i8);
from_u8_impl!(isize);
from_u8_impl!(u128);
from_u8_impl!(u64);
from_u8_impl!(u32);
from_u8_impl!(u16);
from_u8_impl!(u8);
from_u8_impl!(usize);
from_u8_impl!(f32);
from_u8_impl!(f64);

pub trait Multable: Mul<Output = Self> + MulAssign + Copy {}
impl<T: Mul<Output = Self> + MulAssign + Copy> Multable for T {}

pub trait MulDiv: Multable + Div<Output = Self> + DivAssign {}
impl<T: Multable + Div<Output = Self> + DivAssign> MulDiv for T {}

pub trait MulDivRem: MulDiv + Rem<Output = Self> + RemAssign {}
impl<T: MulDiv + Rem<Output = Self> + RemAssign> MulDivRem for T {}
pub trait IsSigned {
    const SIGNED: bool;
}

pub trait Signed: IsSigned {}

pub trait Unsigned: IsSigned {}

macro_rules! unsigned_impl {
    ($t: ident) => {
        impl Unsigned for $t {}

        impl IsSigned for $t {
            const SIGNED: bool = false;
        }
    };
}

unsigned_impl!(u128);
unsigned_impl!(u64);
unsigned_impl!(u32);
unsigned_impl!(u16);
unsigned_impl!(u8);
unsigned_impl!(usize);

macro_rules! signed_impl {
    ($t: ident) => {
        impl Signed for $t {}

        impl IsSigned for $t {
            const SIGNED: bool = true;
        }
    };
}

signed_impl!(i128);
signed_impl!(i64);
signed_impl!(i32);
signed_impl!(i16);
signed_impl!(i8);
signed_impl!(isize);
signed_impl!(f64);
signed_impl!(f32);

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

    fn read_integer<T: IsSigned + ZeroOne + FromU8 + AddSub + Multable + Display>(&mut self) -> T {
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

/*impl<T: Readable, const N: usize> Readable for [T; N] {
    fn read(input: &mut Input) -> Self {
        let mut arr: [T; N] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            arr[i] = T::read(input);
        }
        arr
    }
}*/

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
    };
    () => {
        output().put(b'\n');
    };
}

pub trait Wideable: Sized {
    type W: From<Self>;

    fn downcast(w: Self::W) -> Self;
}

macro_rules! wideable_impl {
    ($t: ident, $w: ident) => {
        impl Wideable for $t {
            type W = $w;

            fn downcast(w: Self::W) -> Self {
                w as $t
            }
        }
    };
}

wideable_impl!(i128, i128);
wideable_impl!(i64, i128);
wideable_impl!(i32, i64);
wideable_impl!(i16, i32);
wideable_impl!(i8, i16);
wideable_impl!(isize, isize);
wideable_impl!(u128, u128);
wideable_impl!(u64, u128);
wideable_impl!(u32, u64);
wideable_impl!(u16, u32);
wideable_impl!(u8, u16);
wideable_impl!(usize, usize);
wideable_impl!(f64, f64);
wideable_impl!(f32, f64);

pub fn extended_gcd<T: Copy + ZeroOne + AddSub + MulDivRem + Wideable + PartialEq>(
    a: T,
    b: T,
) -> (T, T::W, T::W)
where
    T::W: Copy + ZeroOne + AddSub + Multable,
{
    if a == T::zero() {
        (b, T::W::zero(), T::W::one())
    } else {
        let (d, y, mut x) = extended_gcd(b % a, a);
        x -= T::W::from(b / a) * y;
        (d, x, y)
    }
}

pub fn gcd<T: Copy + ZeroOne + MulDivRem + PartialEq>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        a %= b;
        swap(&mut a, &mut b);
    }
    a
}

pub fn lcm<T: Copy + ZeroOne + MulDivRem + PartialEq>(a: T, b: T) -> T {
    (a / gcd(a, b)) * b
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

pub trait BaseModInt: AddSub + MulDiv + Neg + Copy + ZeroOne + PartialEq {
    type W: AddSub + MulDivRem + Copy + ZeroOne;
    type T: AddSub + MulDivRem + Copy + PartialEq + ZeroOne + Wideable<W = Self::W> + Ord;

    fn new(n: Self::T) -> Self;
    fn new_from_long(n: <Self::T as Wideable>::W) -> Self;
    fn inv(&self) -> Option<Self>;
    fn module() -> Self::T;
}

#[derive(Copy, Clone, PartialEq)]
pub struct ModInt<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    n: T,
    phantom: PhantomData<V>,
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn safe_new(n: T) -> Self {
        debug_assert!(n >= T::zero() && n < V::val());
        Self {
            n,
            phantom: Default::default(),
        }
    }

    fn safe(mut n: T) -> T {
        assert!(n < V::val() + V::val() && n >= T::zero());
        if n >= V::val() {
            n -= V::val();
        }
        n
    }

    fn make_safe(&mut self) {
        self.n = Self::safe(self.n);
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord + Hash, V: Value<T>>
    ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    pub fn log(&self, alpha: Self) -> T {
        let mut base = HashMap::new();
        let mut exp = T::zero();
        let mut pow = Self::one();
        let mut inv = *self;
        let alpha_inv = alpha.inv().unwrap();
        while exp * exp < Self::module() {
            if inv == Self::one() {
                return exp;
            }
            base.insert(inv, exp);
            exp += T::one();
            pow *= alpha;
            inv *= alpha_inv;
        }
        let step = pow;
        let mut i = T::one();
        loop {
            if let Some(b) = base.get(&pow) {
                break exp * i + *b;
            }
            pow *= step;
            i += T::one();
        }
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> BaseModInt
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type T = T;
    type W = T::W;

    fn new(n: T) -> Self {
        Self::safe_new(Self::safe(n % (V::val()) + V::val()))
    }

    fn new_from_long(n: T::W) -> Self {
        Self::safe_new(Self::safe(T::downcast(n % (V::val()).into()) + V::val()))
    }

    fn inv(&self) -> Option<Self> {
        let (g, x, _) = extended_gcd(self.n, V::val());
        if g != T::one() {
            None
        } else {
            Some(Self::new_from_long(x))
        }
    }

    fn module() -> T {
        V::val()
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> From<T>
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn from(n: T) -> Self {
        Self::new(n)
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> AddAssign
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn add_assign(&mut self, rhs: Self) {
        self.n += rhs.n;
        self.make_safe();
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> Add
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> SubAssign
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.n += V::val() - rhs.n;
        self.make_safe();
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> Sub
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> MulAssign
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.n = T::downcast(T::W::from(self.n) * T::W::from(rhs.n) % T::W::from(V::val()));
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> Mul
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> DivAssign
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv().unwrap();
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> Div
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> Neg
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.n = V::val() - self.n;
        self.make_safe();
        self
    }
}

impl<
        T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord + Display,
        V: Value<T>,
    > Display for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <T as Display>::fmt(&self.n, f)
    }
}

impl<
        T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord + Readable,
        V: Value<T>,
    > Readable for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn read(input: &mut Input) -> Self {
        Self::new(T::read(input))
    }
}

impl<
        T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord + Writable,
        V: Value<T>,
    > Writable for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn write(&self, output: &mut Output) {
        self.n.write(output);
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> ZeroOne
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn zero() -> Self {
        Self::new(T::zero())
    }

    fn one() -> Self {
        Self::new(T::one())
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord, V: Value<T>> Wideable
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    type W = Self;

    fn downcast(w: Self::W) -> Self {
        w
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord + FromU8, V: Value<T>>
    FromU8 for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn from_u8(n: u8) -> Self {
        Self::new(T::from_u8(n))
    }
}

impl<
        T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord + Display + FromU8,
        V: Value<T>,
    > std::fmt::Debug for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let max = T::from_u8(100);
        if self.n <= max {
            write!(f, "{}", self.n)
        } else if self.n >= V::val() - max {
            write!(f, "-{}", V::val() - self.n)
        } else {
            let mut denum = T::one();
            while denum < max {
                let mut num = T::one();
                while num < max {
                    if Self::new(num) / Self::new(denum) == *self {
                        return write!(f, "{}/{}", num, denum);
                    }
                    if -Self::new(num) / Self::new(denum) == *self {
                        return write!(f, "-{}/{}", num, denum);
                    }
                    num += T::one();
                }
                denum += T::one();
            }
            write!(f, "(?? {} ??)", self.n)
        }
    }
}

impl<T: AddSub + MulDivRem + Copy + PartialEq + Wideable + ZeroOne + Ord + Hash, V: Value<T>> Hash
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.n.hash(state)
    }
}

impl<T: AddSub + MulDivRem + Copy + Eq + Wideable + Hash + ZeroOne + Ord, V: Value<T>> Eq
    for ModInt<T, V>
where
    T::W: AddSub + MulDivRem + Copy + ZeroOne,
{
}

value!(Val7, u32, 1_000_000_007);
pub type ModInt7 = ModInt<u32, Val7>;

value!(Val9, u32, 1_000_000_009);
pub type ModInt9 = ModInt<u32, Val9>;

value!(ValF, u32, 998_244_353);
pub type ModIntF = ModInt<u32, ValF>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a = input.read_vec::<i64>(n);

    let mut sum = 0i64;
    type Mod = ModIntF;

    let mut last_pos = HashMap::new();
    let mut ft: FenwickTree<Mod> = FenwickTree::new(n);
    ft.add(0, Mod::one());
    for (i, v) in a.into_iter().enumerate() {
        if i == n - 1 {
            break;
        }
        sum += v;
        let from = last_pos.insert(sum, i + 1).unwrap_or(0);
        ft.add(i + 1, ft.get(from, i + 1));
    }
    out_line!(ft.get(0, n));
}


fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
