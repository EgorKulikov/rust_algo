use std::ops::Add;
use std::marker::PhantomData;
use std::ops::Div;
use std::hash::Hash;
use std::ops::MulAssign;
use std::io::Read;
use std::io::Write;
use std::cmp;
use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::RemAssign;
use std::ops::IndexMut;
use std::ops::Index;
use std::ops::Mul;
use std::ops::Rem;
use std::fmt::Display;
use std::ops::Sub;
use std::ops::SubAssign;

pub trait EdgeTrait: Clone {
    const REVERSABLE: bool;
    const BIDIRECTIONAL: bool;

    fn to(&self) -> usize;
    fn id(&self) -> usize;
    fn set_id(&mut self, id: usize);
    fn reverse_id(&self) -> usize;
    fn set_reverse_id(&mut self, reverse_id: usize);
    fn reverse_edge(&self, from: usize) -> Self;
}

pub trait BiEdgeTrait: EdgeTrait {}
pub trait EdgeId: Clone {
    fn new() -> Self;
    fn id(&self) -> usize;
    fn set_id(&mut self, id: usize);
}

#[derive(Clone)]
pub struct WithId {
    id: u32,
}

impl EdgeId for WithId {
    fn new() -> Self {
        Self { id: 0 }
    }

    fn id(&self) -> usize {
        self.id as usize
    }

    fn set_id(&mut self, id: usize) {
        self.id = id as u32;
    }
}

#[derive(Clone)]
pub struct NoId {}

impl EdgeId for NoId {
    fn new() -> Self {
        Self {}
    }

    fn id(&self) -> usize {
        panic!("Id called on no id")
    }

    fn set_id(&mut self, _: usize) {}
}

#[derive(Clone)]
pub struct BiEdgeRaw<Id: EdgeId> {
    to: u32,
    id: Id,
}

impl<Id: EdgeId> BiEdgeRaw<Id> {
    pub fn new(to: usize) -> Self {
        Self {
            to: to as u32,
            id: Id::new(),
        }
    }
}

impl<Id: EdgeId> EdgeTrait for BiEdgeRaw<Id> {
    const REVERSABLE: bool = true;
    const BIDIRECTIONAL: bool = true;

    fn to(&self) -> usize {
        self.to as usize
    }

    fn id(&self) -> usize {
        self.id.id()
    }

    fn set_id(&mut self, id: usize) {
        self.id.set_id(id);
    }

    fn reverse_id(&self) -> usize {
        panic!("no reverse id")
    }

    fn set_reverse_id(&mut self, _: usize) {}

    fn reverse_edge(&self, from: usize) -> Self {
        Self::new(from)
    }
}

impl<Id: EdgeId> BiEdgeTrait for BiEdgeRaw<Id> {}

pub type BiEdge = BiEdgeRaw<NoId>;
pub type BiEdgeWithId = BiEdgeRaw<WithId>;
pub fn create_order(size: usize) -> Vec<usize> {
    create_order_with_base(size, 0)
}

pub fn create_order_with_base(size: usize, base: usize) -> Vec<usize> {
    let mut res = Vec::with_capacity(size);
    for i in base..(size + base) {
        res.push(i);
    }
    res
}

pub struct DSU {
    id: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            id: create_order(n),
            size: vec![1; n],
            count: n,
        }
    }

    pub fn size(&self, i: usize) -> usize {
        self.size[self[i]]
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn join(&mut self, mut a: usize, mut b: usize) -> bool {
        a = self[a];
        b = self[b];
        if a == b {
            false
        } else {
            self.size[a] += self.size[b];
            self.id[b] = a;
            self.count -= 1;
            true
        }
    }

    pub fn clear(&mut self) {
        self.count = self.id.len();
        self.size.fill(1);
        for i in 0..self.count {
            self.id[i] = i;
        }
    }
}

impl Index<usize> for DSU {
    type Output = usize;

    fn index(&self, i: usize) -> &Self::Output {
        if self.id[i] != i {
            let res = self[self.id[i]];
            unsafe {
                let const_ptr = self as *const Self;
                let mut_ptr = const_ptr as *mut Self;
                let mut_self = &mut *mut_ptr;
                mut_self.id[i] = res;
            }
        }
        &self.id[i]
    }
}
// use std::slice::Iter;

pub struct Graph<E: EdgeTrait> {
    edges: Vec<Vec<E>>,
    edge_count: usize,
}

impl<E: EdgeTrait> Graph<E> {
    pub fn new(vertex_count: usize) -> Self {
        Self {
            edges: vec![Vec::new(); vertex_count],
            edge_count: 0,
        }
    }

    pub fn add_edge(&mut self, from: usize, mut edge: E) -> usize {
        let to = edge.to();
        assert!(to < self.edges.len());
        let direct_id = self.edges[from].len();
        edge.set_id(self.edge_count);
        self.edges[from].push(edge);
        if E::REVERSABLE {
            let rev_id = self.edges[to].len();
            self.edges[from][direct_id].set_reverse_id(rev_id);
            let mut rev_edge = self.edges[from][direct_id].reverse_edge(from);
            rev_edge.set_id(self.edge_count);
            rev_edge.set_reverse_id(direct_id);
            self.edges[to].push(rev_edge);
        }
        self.edge_count += 1;
        direct_id
    }

    pub fn add_vertices(&mut self, cnt: usize) {
        self.edges.resize(self.edges.len() + cnt, Vec::new());
    }

    pub fn clear(&mut self) {
        self.edge_count = 0;
        for ve in self.edges.iter_mut() {
            ve.clear();
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.edges.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn is_tree(&self) -> bool {
        if !E::BIDIRECTIONAL || self.edge_count + 1 != self.vertex_count() {
            false
        } else {
            let mut dsu = DSU::new(self.vertex_count());
            for i in 0..self.vertex_count() {
                for e in self[i].iter() {
                    dsu.join(i, e.to());
                }
            }
            dsu.count() == 1
        }
    }
}

impl<E: EdgeTrait> Index<usize> for Graph<E> {
    type Output = [E];

    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
    }
}

impl<E: EdgeTrait> IndexMut<usize> for Graph<E> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.edges[index]
    }
}

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

pub trait Callable<Args, Output> {
    fn call(&mut self, args: Args) -> Output;
}

pub struct RecursiveFunction<F, Args, Output>
where
    F: FnMut(&mut dyn Callable<Args, Output>, Args) -> Output,
{
    f: F,
    phantom_args: PhantomData<Args>,
    phantom_output: PhantomData<Output>,
}

impl<F, Args, Output> RecursiveFunction<F, Args, Output>
where
    F: FnMut(&mut dyn Callable<Args, Output>, Args) -> Output,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            phantom_args: Default::default(),
            phantom_output: Default::default(),
        }
    }
}

impl<F, Args, Output> Callable<Args, Output> for RecursiveFunction<F, Args, Output>
where
    F: FnMut(&mut dyn Callable<Args, Output>, Args) -> Output,
{
    fn call(&mut self, args: Args) -> Output {
        let const_ptr = &self.f as *const F;
        let mut_ptr = const_ptr as *mut F;
        unsafe { (&mut *mut_ptr)(self, args) }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let ranges = input.read_vec::<(i32, i32)>(n);
    let edges = input.read_vec::<(usize, usize)>(n - 1);

    let mut graph = Graph::new(n);
    for (from, to) in edges {
        graph.add_edge(from - 1, BiEdge::new(to - 1));
    }
    assert!(graph.is_tree());
    let mut index = 0usize;
    let mut dfs = RecursiveFunction::new(|f, (vert, parent): (usize, usize)| -> (u64, u64) {
        let mut low = 0u64;
        let mut high = 0u64;
        index += 1;
        for e in graph[vert].iter() {
            let next = e.to();
            if next == parent {
                continue;
            }
            let (clow, chigh) = f.call((next, vert));
            low += cmp::max(
                (ranges[vert].0 - ranges[next].0).abs() as u64 + clow,
                (ranges[vert].0 - ranges[next].1).abs() as u64 + chigh,
            );
            high += cmp::max(
                (ranges[vert].1 - ranges[next].0).abs() as u64 + clow,
                (ranges[vert].1 - ranges[next].1).abs() as u64 + chigh,
            );
        }
        (low, high)
    });
    let res = dfs.call((0, n));
    assert_eq!(index, n);
    out_line!(res.0.max(res.1));
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
