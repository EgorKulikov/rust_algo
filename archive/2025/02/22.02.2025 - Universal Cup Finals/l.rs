// L: Not Another Constructive Problem
use crate::algo_lib::collections::md_arr::arr2d::Arr2dRead;
use crate::algo_lib::collections::slice_ext::permutation::Permutation;
use crate::algo_lib::collections::vec_ext::inc_dec::IncDec;
use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::Output;
use crate::algo_lib::misc::memo::memoization_2d::Memoization2d;
use crate::algo_lib::misc::recursive_function::Callable2;
use crate::algo_lib::misc::test_type::TaskType;
use crate::algo_lib::misc::test_type::TestType;
use crate::algo_lib::numbers::mod_int::ModInt7;
use crate::algo_lib::numbers::num_traits::algebra::{One, Zero};
type PreCalc = ();
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec();
    let q = input.read_size_vec(n).dec();
    type Mod = ModInt7;
    let c = input.read_table::<Mod>(n, n);
    let p_inv = p.inv();
    let q = p_inv.mul(&q);
    let mut cyc = Vec::new();
    let mut cur = 0;
    loop {
        cyc.push(cur);
        cur = q[cur];
        if cur == 0 {
            break;
        }
    }
    if cyc.len() != n {
        out.print_line(Mod::zero());
        return;
    }
    let mut mem = Memoization2d::new(
        n,
        n,
        |mem, i, j| -> (Mod, Mod) {
            if i == j {
                (Mod::one(), Mod::zero())
            } else {
                let mut b = Mod::zero();
                for k in i + 1.. {
                    b += mem.call(i, (k - 1) % n).0 * mem.call(k % n, j).0;
                    if k % n == j {
                        break;
                    }
                }
                b *= c[(cyc[i], cyc[j])];
                let mut a = b;
                for k in i + 1.. {
                    if k % n == j {
                        break;
                    }
                    a += mem.call(i, k % n).1 * mem.call(k % n, j).0;
                }
                (a, b)
            }
        },
    );
    out.print_line(mem.call(0, n - 1).0);
}
pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;
pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}


fn main() {
    let input = crate::algo_lib::io::input::Input::stdin();
    let output = crate::algo_lib::io::output::Output::stdout();
    run(input, output);
}
pub mod algo_lib {
pub mod collections {
pub mod fx_hash_map {
use crate::algo_lib::misc::lazy_lock::LazyLock;
use std::convert::TryInto;
use std::time::SystemTime;
use std::{
    collections::{HashMap, HashSet},
    hash::{BuildHasherDefault, Hasher},
    ops::BitXor,
};
pub type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
pub type FxHashSet<V> = HashSet<V, BuildHasherDefault<FxHasher>>;
#[derive(Default)]
pub struct FxHasher {
    hash: usize,
}
static K: LazyLock<usize> = LazyLock::new(|| {
    ((SystemTime::UNIX_EPOCH.elapsed().unwrap().as_nanos().wrapping_mul(2) + 1)
        & 0xFFFFFFFFFFFFFFFF) as usize
});
impl FxHasher {
    #[inline]
    fn add_to_hash(&mut self, i: usize) {
        self.hash = self.hash.rotate_left(5).bitxor(i).wrapping_mul(*K);
    }
}
impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        let read_usize = |bytes: &[u8]| u64::from_ne_bytes(
            bytes[..8].try_into().unwrap(),
        );
        let mut hash = FxHasher { hash: self.hash };
        while bytes.len() >= 8 {
            hash.add_to_hash(read_usize(bytes) as usize);
            bytes = &bytes[8..];
        }
        if bytes.len() >= 4 {
            hash.add_to_hash(
                u32::from_ne_bytes(bytes[..4].try_into().unwrap()) as usize,
            );
            bytes = &bytes[4..];
        }
        if bytes.len() >= 2 {
            hash.add_to_hash(
                u16::from_ne_bytes(bytes[..2].try_into().unwrap()) as usize,
            );
            bytes = &bytes[2..];
        }
        if !bytes.is_empty() {
            hash.add_to_hash(bytes[0] as usize);
        }
        self.hash = hash.hash;
    }
    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add_to_hash(i as usize);
    }
    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.add_to_hash(i as usize);
    }
    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add_to_hash(i as usize);
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add_to_hash(i as usize);
    }
    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add_to_hash(i);
    }
    #[inline]
    fn finish(&self) -> u64 {
        self.hash as u64
    }
}
}
pub mod md_arr {
pub mod arr2d {
use crate::algo_lib::io::input::{Input, Readable};
use crate::algo_lib::io::output::{Output, Writable};
use std::iter::{Skip, StepBy, Take};
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut, Range};
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;
#[derive(Clone, Eq, PartialEq, Default, Debug, Hash, Ord, PartialOrd)]
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
    pub fn with_gen<F>(d1: usize, d2: usize, mut g: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut data = Vec::with_capacity(d1 * d2);
        for i in 0usize..d1 {
            for j in 0usize..d2 {
                data.push(g(i, j));
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
    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }
    pub fn row(&self, row: usize) -> Take<Skip<Iter<'_, T>>> {
        assert!(row < self.d1);
        self.data.iter().skip(row * self.d2).take(self.d2)
    }
    pub fn row_mut(&mut self, row: usize) -> Take<Skip<IterMut<'_, T>>> {
        assert!(row < self.d1);
        self.data.iter_mut().skip(row * self.d2).take(self.d2)
    }
    pub fn col(&self, col: usize) -> StepBy<Skip<Iter<'_, T>>> {
        assert!(col < self.d2);
        self.data.iter().skip(col).step_by(self.d2)
    }
    pub fn col_mut(&mut self, col: usize) -> StepBy<Skip<IterMut<'_, T>>> {
        assert!(col < self.d2);
        self.data.iter_mut().skip(col).step_by(self.d2)
    }
    pub fn swap(&mut self, r1: usize, c1: usize, r2: usize, c2: usize) {
        assert!(c1 < self.d2);
        assert!(c2 < self.d2);
        self.data.swap(r1 * self.d2 + c1, r2 * self.d2 + c2);
    }
    pub fn rows(&self) -> Range<usize> {
        0..self.d1
    }
    pub fn cols(&self) -> Range<usize> {
        0..self.d2
    }
    pub fn swap_rows(&mut self, r1: usize, r2: usize) {
        if r1 == r2 {
            assert!(r1 < self.d1);
            return;
        }
        let (r1, r2) = (r1.min(r2), r1.max(r2));
        let (head, tail) = self.data.split_at_mut(r2 * self.d2);
        head[r1 * self.d2..(r1 + 1) * self.d2].swap_with_slice(&mut tail[..self.d2]);
    }
    pub fn find(&self, value: &T) -> Option<(usize, usize)>
    where
        T: PartialEq,
    {
        self.data
            .iter()
            .position(|x| x == value)
            .map(|pos| (pos / self.d2, pos % self.d2))
    }
    pub fn rotate_clockwise(self) -> Self {
        unsafe {
            let d1 = self.d1;
            let d2 = self.d2;
            let mut res = MaybeUninit::new(Vec::with_capacity(d1 * d2));
            (*res.as_mut_ptr()).set_len(d1 * d2);
            for (id, element) in self.into_iter().enumerate() {
                let (i, j) = (id / d2, id % d2);
                let ptr: *mut T = (*res.as_mut_ptr()).as_mut_ptr();
                ptr.add(j * d1 + d1 - i - 1).write(element);
            }
            Self {
                d1: d2,
                d2: d1,
                data: res.assume_init(),
            }
        }
    }
    pub fn rotate_counterclockwise(self) -> Self {
        unsafe {
            let d1 = self.d1;
            let d2 = self.d2;
            let mut res = MaybeUninit::new(Vec::with_capacity(d1 * d2));
            (*res.as_mut_ptr()).set_len(d1 * d2);
            for (id, element) in self.into_iter().enumerate() {
                let (i, j) = (id / d2, id % d2);
                let ptr: *mut T = (*res.as_mut_ptr()).as_mut_ptr();
                ptr.add((d2 - j - 1) * d1 + i).write(element);
            }
            Self {
                d1: d2,
                d2: d1,
                data: res.assume_init(),
            }
        }
    }
    pub fn transpose(self) -> Self {
        unsafe {
            let d1 = self.d1;
            let d2 = self.d2;
            let mut res = MaybeUninit::new(Vec::with_capacity(d1 * d2));
            (*res.as_mut_ptr()).set_len(d1 * d2);
            for (id, element) in self.into_iter().enumerate() {
                let (i, j) = (id / d2, id % d2);
                let ptr: *mut T = (*res.as_mut_ptr()).as_mut_ptr();
                ptr.add(j * d1 + i).write(element);
            }
            Self {
                d1: d2,
                d2: d1,
                data: res.assume_init(),
            }
        }
    }
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }
}
impl<T: Clone> Arr2d<T> {
    pub fn fill(&mut self, elem: T) {
        self.data.fill(elem);
    }
}
impl<T> Index<(usize, usize)> for Arr2d<T> {
    type Output = T;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        assert!(col < self.d2);
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
        assert!(col < self.d2);
        &mut self.data[self.d2 * row + col]
    }
}
impl<T> IndexMut<usize> for Arr2d<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        &mut self.data[self.d2 * index..self.d2 * (index + 1)]
    }
}
impl<T> AsRef<Vec<T>> for Arr2d<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.data
    }
}
impl<T> AsMut<Vec<T>> for Arr2d<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
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
                    output.put(output.separator());
                }
                self.data[at].write(output);
                at += 1;
            }
        }
    }
}
impl<T> IntoIterator for Arr2d<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
impl<'a, T> IntoIterator for &'a Arr2d<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
pub trait Arr2dRead {
    fn read_table<T: Readable>(&mut self, d1: usize, d2: usize) -> Arr2d<T>;
    fn read_int_table(&mut self, d1: usize, d2: usize) -> Arr2d<i32>;
    fn read_long_table(&mut self, d1: usize, d2: usize) -> Arr2d<i64>;
    fn read_size_table(&mut self, d1: usize, d2: usize) -> Arr2d<usize>;
    fn read_char_table(&mut self, d1: usize, d2: usize) -> Arr2d<u8>;
}
impl Arr2dRead for Input {
    fn read_table<T: Readable>(&mut self, d1: usize, d2: usize) -> Arr2d<T> {
        Arr2d::with_gen(d1, d2, |_, _| self.read())
    }
    fn read_int_table(&mut self, d1: usize, d2: usize) -> Arr2d<i32> {
        self.read_table(d1, d2)
    }
    fn read_long_table(&mut self, d1: usize, d2: usize) -> Arr2d<i64> {
        self.read_table(d1, d2)
    }
    fn read_size_table(&mut self, d1: usize, d2: usize) -> Arr2d<usize> {
        self.read_table(d1, d2)
    }
    fn read_char_table(&mut self, d1: usize, d2: usize) -> Arr2d<u8> {
        self.read_table(d1, d2)
    }
}
pub trait Arr2dCharWrite {
    fn print_table(&mut self, table: &Arr2d<u8>);
}
impl Arr2dCharWrite for Output<'_> {
    fn print_table(&mut self, table: &Arr2d<u8>) {
        let mut at = 0usize;
        for _ in 0..table.d1 {
            for _ in 0..table.d2 {
                self.put(table.data[at]);
                at += 1;
            }
            self.put(b'\n');
        }
    }
}
impl<T: Readable> Readable for Arr2d<T> {
    fn read(input: &mut Input) -> Self {
        let d1 = input.read();
        let d2 = input.read();
        input.read_table(d1, d2)
    }
}
}
}
pub mod slice_ext {
pub mod next_permutation {
pub trait NextPermutation {
    fn next_permutation(&mut self) -> bool;
}
impl<T: Ord> NextPermutation for [T] {
    fn next_permutation(&mut self) -> bool {
        if self.len() <= 1 {
            return false;
        }
        let mut i = self.len() - 1;
        while i > 0 && self[i - 1] >= self[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        let mut j = self.len() - 1;
        while self[j] <= self[i - 1] {
            j -= 1;
        }
        self.swap(i - 1, j);
        self[i..].reverse();
        true
    }
}
}
pub mod permutation {
use crate::algo_lib::collections::slice_ext::next_permutation::NextPermutation;
pub trait Permutation: NextPermutation {
    fn inv(&self) -> Vec<usize>;
    fn mul(&self, other: &Self) -> Vec<usize>;
}
impl Permutation for [usize] {
    fn inv(&self) -> Vec<usize> {
        let mut inv = vec![0; self.len()];
        for i in 0..self.len() {
            inv[self[i]] = i;
        }
        inv
    }
    fn mul(&self, other: &Self) -> Vec<usize> {
        assert_eq!(self.len(), other.len());
        let mut res = vec![0; self.len()];
        for i in 0..self.len() {
            res[i] = self[other[i]];
        }
        res
    }
}
}
}
pub mod vec_ext {
pub mod inc_dec {
use crate::algo_lib::numbers::num_traits::algebra::{AdditionMonoidWithSub, One};
pub trait IncDec {
    #[must_use]
    fn inc(self) -> Self;
    #[must_use]
    fn dec(self) -> Self;
}
impl<T: AdditionMonoidWithSub + One> IncDec for T {
    fn inc(self) -> Self {
        self + T::one()
    }
    fn dec(self) -> Self {
        self - T::one()
    }
}
impl<T: AdditionMonoidWithSub + One> IncDec for Vec<T> {
    fn inc(mut self) -> Self {
        self.iter_mut().for_each(|i| *i += T::one());
        self
    }
    fn dec(mut self) -> Self {
        self.iter_mut().for_each(|i| *i -= T::one());
        self
    }
}
impl<T: AdditionMonoidWithSub + One> IncDec for Vec<Vec<T>> {
    fn inc(mut self) -> Self {
        self.iter_mut().for_each(|v| v.iter_mut().for_each(|i| *i += T::one()));
        self
    }
    fn dec(mut self) -> Self {
        self.iter_mut().for_each(|v| v.iter_mut().for_each(|i| *i -= T::one()));
        self
    }
}
impl<T: AdditionMonoidWithSub + One, U: AdditionMonoidWithSub + One> IncDec
for Vec<(T, U)> {
    fn inc(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j)| {
                *i += T::one();
                *j += U::one();
            });
        self
    }
    fn dec(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j)| {
                *i -= T::one();
                *j -= U::one();
            });
        self
    }
}
impl<T: AdditionMonoidWithSub + One, U: AdditionMonoidWithSub + One, V> IncDec
for Vec<(T, U, V)> {
    fn inc(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j, _)| {
                *i += T::one();
                *j += U::one();
            });
        self
    }
    fn dec(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j, _)| {
                *i -= T::one();
                *j -= U::one();
            });
        self
    }
}
impl<T: AdditionMonoidWithSub + One, U: AdditionMonoidWithSub + One, V, W> IncDec
for Vec<(T, U, V, W)> {
    fn inc(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j, ..)| {
                *i += T::one();
                *j += U::one();
            });
        self
    }
    fn dec(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j, ..)| {
                *i -= T::one();
                *j -= U::one();
            });
        self
    }
}
impl<T: AdditionMonoidWithSub + One, U: AdditionMonoidWithSub + One, V, W, X> IncDec
for Vec<(T, U, V, W, X)> {
    fn inc(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j, ..)| {
                *i += T::one();
                *j += U::one();
            });
        self
    }
    fn dec(mut self) -> Self {
        self.iter_mut()
            .for_each(|(i, j, ..)| {
                *i -= T::one();
                *j -= U::one();
            });
        self
    }
}
impl<T: AdditionMonoidWithSub + One, U: AdditionMonoidWithSub + One> IncDec for (T, U) {
    fn inc(mut self) -> Self {
        self.0 += T::one();
        self.1 += U::one();
        self
    }
    fn dec(mut self) -> Self {
        self.0 -= T::one();
        self.1 -= U::one();
        self
    }
}
}
}
}
pub mod io {
pub mod input {
use std::fs::File;
use std::io::{Read, Stdin};
use std::mem::MaybeUninit;
enum InputSource {
    Stdin(Stdin),
    File(File),
    Slice,
    Delegate(Box<dyn Read + Send>),
}
pub struct Input {
    input: InputSource,
    buf: Vec<u8>,
    at: usize,
    buf_read: usize,
    eol: bool,
}
macro_rules! read_impl {
    ($t:ty, $read_name:ident, $read_vec_name:ident) => {
        pub fn $read_name (& mut self) -> $t { self.read() } pub fn $read_vec_name (& mut
        self, len : usize) -> Vec <$t > { self.read_vec(len) }
    };
    ($t:ty, $read_name:ident, $read_vec_name:ident, $read_pair_vec_name:ident) => {
        read_impl!($t, $read_name, $read_vec_name); pub fn $read_pair_vec_name (& mut
        self, len : usize) -> Vec < ($t, $t) > { self.read_vec(len) }
    };
}
impl Input {
    const DEFAULT_BUF_SIZE: usize = 4096;
    pub fn slice(input: &[u8]) -> Self {
        Self {
            input: InputSource::Slice,
            buf: input.to_vec(),
            at: 0,
            buf_read: input.len(),
            eol: true,
        }
    }
    pub fn stdin() -> Self {
        Self {
            input: InputSource::Stdin(std::io::stdin()),
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            buf_read: 0,
            eol: true,
        }
    }
    pub fn file(file: File) -> Self {
        Self {
            input: InputSource::File(file),
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            buf_read: 0,
            eol: true,
        }
    }
    pub fn delegate(reader: impl Read + Send + 'static) -> Self {
        Self {
            input: InputSource::Delegate(Box::new(reader)),
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            buf_read: 0,
            eol: true,
        }
    }
    pub fn get(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            let res = self.buf[self.at];
            self.at += 1;
            if res == b'\r' {
                self.eol = true;
                if self.refill_buffer() && self.buf[self.at] == b'\n' {
                    self.at += 1;
                }
                return Some(b'\n');
            }
            self.eol = res == b'\n';
            Some(res)
        } else {
            None
        }
    }
    pub fn peek(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            let res = self.buf[self.at];
            Some(if res == b'\r' { b'\n' } else { res })
        } else {
            None
        }
    }
    pub fn skip_whitespace(&mut self) {
        while let Some(b) = self.peek() {
            if !b.is_ascii_whitespace() {
                return;
            }
            self.get();
        }
    }
    pub fn next_token(&mut self) -> Option<Vec<u8>> {
        self.skip_whitespace();
        let mut res = Vec::new();
        while let Some(c) = self.get() {
            if c.is_ascii_whitespace() {
                break;
            }
            res.push(c);
        }
        if res.is_empty() { None } else { Some(res) }
    }
    pub fn is_exhausted(&mut self) -> bool {
        self.peek().is_none()
    }
    pub fn is_empty(&mut self) -> bool {
        self.skip_whitespace();
        self.is_exhausted()
    }
    pub fn read<T: Readable>(&mut self) -> T {
        T::read(self)
    }
    pub fn read_vec<T: Readable>(&mut self, size: usize) -> Vec<T> {
        let mut res = Vec::with_capacity(size);
        for _ in 0..size {
            res.push(self.read());
        }
        res
    }
    pub fn read_char(&mut self) -> u8 {
        self.skip_whitespace();
        self.get().unwrap()
    }
    read_impl!(u32, read_unsigned, read_unsigned_vec);
    read_impl!(u64, read_u64, read_u64_vec);
    read_impl!(usize, read_size, read_size_vec, read_size_pair_vec);
    read_impl!(i32, read_int, read_int_vec, read_int_pair_vec);
    read_impl!(i64, read_long, read_long_vec, read_long_pair_vec);
    read_impl!(i128, read_i128, read_i128_vec);
    fn refill_buffer(&mut self) -> bool {
        if self.at == self.buf_read {
            self.at = 0;
            self.buf_read = match &mut self.input {
                InputSource::Stdin(stdin) => stdin.read(&mut self.buf).unwrap(),
                InputSource::File(file) => file.read(&mut self.buf).unwrap(),
                InputSource::Delegate(reader) => reader.read(&mut self.buf).unwrap(),
                InputSource::Slice => 0,
            };
            self.buf_read != 0
        } else {
            true
        }
    }
    pub fn is_eol(&self) -> bool {
        self.eol
    }
}
pub trait Readable {
    fn read(input: &mut Input) -> Self;
}
impl Readable for u8 {
    fn read(input: &mut Input) -> Self {
        input.read_char()
    }
}
impl<T: Readable> Readable for Vec<T> {
    fn read(input: &mut Input) -> Self {
        let size = input.read();
        input.read_vec(size)
    }
}
impl<T: Readable, const SIZE: usize> Readable for [T; SIZE] {
    fn read(input: &mut Input) -> Self {
        unsafe {
            let mut res = MaybeUninit::<[T; SIZE]>::uninit();
            for i in 0..SIZE {
                let ptr: *mut T = (*res.as_mut_ptr()).as_mut_ptr();
                ptr.add(i).write(input.read::<T>());
            }
            res.assume_init()
        }
    }
}
macro_rules! read_integer {
    ($($t:ident)+) => {
        $(impl Readable for $t { fn read(input : & mut Input) -> Self { input
        .skip_whitespace(); let mut c = input.get().unwrap(); let sgn = match c { b'-' =>
        { c = input.get().unwrap(); true } b'+' => { c = input.get().unwrap(); false } _
        => false, }; let mut res = 0; loop { assert!(c.is_ascii_digit()); res *= 10; let
        d = (c - b'0') as $t; if sgn { res -= d; } else { res += d; } match input.get() {
        None => break, Some(ch) => { if ch.is_ascii_whitespace() { break; } else { c =
        ch; } } } } res } })+
    };
}
read_integer!(i8 i16 i32 i64 i128 isize u16 u32 u64 u128 usize);
macro_rules! tuple_readable {
    ($($name:ident)+) => {
        impl <$($name : Readable),+> Readable for ($($name,)+) { fn read(input : & mut
        Input) -> Self { ($($name ::read(input),)+) } }
    };
}
tuple_readable! {
    T
}
tuple_readable! {
    T U
}
tuple_readable! {
    T U V
}
tuple_readable! {
    T U V X
}
tuple_readable! {
    T U V X Y
}
tuple_readable! {
    T U V X Y Z
}
tuple_readable! {
    T U V X Y Z A
}
tuple_readable! {
    T U V X Y Z A B
}
tuple_readable! {
    T U V X Y Z A B C
}
tuple_readable! {
    T U V X Y Z A B C D
}
tuple_readable! {
    T U V X Y Z A B C D E
}
tuple_readable! {
    T U V X Y Z A B C D E F
}
}
pub mod output {
use std::cmp::Reverse;
use std::fs::File;
use std::io::{Stdout, Write};
#[derive(Copy, Clone)]
pub enum BoolOutput {
    YesNo,
    YesNoCaps,
    PossibleImpossible,
    Custom(&'static str, &'static str),
}
impl BoolOutput {
    pub fn output(&self, output: &mut Output, val: bool) {
        (if val { self.yes() } else { self.no() }).write(output);
    }
    fn yes(&self) -> &str {
        match self {
            BoolOutput::YesNo => "Yes",
            BoolOutput::YesNoCaps => "YES",
            BoolOutput::PossibleImpossible => "Possible",
            BoolOutput::Custom(yes, _) => yes,
        }
    }
    fn no(&self) -> &str {
        match self {
            BoolOutput::YesNo => "No",
            BoolOutput::YesNoCaps => "NO",
            BoolOutput::PossibleImpossible => "Impossible",
            BoolOutput::Custom(_, no) => no,
        }
    }
}
enum OutputDest<'s> {
    Stdout(Stdout),
    File(File),
    Buf(&'s mut Vec<u8>),
    Delegate(Box<dyn Write + 's>),
}
pub struct Output<'s> {
    output: OutputDest<'s>,
    buf: Vec<u8>,
    at: usize,
    bool_output: BoolOutput,
    precision: Option<usize>,
    separator: u8,
}
impl<'s> Output<'s> {
    pub fn buf(buf: &'s mut Vec<u8>) -> Self {
        Self::new(OutputDest::Buf(buf))
    }
    pub fn delegate(delegate: impl Write + 'static) -> Self {
        Self::new(OutputDest::Delegate(Box::new(delegate)))
    }
    fn new(output: OutputDest<'s>) -> Self {
        Self {
            output,
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            bool_output: BoolOutput::YesNoCaps,
            precision: None,
            separator: b' ',
        }
    }
}
impl Output<'static> {
    pub fn stdout() -> Self {
        Self::new(OutputDest::Stdout(std::io::stdout()))
    }
    pub fn file(file: File) -> Self {
        Self::new(OutputDest::File(file))
    }
}
impl Output<'_> {
    const DEFAULT_BUF_SIZE: usize = 4096;
    pub fn flush(&mut self) {
        if self.at != 0 {
            match &mut self.output {
                OutputDest::Stdout(stdout) => {
                    stdout.write_all(&self.buf[..self.at]).unwrap();
                    stdout.flush().unwrap();
                }
                OutputDest::File(file) => {
                    file.write_all(&self.buf[..self.at]).unwrap();
                    file.flush().unwrap();
                }
                OutputDest::Buf(buf) => buf.extend_from_slice(&self.buf[..self.at]),
                OutputDest::Delegate(delegate) => {
                    delegate.write_all(&self.buf[..self.at]).unwrap();
                    delegate.flush().unwrap();
                }
            }
            self.at = 0;
        }
    }
    pub fn print<T: Writable>(&mut self, s: T) {
        s.write(self);
    }
    pub fn print_line<T: Writable>(&mut self, s: T) {
        self.print(s);
        self.put(b'\n');
    }
    pub fn put(&mut self, b: u8) {
        self.buf[self.at] = b;
        self.at += 1;
        if self.at == self.buf.len() {
            self.flush();
        }
    }
    pub fn print_per_line<T: Writable>(&mut self, arg: &[T]) {
        self.print_per_line_iter(arg.iter());
    }
    pub fn print_iter<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
        let mut first = true;
        for e in iter {
            if first {
                first = false;
            } else {
                self.put(self.separator);
            }
            e.write(self);
        }
    }
    pub fn print_line_iter<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
        self.print_iter(iter);
        self.put(b'\n');
    }
    pub fn print_per_line_iter<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
        for e in iter {
            e.write(self);
            self.put(b'\n');
        }
    }
    pub fn set_bool_output(&mut self, bool_output: BoolOutput) {
        self.bool_output = bool_output;
    }
    pub fn set_precision(&mut self, precision: usize) {
        self.precision = Some(precision);
    }
    pub fn reset_precision(&mut self) {
        self.precision = None;
    }
    pub fn get_precision(&self) -> Option<usize> {
        self.precision
    }
    pub fn separator(&self) -> u8 {
        self.separator
    }
    pub fn set_separator(&mut self, separator: u8) {
        self.separator = separator;
    }
}
impl Write for Output<'_> {
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
        output.write_all(self.as_bytes()).unwrap();
    }
}
impl Writable for String {
    fn write(&self, output: &mut Output) {
        output.write_all(self.as_bytes()).unwrap();
    }
}
impl Writable for char {
    fn write(&self, output: &mut Output) {
        output.put(*self as u8);
    }
}
impl Writable for u8 {
    fn write(&self, output: &mut Output) {
        output.put(*self);
    }
}
impl<T: Writable> Writable for [T] {
    fn write(&self, output: &mut Output) {
        output.print_iter(self.iter());
    }
}
impl<T: Writable, const N: usize> Writable for [T; N] {
    fn write(&self, output: &mut Output) {
        output.print_iter(self.iter());
    }
}
impl<T: Writable + ?Sized> Writable for &T {
    fn write(&self, output: &mut Output) {
        T::write(self, output)
    }
}
impl<T: Writable> Writable for Vec<T> {
    fn write(&self, output: &mut Output) {
        self.as_slice().write(output);
    }
}
impl Writable for () {
    fn write(&self, _output: &mut Output) {}
}
macro_rules! write_to_string {
    ($($t:ident)+) => {
        $(impl Writable for $t { fn write(& self, output : & mut Output) { self
        .to_string().write(output); } })+
    };
}
write_to_string!(u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
macro_rules! tuple_writable {
    ($name0:ident $($name:ident : $id:tt)*) => {
        impl <$name0 : Writable, $($name : Writable,)*> Writable for ($name0, $($name,)*)
        { fn write(& self, out : & mut Output) { self.0.write(out); $(out.put(out
        .separator); self.$id .write(out);)* } }
    };
}
tuple_writable! {
    T
}
tuple_writable! {
    T U : 1
}
tuple_writable! {
    T U : 1 V : 2
}
tuple_writable! {
    T U : 1 V : 2 X : 3
}
tuple_writable! {
    T U : 1 V : 2 X : 3 Y : 4
}
tuple_writable! {
    T U : 1 V : 2 X : 3 Y : 4 Z : 5
}
tuple_writable! {
    T U : 1 V : 2 X : 3 Y : 4 Z : 5 A : 6
}
tuple_writable! {
    T U : 1 V : 2 X : 3 Y : 4 Z : 5 A : 6 B : 7
}
tuple_writable! {
    T U : 1 V : 2 X : 3 Y : 4 Z : 5 A : 6 B : 7 C : 8
}
impl<T: Writable> Writable for Option<T> {
    fn write(&self, output: &mut Output) {
        match self {
            None => (-1).write(output),
            Some(t) => t.write(output),
        }
    }
}
impl Writable for bool {
    fn write(&self, output: &mut Output) {
        let bool_output = output.bool_output;
        bool_output.output(output, *self)
    }
}
impl<T: Writable> Writable for Reverse<T> {
    fn write(&self, output: &mut Output) {
        self.0.write(output);
    }
}
}
}
pub mod misc {
pub mod lazy_lock {
use std::cell::UnsafeCell;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::sync::Once;
union Data<T, F> {
    value: ManuallyDrop<T>,
    f: ManuallyDrop<F>,
}
pub struct LazyLock<T, F = fn() -> T> {
    once: Once,
    data: UnsafeCell<Data<T, F>>,
}
impl<T, F: FnOnce() -> T> LazyLock<T, F> {
    #[inline]
    pub const fn new(f: F) -> LazyLock<T, F> {
        LazyLock {
            once: Once::new(),
            data: UnsafeCell::new(Data { f: ManuallyDrop::new(f) }),
        }
    }
    #[inline]
    pub fn force(this: &LazyLock<T, F>) -> &T {
        this.once
            .call_once(|| {
                let data = unsafe { &mut *this.data.get() };
                let f = unsafe { ManuallyDrop::take(&mut data.f) };
                let value = f();
                data.value = ManuallyDrop::new(value);
            });
        unsafe { &(*this.data.get()).value }
    }
}
impl<T, F: FnOnce() -> T> Deref for LazyLock<T, F> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        LazyLock::force(self)
    }
}
unsafe impl<T: Sync + Send, F: Send> Sync for LazyLock<T, F> {}
}
pub mod memo {
pub mod memoization_2d {
use crate::algo_lib::collections::md_arr::arr2d::Arr2d;
use crate::algo_lib::misc::recursive_function::Callable2;
pub struct Memoization2d<F, Output>
where
    F: FnMut(&mut dyn Callable2<usize, usize, Output>, usize, usize) -> Output,
{
    f: std::cell::UnsafeCell<F>,
    res: Arr2d<Option<Output>>,
}
impl<F, Output: Clone> Memoization2d<F, Output>
where
    F: FnMut(&mut dyn Callable2<usize, usize, Output>, usize, usize) -> Output,
{
    pub fn new(d1: usize, d2: usize, f: F) -> Self {
        Self {
            f: std::cell::UnsafeCell::new(f),
            res: Arr2d::new(d1, d2, None),
        }
    }
}
impl<F, Output: Clone> Callable2<usize, usize, Output> for Memoization2d<F, Output>
where
    F: FnMut(&mut dyn Callable2<usize, usize, Output>, usize, usize) -> Output,
{
    fn call(&mut self, n: usize, m: usize) -> Output {
        match self.res[(n, m)].as_ref() {
            None => {
                let res = unsafe { (*self.f.get())(self, n, m) };
                self.res[(n, m)] = Some(res.clone());
                res
            }
            Some(res) => res.clone(),
        }
    }
}
}
}
pub mod recursive_function {
use std::marker::PhantomData;
macro_rules! recursive_function {
    ($name:ident, $trait:ident, ($($type:ident $arg:ident,)*)) => {
        pub trait $trait <$($type,)* Output > { fn call(& mut self, $($arg : $type,)*) ->
        Output; } pub struct $name < F, $($type,)* Output > where F : FnMut(& mut dyn
        $trait <$($type,)* Output >, $($type,)*) -> Output, { f : std::cell::UnsafeCell <
        F >, $($arg : PhantomData <$type >,)* phantom_output : PhantomData < Output >, }
        impl < F, $($type,)* Output > $name < F, $($type,)* Output > where F : FnMut(&
        mut dyn $trait <$($type,)* Output >, $($type,)*) -> Output, { pub fn new(f : F)
        -> Self { Self { f : std::cell::UnsafeCell::new(f), $($arg :
        Default::default(),)* phantom_output : Default::default(), } } } impl < F,
        $($type,)* Output > $trait <$($type,)* Output > for $name < F, $($type,)* Output
        > where F : FnMut(& mut dyn $trait <$($type,)* Output >, $($type,)*) -> Output, {
        fn call(& mut self, $($arg : $type,)*) -> Output { unsafe { (* self.f.get())
        (self, $($arg,)*) } } }
    };
}
recursive_function!(RecursiveFunction0, Callable0, ());
recursive_function!(RecursiveFunction, Callable, (Arg arg,));
recursive_function!(RecursiveFunction2, Callable2, (Arg1 arg1, Arg2 arg2,));
recursive_function!(RecursiveFunction3, Callable3, (Arg1 arg1, Arg2 arg2, Arg3 arg3,));
recursive_function!(
    RecursiveFunction4, Callable4, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4,)
);
recursive_function!(
    RecursiveFunction5, Callable5, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5
    arg5,)
);
recursive_function!(
    RecursiveFunction6, Callable6, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5
    arg5, Arg6 arg6,)
);
recursive_function!(
    RecursiveFunction7, Callable7, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5
    arg5, Arg6 arg6, Arg7 arg7,)
);
recursive_function!(
    RecursiveFunction8, Callable8, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5
    arg5, Arg6 arg6, Arg7 arg7, Arg8 arg8,)
);
recursive_function!(
    RecursiveFunction9, Callable9, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5
    arg5, Arg6 arg6, Arg7 arg7, Arg8 arg8, Arg9 arg9,)
);
}
pub mod test_type {
pub enum TestType {
    Single,
    MultiNumber,
    MultiEof,
}
pub enum TaskType {
    Classic,
    Interactive,
}
}
pub mod value {
use std::hash::Hash;
pub trait Value<T>: Copy + Ord + Hash + Default {
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
    ($v:vis $name:ident : $t:ty = $val:expr) => {
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)] $v struct
        $name {} impl $crate ::algo_lib::misc::value::ConstValue <$t > for $name { const
        VAL : $t = $val; }
    };
}
pub trait DynamicValue<T>: Value<T> {
    fn set(t: T);
}
#[macro_export]
macro_rules! dynamic_value {
    ($v:vis $name:ident : $t:ty) => {
        thread_local! { #[allow(non_upper_case_globals)] static $name : std::cell::Cell <
        Option <$t >> = std::cell::Cell::new(None); } #[derive(Copy, Clone, Eq,
        PartialEq, Ord, PartialOrd, Hash, Default)] $v struct $name {} impl $crate
        ::algo_lib::misc::value::DynamicValue <$t > for $name { fn set(t : $t) { $name
        .with(| c | c.set(Some(t))); } } impl $crate ::algo_lib::misc::value::Value <$t >
        for $name { fn val() -> $t { $name .with(| c | c.get().unwrap()) } }
    };
    ($v:vis $name:ident : $t:ty = $val:expr) => {
        dynamic_value!($v $name : $t); <$name as $crate
        ::algo_lib::misc::value::DynamicValue <$t >>::set($val);
    };
}
}
pub mod when {
#[macro_export]
macro_rules! when {
    {$($cond:expr => $then:expr,)*} => {
        match () { $(_ if $cond => $then,)* _ => unreachable!(), }
    };
    {$($cond:expr => $then:expr,)* else $(=>)? $else:expr $(,)?} => {
        match () { $(_ if $cond => $then,)* _ => $else, }
    };
}
}
}
pub mod numbers {
pub mod gcd {
use crate::algo_lib::numbers::num_traits::algebra::{
    IntegerMultiplicationMonoid, IntegerSemiRingWithSub, Zero,
};
use std::mem::swap;
pub fn extended_gcd<T: IntegerSemiRingWithSub + Copy>(a: T, b: T) -> (T, T, T) {
    if a == T::zero() {
        (b, T::zero(), T::one())
    } else {
        let (d, y, mut x) = extended_gcd(b % a, a);
        x -= b / a * y;
        (d, x, y)
    }
}
pub fn gcd<T: Copy + Zero + IntegerMultiplicationMonoid>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        a %= b;
        swap(&mut a, &mut b);
    }
    a
}
pub fn lcm<T: Copy + Zero + IntegerMultiplicationMonoid>(a: T, b: T) -> T {
    (a / gcd(a, b)) * b
}
pub fn remainder<T: IntegerSemiRingWithSub + Copy>(
    a1: T,
    n1: T,
    a2: T,
    n2: T,
) -> Option<T> {
    let (d, m1, m2) = extended_gcd(n1, n2);
    if (a2 - a1) % d != T::zero() {
        return None;
    }
    let m = lcm(n1, n2);
    Some(((a1 * m2) % n1 * n2 + (a2 * m1) % n2 * n1) % m)
}
}
pub mod mod_int {
use crate::algo_lib::collections::fx_hash_map::FxHashMap;
use crate::algo_lib::io::input::{Input, Readable};
use crate::algo_lib::io::output::{Output, Writable};
use crate::algo_lib::misc::value::Value;
use crate::algo_lib::numbers::gcd::extended_gcd;
use crate::algo_lib::numbers::num_traits::algebra::{Field, One, Zero};
use crate::algo_lib::numbers::num_traits::as_index::AsIndex;
use crate::algo_lib::numbers::num_traits::invertible::Invertible;
use crate::{value, when};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};



pub trait BaseModInt<T>: Field + Copy {
    fn from(v: T) -> Self;
    fn module() -> T;
    fn value(&self) -> T;
}
macro_rules! mod_int {
    ($name:ident, $t:ty, $s:ty, $w:ty) => {
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Default)] pub struct $name < V : Value
        <$t >> { n : $t, phantom : PhantomData < V >, } impl < V : Value <$t >> $name < V
        > { unsafe fn unchecked_new(n : $t) -> Self { debug_assert!(n < V::val()); Self {
        n, phantom : Default::default(), } } unsafe fn maybe_subtract_mod(n : $t) -> $t {
        debug_assert!(n < 2 * V::val()); n - (n >= V::val()) as $t * V::val() } } impl <
        V : Value <$t >> $name < V > { pub fn new(n : $t) -> Self { unsafe {
        Self::unchecked_new(n % (V::val())) } } pub fn new_signed(n : $s) -> Self {
        unsafe { Self::unchecked_new(Self::maybe_subtract_mod((n % (V::val() as $s) +
        V::val() as $s) as $t,)) } } pub fn val(& self) -> $t { self.n } } impl < V :
        Value <$t >> $name < V > { pub fn log(& self, alpha : Self) -> $t { let mut base
        = FxHashMap::default(); let mut exp = 0; let mut pow = Self::one(); let mut inv =
        * self; let alpha_inv = alpha.inv().unwrap(); while exp * exp < Self::module() {
        if inv == Self::one() { return exp; } base.insert(inv, exp); exp += 1; pow *=
        alpha; inv *= alpha_inv; } let step = pow; let mut i = 1; loop { if let Some(b) =
        base.get(& pow) { break exp * i + * b; } pow *= step; i += 1; } } } impl < V :
        Value <$t >> $name < V > { pub fn new_wide(n : $w) -> Self { unsafe {
        Self::unchecked_new(Self::maybe_subtract_mod((n % V::val() as $w + V::val() as
        $w) as $t,)) } } } impl < V : Value <$t >> Invertible for $name < V > { type
        Output = Self; fn inv(& self) -> Option < Self > { let (g, x, _) =
        extended_gcd(self.n as $w, V::val() as $w); if g != 1 { None } else {
        Some(Self::new_wide(x)) } } } impl < V : Value <$t >> BaseModInt <$t > for $name
        < V > { fn from(v : $t) -> Self { Self::new(v) } fn module() -> $t { V::val() }
        fn value(& self) -> $t { self.n } } impl < V : Value <$t >> From <$t > for $name
        < V > { fn from(n : $t) -> Self { Self::new(n) } } impl < V : Value <$t >>
        AddAssign for $name < V > { fn add_assign(& mut self, rhs : Self) { self.n =
        unsafe { Self::maybe_subtract_mod(self.n + rhs.n) }; } } impl < V : Value <$t >>
        Add for $name < V > { type Output = Self; fn add(mut self, rhs : Self) ->
        Self::Output { self += rhs; self } } impl < V : Value <$t >> SubAssign for $name
        < V > { fn sub_assign(& mut self, rhs : Self) { self.n = unsafe {
        Self::maybe_subtract_mod(self.n + V::val() - rhs.n) }; } } impl < V : Value <$t
        >> Sub for $name < V > { type Output = Self; fn sub(mut self, rhs : Self) ->
        Self::Output { self -= rhs; self } } impl < V : Value <$t >> MulAssign for $name
        < V > { fn mul_assign(& mut self, rhs : Self) { self.n = ((self.n as $w) * (rhs.n
        as $w) % (V::val() as $w)) as $t; } } impl < V : Value <$t >> Mul for $name < V >
        { type Output = Self; fn mul(mut self, rhs : Self) -> Self::Output { self *= rhs;
        self } } impl < V : Value <$t >> DivAssign for $name < V > {
        #[allow(clippy::suspicious_op_assign_impl)] fn div_assign(& mut self, rhs : Self)
        { * self *= rhs.inv().unwrap(); } } impl < V : Value <$t >> Div for $name < V > {
        type Output = Self; fn div(mut self, rhs : Self) -> Self::Output { self /= rhs;
        self } } impl < V : Value <$t >> Neg for $name < V > { type Output = Self; fn
        neg(mut self) -> Self::Output { if self.n != 0 { self.n = V::val() - self.n; }
        self } } impl < V : Value <$t >> Display for $name < V > { fn fmt(& self, f : &
        mut Formatter <'_ >) -> std::fmt::Result { <$t as Display >::fmt(& self.n, f) } }
        impl < V : Value <$t >> Readable for $name < V > { fn read(input : & mut Input)
        -> Self { Self::new(input.read()) } } impl < V : Value <$t >> Writable for $name
        < V > { fn write(& self, output : & mut Output) { self.n.write(output); } } impl
        < V : Value <$t >> Zero for $name < V > { fn zero() -> Self { unsafe {
        Self::unchecked_new(0) } } } impl < V : Value <$t >> One for $name < V > { fn
        one() -> Self { Self::new(1) } } impl < V : Value <$t >> std::fmt::Debug for
        $name < V > { fn fmt(& self, f : & mut Formatter) -> std::fmt::Result { let max =
        100; when! { self.n <= max => write!(f, "{}", self.n), self.n >= V::val() - max
        => write!(f, "-{}", V::val() - self.n), else => { for denominator in 1..max { for
        num in 1..max { if Self::new(num) / Self::new(denominator) == * self { return
        write!(f, "{}/{}", num, denominator); } if - Self::new(num) /
        Self::new(denominator) == * self { return write!(f, "-{}/{}", num, denominator);
        } } } write!(f, "(?? {} ??)", self.n) }, } } } impl < V : Value <$t >> AsIndex
        for $name < V > { fn from_index(idx : usize) -> Self { let v = idx as $w; if v >=
        V::val() as $w { Self::new_wide(v) } else { unsafe { Self::unchecked_new(v as $t)
        } } } fn to_index(self) -> usize { self.n.to_index() } }
    };
}
mod_int!(ModInt, u32, i32, i64);
mod_int!(ModInt64, u64, i64, i128);
value!(pub Val7 : u32 = 1_000_000_007);
pub type ModInt7 = ModInt<Val7>;
value!(pub Val9 : u32 = 1_000_000_009);
pub type ModInt9 = ModInt<Val9>;
value!(pub ValF : u32 = 998_244_353);
pub type ModIntF = ModInt<ValF>;
}
pub mod num_traits {
pub mod algebra {
use crate::algo_lib::numbers::num_traits::invertible::Invertible;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
pub trait Zero {
    fn zero() -> Self;
}
pub trait One {
    fn one() -> Self;
}
pub trait AdditionMonoid: Add<Output = Self> + AddAssign + Zero + Eq + Sized {}
impl<T: Add<Output = Self> + AddAssign + Zero + Eq> AdditionMonoid for T {}
pub trait AdditionMonoidWithSub: AdditionMonoid + Sub<Output = Self> + SubAssign {}
impl<T: AdditionMonoid + Sub<Output = Self> + SubAssign> AdditionMonoidWithSub for T {}
pub trait AdditionGroup: AdditionMonoidWithSub + Neg<Output = Self> {}
impl<T: AdditionMonoidWithSub + Neg<Output = Self>> AdditionGroup for T {}
pub trait MultiplicationMonoid: Mul<Output = Self> + MulAssign + One + Eq + Sized {}
impl<T: Mul<Output = Self> + MulAssign + One + Eq> MultiplicationMonoid for T {}
pub trait IntegerMultiplicationMonoid: MultiplicationMonoid + Div<
        Output = Self,
    > + Rem<Output = Self> + DivAssign + RemAssign {}
impl<
    T: MultiplicationMonoid + Div<Output = Self> + Rem<Output = Self> + DivAssign
        + RemAssign,
> IntegerMultiplicationMonoid for T {}
pub trait MultiplicationGroup: MultiplicationMonoid + Div<
        Output = Self,
    > + DivAssign + Invertible<Output = Self> {}
impl<
    T: MultiplicationMonoid + Div<Output = Self> + DivAssign + Invertible<Output = Self>,
> MultiplicationGroup for T {}
pub trait SemiRing: AdditionMonoid + MultiplicationMonoid {}
impl<T: AdditionMonoid + MultiplicationMonoid> SemiRing for T {}
pub trait SemiRingWithSub: AdditionMonoidWithSub + SemiRing {}
impl<T: AdditionMonoidWithSub + SemiRing> SemiRingWithSub for T {}
pub trait Ring: SemiRing + AdditionGroup {}
impl<T: SemiRing + AdditionGroup> Ring for T {}
pub trait IntegerSemiRing: SemiRing + IntegerMultiplicationMonoid {}
impl<T: SemiRing + IntegerMultiplicationMonoid> IntegerSemiRing for T {}
pub trait IntegerSemiRingWithSub: SemiRingWithSub + IntegerSemiRing {}
impl<T: SemiRingWithSub + IntegerSemiRing> IntegerSemiRingWithSub for T {}
pub trait IntegerRing: IntegerSemiRing + Ring {}
impl<T: IntegerSemiRing + Ring> IntegerRing for T {}
pub trait Field: Ring + MultiplicationGroup {}
impl<T: Ring + MultiplicationGroup> Field for T {}
macro_rules! zero_one_integer_impl {
    ($($t:ident)+) => {
        $(impl Zero for $t { fn zero() -> Self { 0 } } impl One for $t { fn one() -> Self
        { 1 } })+
    };
}
zero_one_integer_impl!(i128 i64 i32 i16 i8 isize u128 u64 u32 u16 u8 usize);
}
pub mod as_index {
pub trait AsIndex {
    fn from_index(idx: usize) -> Self;
    fn to_index(self) -> usize;
}
macro_rules! from_index_impl {
    ($($t:ident)+) => {
        $(impl AsIndex for $t { fn from_index(idx : usize) -> Self { idx as $t } fn
        to_index(self) -> usize { self as usize } })+
    };
}
from_index_impl!(i128 i64 i32 i16 i8 isize u128 u64 u32 u16 u8 usize);
}
pub mod invertible {
pub trait Invertible {
    type Output;
    fn inv(&self) -> Option<Self::Output>;
}
}
}
}
}
