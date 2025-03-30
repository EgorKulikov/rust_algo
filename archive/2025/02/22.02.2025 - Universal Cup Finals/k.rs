// K: Master of Modular Arithmetic
use crate::algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use crate::algo_lib::collections::slice_ext::backward::Back;
use crate::algo_lib::collections::slice_ext::indices::Indices;
use crate::algo_lib::collections::vec_ext::gen_vec::VecGen;
use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::Output;
use crate::algo_lib::misc::test_type::TaskType;
use crate::algo_lib::misc::test_type::TestType;
use crate::algo_lib::numbers::primes::prime::next_prime;
use crate::algo_lib::numbers::unsigned_big_int::UBigInt;
type PreCalc = ();
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);
    if a == b {
        out.print_line("Yes");
        out.print_line(0);
        return;
    }
    if n == 1 || b.copy_count(b[0]) == n {
        out.print_line("No");
        return;
    }
    let mut ans = Vec::new();
    let mut add_op = |i: usize, j: usize, x: i64| {
        ans.push((i + 1, j + 1, x));
    };
    let mut pos = n;
    for i in 0..n {
        if b[i] != 1 {
            if pos == n || b[pos] > b[i] {
                pos = i;
            }
        }
    }
    assert_ne!(pos, n);
    let alpha = if b[pos] % 2 == 0 {
        pos
    } else {
        let mut res = n;
        for i in 0..n {
            if b[pos] != b[i] {
                res = i;
                break;
            }
        }
        assert_ne!(res, n);
        res
    };
    let p = next_prime(100_000_000) as i64;
    let mut val = a[alpha];
    let mut twos = Vec::new();
    for i in 0..n {
        if i != alpha && a[i] > 2 {
            val *= a[i] - 1;
            val %= p;
            add_op(i, alpha, a[i] - 1);
        } else if i != alpha && a[i] == 2 {
            twos.push(i);
        }
    }
    if twos.len() > 1 {
        add_op(twos[1], twos[0], 3);
        let mut t_val = 6;
        for i in twos.indices().take(twos.len() - 1) {
            if t_val == 6 {
                add_op(twos[i], twos[i + 1], 5);
                t_val = 10;
            } else {
                add_op(twos[i], twos[i + 1], 3);
                t_val = 6;
            }
        }
        add_op(twos[Back(0)], alpha, t_val - 1);
        val *= t_val - 1;
        val %= p;
    } else if twos.len() == 1 {
        add_op(alpha, twos[0], p);
        add_op(twos[0], alpha, 2 * p - 1);
        val *= 2 * p - 1;
        val %= p;
    }
    let beta = if alpha == pos { (alpha + 1) % n } else { pos };
    add_op(alpha, beta, p);
    add_op(beta, alpha, 2);
    val *= 2;
    while val != 2 {
        for p in (3..).step_by(2) {
            if val % p != 0 {
                val %= p;
                val *= 2;
                add_op(alpha, beta, p);
                add_op(beta, alpha, 2);
                break;
            }
        }
    }
    let mut res = n;
    for i in 0..n {
        if i != alpha && i != pos && b[i] == b[pos] {
            if res == n {
                for j in 0..n {
                    if j != alpha && (b[j] != b[pos] || j == pos) {
                        res = j;
                        break;
                    }
                }
            }
            assert_ne!(res, n);
            add_op(res, i, b[i]);
        }
    }
    if pos != alpha {
        add_op(pos, alpha, (b[pos] + 1) / 2);
        add_op(alpha, pos, b[pos]);
    } else {
        if b[pos] != 2 {
            let mut res = n;
            for i in 0..n {
                if b[i] != b[pos] {
                    res = i;
                    break;
                }
            }
            assert_ne!(res, n);
            add_op(res, pos, b[pos] / 2);
        }
    }
    for i in 0..n {
        if b[i] != 1 && b[i] != b[pos] {
            assert!(b[i] > b[pos]);
            add_op(pos, i, b[i]);
        }
    }
    out.print_line("Yes");
    out.print_line(ans.len());
    out.print_per_line(&ans);
}
pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
            pub type FxHashMap = HashMap>;
            pub type FxHashSet = HashSet>;
            #[derive(Default)]
            pub struct FxHasher {
                hash: usize,
            }
            static K: LazyLock = LazyLock::new(|| {
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
        pub mod iter_ext {
            pub mod iter_copied {
                use std::iter::{
                    Chain, Copied, Enumerate, Filter, Map, Rev, Skip, StepBy, Sum, Take, Zip,
                };
                pub trait ItersCopied<'a, T: 'a + Copy>: Sized + 'a
                where
                    &'a Self: IntoIterator,
                {
                    fn copy_iter(&'a self) -> Copied<<&'a Self as IntoIterator>::IntoIter> {
                        self.into_iter().copied()
                    }
                    fn copy_enumerate(
                        &'a self,
                    ) -> Enumerate::IntoIter>> {
                self.copy_iter().enumerate()
                }
                    fn copy_rev(&'a self) -> Rev::IntoIter>>
                    where
                    Copied<<&'a Self as IntoIterator>::IntoIter>: DoubleEndedIterator,
                    {
                    self.copy_iter().rev()
                    }
                    fn copy_skip(
                        &'a self,
                        n: usize,
                    ) -> Skip::IntoIter>> {
                self.copy_iter().skip(n)
                }
                    fn copy_take(
                        &'a self,
                        n: usize,
                    ) -> Take::IntoIter>> {
                self.copy_iter().take(n)
                }
                    fn copy_chain(
                        &'a self,
                        chained: &'a V,
                    ) -> Chain<
                        Copied<<&'a Self as IntoIterator>::IntoIter>,
                        Copied<<&'a V as IntoIterator>::IntoIter>,
                    >
                    where
                        &'a V: IntoIterator,
                    {
                        self.copy_iter().chain(chained.into_iter().copied())
                    }
                    fn copy_zip(
                        &'a self,
                        other: &'a V,
                    ) -> Zip<
                        Copied<<&'a Self as IntoIterator>::IntoIter>,
                        Copied<<&'a V as IntoIterator>::IntoIter>,
                    >
                    where
                        &'a V: IntoIterator,
                    {
                        self.copy_iter().zip(other.into_iter().copied())
                    }
                    fn copy_max(&'a self) -> T
                    where
                        T: Ord,
                    {
                        self.copy_iter().max().unwrap()
                    }
                    fn copy_max_by_key(&'a self, f: F) -> T
                    where
                        F: FnMut(&T) -> B,
                        B: Ord,
                    {
                        self.copy_iter().max_by_key(f).unwrap()
                    }
                    fn copy_min(&'a self) -> T
                    where
                        T: Ord,
                    {
                        self.copy_iter().min().unwrap()
                    }
                    fn copy_min_by_key(&'a self, f: F) -> T
                    where
                        F: FnMut(&T) -> B,
                        B: Ord,
                    {
                        self.copy_iter().min_by_key(f).unwrap()
                    }
                    fn copy_sum(&'a self) -> T
                    where
                        T: Sum,
                    {
                        self.copy_iter().sum()
                    }
                    fn copy_map(
                        &'a self,
                        f: F,
                    ) -> Map::IntoIter>, F>
                    where
                    F: FnMut(T) -> U,
                    {
                    self.copy_iter().map(f)
                    }
                    fn copy_all(&'a self, f: impl FnMut(T) -> bool) -> bool {
                        self.copy_iter().all(f)
                    }
                    fn copy_any(&'a self, f: impl FnMut(T) -> bool) -> bool {
                        self.copy_iter().any(f)
                    }
                    fn copy_step_by(
                        &'a self,
                        step: usize,
                    ) -> StepBy::IntoIter>> {
                self.copy_iter().step_by(step)
                }
                    fn copy_filter bool>(
                    &'a self,
                    f: F,
                    ) -> Filter::IntoIter>, F> {
                self.copy_iter().filter(f)
                }
                    fn copy_fold(&'a self, init: Acc, f: F) -> Acc
                    where
                        F: FnMut(Acc, T) -> Acc,
                    {
                        self.copy_iter().fold(init, f)
                    }
                    fn copy_reduce(&'a self, f: F) -> Option
                    where
                        F: FnMut(T, T) -> T,
                    {
                        self.copy_iter().reduce(f)
                    }
                    fn copy_position
                    (&'a self, predicate: P) -> Option
                    where
                        P: FnMut(T) -> bool,
                    {
                        self.copy_iter().position(predicate)
                    }
                    fn copy_find(&'a self, val: T) -> Option
                    where
                        T: PartialEq,
                    {
                        self.copy_iter().position(|x| x == val)
                    }
                    fn copy_count(&'a self, val: T) -> usize
                    where
                        T: PartialEq,
                    {
                        self.copy_iter().filter(|&x| x == val).count()
                    }
                }
                impl<'a, U: 'a, T: 'a + Copy> ItersCopied<'a, T> for U
                where
                    &'a U: IntoIterator,
                {}
            }
        }
        pub mod slice_ext {
            pub mod backward {
                use std::ops::{Index, IndexMut};
                pub struct Back(pub usize);
                impl Index for [T] {
                    type Output = T;
                    fn index(&self, index: Back) -> &Self::Output {
                        &self[self.len() - index.0 - 1]
                    }
                }
                impl IndexMut for [T] {
                    fn index_mut(&mut self, index: Back) -> &mut Self::Output {
                        &mut self[self.len() - index.0 - 1]
                    }
                }
                impl Index for Vec {
                    type Output = T;
                    fn index(&self, index: Back) -> &Self::Output {
                        self.as_slice().index(index)
                    }
                }
                impl IndexMut for Vec {
                    fn index_mut(&mut self, index: Back) -> &mut Self::Output {
                        self.as_mut_slice().index_mut(index)
                    }
                }
            }
            pub mod indices {
                use std::ops::Range;
                pub trait Indices {
                    fn indices(&self) -> Range;
                }
                impl Indices for [T] {
                    fn indices(&self) -> Range {
                        0..self.len()
                    }
                }
            }
        }
        pub mod vec_ext {
            pub mod default {
                pub fn default_vec(len: usize) -> Vec {
                    let mut v = Vec::with_capacity(len);
                    for _ in 0..len {
                        v.push(T::default());
                    }
                    v
                }
            }
            pub mod gen_vec {
                use crate::algo_lib::collections::vec_ext::default::default_vec;
                pub trait VecGen {
                    fn with_gen(n: usize, f: impl FnMut(usize) -> T) -> Vec;
                    fn with_gen_prefix(n: usize, f: impl FnMut(usize, &Self) -> T) -> Vec;
                    fn gen_append(&mut self, n: usize, f: impl FnMut(usize, &Self) -> T);
                    fn with_gen_back(n: usize, f: impl FnMut(usize, &Self) -> T) -> Vec
                    where
                        T: Default;
                }
                impl VecGen for Vec {
                    fn with_gen(n: usize, mut f: impl FnMut(usize) -> T) -> Vec {
                        Self::with_gen_prefix(n, |i, _| f(i))
                    }
                    fn with_gen_prefix(n: usize, f: impl FnMut(usize, &Self) -> T) -> Vec {
                        let mut vec = Vec::with_capacity(n);
                        vec.gen_append(n, f);
                        vec
                    }
                    fn gen_append(&mut self, n: usize, mut f: impl FnMut(usize, &Self) -> T) {
                        self.reserve(n);
                        let len = self.len();
                        for i in 0..n {
                            self.push(f(len + i, self));
                        }
                    }
                    fn with_gen_back(n: usize, mut f: impl FnMut(usize, &Self) -> T) -> Vec
                    where
                        T: Default,
                    {
                        let mut vec = default_vec(n);
                        for i in (0..n).rev() {
                            vec[i] = f(i, &vec);
                        }
                        vec
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
                Delegate(Box),
            }
            pub struct Input {
                input: InputSource,
                buf: Vec,
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
                pub fn get(&mut self) -> Option {
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
                pub fn peek(&mut self) -> Option {
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
                pub fn next_token(&mut self) -> Option> {
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
                pub fn read(&mut self) -> T {
                    T::read(self)
                }
                pub fn read_vec(&mut self, size: usize) -> Vec {
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
            impl Readable for Vec {
                fn read(input: &mut Input) -> Self {
                    let size = input.read();
                    input.read_vec(size)
                }
            }
            impl Readable for [T; SIZE] {
                fn read(input: &mut Input) -> Self {
                    unsafe {
                        let mut res = MaybeUninit::<[T; SIZE]>::uninit();
                        for i in 0..SIZE {
                            let ptr: *mut T = (*res.as_mut_ptr()).as_mut_ptr();
                            ptr.add(i).write(input.read::());
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
                Buf(&'s mut Vec),
                Delegate(Box),
            }
            pub struct Output<'s> {
                output: OutputDest<'s>,
                buf: Vec,
                at: usize,
                bool_output: BoolOutput,
                precision: Option,
                separator: u8,
            }
            impl<'s> Output<'s> {
                pub fn buf(buf: &'s mut Vec) -> Self {
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
                pub fn print(&mut self, s: T) {
                    s.write(self);
                }
                pub fn print_line(&mut self, s: T) {
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
                pub fn print_per_line(&mut self, arg: &[T]) {
                    self.print_per_line_iter(arg.iter());
                }
                pub fn print_iter>(&mut self, iter: I) {
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
                pub fn print_line_iter>(&mut self, iter: I) {
                self.print_iter(iter);
                self.put(b'\n');
                }
                pub fn print_per_line_iter>(&mut self, iter: I) {
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
                pub fn get_precision(&self) -> Option {
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
                fn write(&mut self, buf: &[u8]) -> std::io::Result {
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
            impl Writable for [T] {
                fn write(&self, output: &mut Output) {
                    output.print_iter(self.iter());
                }
            }
            impl Writable for [T; N] {
                fn write(&self, output: &mut Output) {
                    output.print_iter(self.iter());
                }
            }
            impl Writable for &T {
                fn write(&self, output: &mut Output) {
                    T::write(self, output)
                }
            }
            impl Writable for Vec {
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
            impl Writable for Option {
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
            impl Writable for Reverse {
                fn write(&self, output: &mut Output) {
                    self.0.write(output);
                }
            }
        }
    }
    pub mod misc {
        pub mod extensions {
            pub mod replace_with {
                use std::{mem, ptr};
                pub trait ReplaceWith Self>: Sized {
                fn replace_with(&mut self, f: F) {
                    unsafe {
                        let old = ptr::read(self);
                        let new = on_unwind(move || f(old), || std::process::abort());
                        ptr::write(self, new);
                    }
                }
            }
            impl Self> ReplaceWith for T {}
        pub fn on_unwind T, T, P: FnOnce()>(f: F, p: P) -> T {
    let x = OnDrop(mem::ManuallyDrop::new(p));
    let t = f();
    let mut x = mem::ManuallyDrop::new(x);
    unsafe { mem::ManuallyDrop::drop(&mut x.0) };
    t
    }
        struct OnDrop(mem::ManuallyDrop);
        impl Drop for OnDrop {
            #[inline(always)]
            fn drop(&mut self) {
                (unsafe { ptr::read(&*self.0) })();
            }
        }
    }
}
pub mod lazy_lock {
    use std::cell::UnsafeCell;
    use std::mem::ManuallyDrop;
    use std::ops::Deref;
    use std::sync::Once;
    union Data {
        value: ManuallyDrop,
        f: ManuallyDrop,
    }
    pub struct LazyLock T> {
once: Once,
data: UnsafeCell>,
}
    impl T> LazyLock {
    #[inline]
    pub const fn new(f: F) -> LazyLock {
        LazyLock {
            once: Once::new(),
            data: UnsafeCell::new(Data { f: ManuallyDrop::new(f) }),
        }
    }
    #[inline]
    pub fn force(this: &LazyLock) -> &T {
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
impl T> Deref for LazyLock {
type Target = T;
#[inline]
fn deref(&self) -> &T {
    LazyLock::force(self)
}
}
unsafe impl Sync for LazyLock {}
}
pub mod random {
    use crate::algo_lib::collections::slice_ext::indices::Indices;
    use crate::algo_lib::numbers::num_traits::algebra::IntegerSemiRingWithSub;
    use crate::algo_lib::numbers::num_traits::ord::MinMax;
    use crate::algo_lib::numbers::num_traits::primitive::Primitive;
    use std::cell::RefCell;
    use std::ops::{RangeBounds, Rem};
    use std::time::SystemTime;
    pub trait RandomTrait {
        fn gen_impl(&mut self) -> u64;
        fn gen_int(&mut self) -> T
        where
            u64: Primitive,
        {
            self.gen_impl().to()
        }
        fn gen_u128(&mut self) -> u128 {
            (self.gen_impl() as u128) << 64 | self.gen_impl() as u128
        }
        fn gen_i128(&mut self) -> i128 {
            self.gen_u128() as i128
        }
        fn gen_bool(&mut self) -> bool {
            (self.gen_impl() & 1) == 1
        }
        fn gen_bound + Primitive>(&mut self, n: T) -> T
        where
        u64: Primitive,
        {
        (self.gen_impl() % n.to()).to()
        }
        fn gen_range + MinMax>(
        &mut self,
        range: impl RangeBounds,
        ) -> T
        where
        u64: Primitive,
        {
        let f = match range.start_bound() {
        std::ops::Bound::Included(&s) => s,
        std::ops::Bound::Excluded(&s) => s + T::one(),
        std::ops::Bound::Unbounded => T::min_val(),
        };
        let t = match range.end_bound() {
        std::ops::Bound::Included(&e) => e,
        std::ops::Bound::Excluded(&e) => e - T::one(),
        std::ops::Bound::Unbounded => T::max_val(),
        };
        if f == T::min_val() && t == T::max_val() {
        self.gen_int()
        } else {
        f + self.gen_bound(t - f + T::one())
        }
        }
    }
    const NN: usize = 312;
    const MM: usize = 156;
    const MATRIX_A: u64 = 0xB5026F5AA96619E9;
    const UM: u64 = 0xFFFFFFFF80000000;
    const LM: u64 = 0x7FFFFFFF;
    const F: u64 = 6364136223846793005;
    const MAG01: [u64; 2] = [0, MATRIX_A];
    pub struct Random {
        mt: [u64; NN],
        index: usize,
    }
    impl Random {
        pub fn new() -> Self {
            Self::new_with_seed(
                (SystemTime::UNIX_EPOCH.elapsed().unwrap().as_nanos() & 0xFFFFFFFFFFFFFFFF)
                    as u64,
            )
        }
        pub fn new_with_seed(seed: u64) -> Self {
            let mut res = Self { mt: [0u64; NN], index: NN };
            res.mt[0] = seed;
            for i in 1..NN {
                res.mt[i] = F
                    .wrapping_mul(res.mt[i - 1] ^ (res.mt[i - 1] >> 62))
                    .wrapping_add(i as u64);
            }
            res
        }
    }
    impl Default for Random {
        fn default() -> Self {
            Self::new()
        }
    }
    impl RandomTrait for Random {
        fn gen_impl(&mut self) -> u64 {
            if self.index == NN {
                for i in 0..(NN - MM) {
                    let x = (self.mt[i] & UM) | (self.mt[i + 1] & LM);
                    self.mt[i] = self.mt[i + MM] ^ (x >> 1) ^ MAG01[(x & 1) as usize];
                }
                for i in (NN - MM)..(NN - 1) {
                    let x = (self.mt[i] & UM) | (self.mt[i + 1] & LM);
                    self.mt[i] = self.mt[i + MM - NN] ^ (x >> 1) ^ MAG01[(x & 1) as usize];
                }
                let x = (self.mt[NN - 1] & UM) | (self.mt[0] & LM);
                self.mt[NN - 1] = self.mt[MM - 1] ^ (x >> 1) ^ MAG01[(x & 1) as usize];
                self.index = 0;
            }
            let mut x = self.mt[self.index];
            self.index += 1;
            x ^= (x >> 29) & 0x5555555555555555;
            x ^= (x << 17) & 0x71D67FFFEDA60000;
            x ^= (x << 37) & 0xFFF7EEE000000000;
            x ^= x >> 43;
            x
        }
    }
    thread_local! {
    static RANDOM : RefCell < Random > = RefCell::new(Random::new());
}
    pub struct StaticRandom;
    impl RandomTrait for StaticRandom {
        fn gen_impl(&mut self) -> u64 {
            RANDOM.with(|r| r.borrow_mut().gen_impl())
        }
    }
    pub trait Shuffle {
        fn shuffle(&mut self);
    }
    impl Shuffle for [T] {
        fn shuffle(&mut self) {
            for i in self.indices() {
                let at = StaticRandom.gen_bound(i + 1);
                self.swap(i, at);
            }
        }
    }
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
pub mod transparent_wrapper {
    #[macro_export]
    macro_rules! transparent_wrapper {
    ($name:ident $(<$($par:ident $(,)?)+>)? = $t:ty $(, derive $($d:ty $(,)?)+)?) => {
        $(#[derive($($d,)+)])? pub struct $name $(<$($par,)+>)? ($t); impl
        $(<$($par,)+>)? Deref for $name $(<$($par,)+>)? { type Target = $t; fn deref(&
        self) -> & Self::Target { & self.0 } } impl $(<$($par,)+>)? DerefMut for $name
        $(<$($par,)+>)? { fn deref_mut(& mut self) -> & mut Self::Target { & mut self.0 }
        }
    };
}
}
pub mod value {
    use std::hash::Hash;
    pub trait Value: Copy + Ord + Hash + Default {
        fn val() -> T;
    }
    pub trait ConstValue: Value {
        const VAL: T;
    }
    impl> Value for V {
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
pub trait DynamicValue: Value {
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
        pub fn extended_gcd(a: T, b: T) -> (T, T, T) {
            if a == T::zero() {
                (b, T::zero(), T::one())
            } else {
                let (d, y, mut x) = extended_gcd(b % a, a);
                x -= b / a * y;
                (d, x, y)
            }
        }
        pub fn gcd(mut a: T, mut b: T) -> T {
            while b != T::zero() {
                a %= b;
                swap(&mut a, &mut b);
            }
            a
        }
        pub fn lcm(a: T, b: T) -> T {
            (a / gcd(a, b)) * b
        }
        pub fn remainder(
            a1: T,
            n1: T,
            a2: T,
            n2: T,
        ) -> Option {
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



        pub trait BaseModInt: Field + Copy {
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
        pub type ModInt7 = ModInt;
        value!(pub Val9 : u32 = 1_000_000_009);
        pub type ModInt9 = ModInt;
        value!(pub ValF : u32 = 998_244_353);
        pub type ModIntF = ModInt;
        pub mod convolution {
            use crate::algo_lib::collections::vec_ext::gen_vec::VecGen;
            use crate::algo_lib::misc::value::Value;
            use crate::algo_lib::numbers::gcd::remainder;
            use crate::algo_lib::numbers::mod_int::prime_fft::PrimeFFT;
            use crate::algo_lib::numbers::mod_int::BaseModInt;
            use crate::algo_lib::numbers::mod_int::ModInt;
            use crate::value;
            pub fn convolution(a: &[i32], b: &[i32]) -> Vec {
                value!(Module1 : u32 = 998244353);
                value!(Module2 : u32 = 985661441);
                value!(Module3 : u32 = 975175681);
                type Mod1 = ModInt;
                type Mod2 = ModInt;
                type Mod3 = ModInt;
                let c1 = PrimeFFT::::new()
                    .multiply(
                        &a.iter().map(|&x| Mod1::new_wide(x as i64)).collect::>(),
                        &b.iter().map(|&x| Mod1::new_wide(x as i64)).collect::>(),
                    );
                let c2 = PrimeFFT::::new()
                    .multiply(
                        &a.iter().map(|&x| Mod2::new_wide(x as i64)).collect::>(),
                        &b.iter().map(|&x| Mod2::new_wide(x as i64)).collect::>(),
                    );
                let c3 = PrimeFFT::::new()
                    .multiply(
                        &a.iter().map(|&x| Mod3::new_wide(x as i64)).collect::>(),
                        &b.iter().map(|&x| Mod3::new_wide(x as i64)).collect::>(),
                    );
                let mod12 = (Module1::val() as i64 * Module2::val() as i64) as i128;
                let mod123 = mod12 * Module3::val() as i128;
                Vec::with_gen_prefix(
                    c1.len(),
                    |i, _| {
                        let x1 = c1[i].value();
                        let x2 = c2[i].value();
                        let x3 = c3[i].value();
                        let x12 = remainder(
                            x1 as i64,
                            Module1::val() as i64,
                            x2 as i64,
                            Module2::val() as i64,
                        )
                            .unwrap();
                        let mut x123 = remainder(
                            x12 as i128,
                            mod12,
                            x3 as i128,
                            Module3::val() as i128,
                        )
                            .unwrap();
                        if x123 < 0 {
                            x123 += mod123;
                        }
                        if x123 >= mod123 / 2 {
                            x123 -= mod123;
                        }
                        x123
                    },
                )
            }
        }
        pub mod prime_fft {
            use crate::algo_lib::numbers::mod_int::BaseModInt;
            use crate::algo_lib::numbers::number_ext::Power;
            pub struct PrimeFFT> {
        root: M,
        reverse_root: M,
        root_power: u32,
        aa: Vec,
        bb: Vec,
        }
            impl> Default for PrimeFFT {
            fn default() -> Self {
                Self::new()
            }
        }
        impl> PrimeFFT {
        pub fn new() -> Self {
            let mut exp = 0;
            let mut root_power = 1;
            while (M::module() - 1) % (2 * root_power) == 0 {
                exp = root_power;
                root_power += root_power;
            }
            let mut i = M::from(2);
            let rem = (M::module() - 1) / root_power;
            loop {
                let j = i.power(rem);
                if j.power(exp) != M::one() && j.power(root_power) == M::one() {
                    break Self {
                        root: j,
                        reverse_root: j.inv().unwrap(),
                        root_power,
                        aa: Vec::new(),
                        bb: Vec::new(),
                    };
                }
                i += M::one();
            }
        }
        pub fn multiply_res(&mut self, a: &[M], b: &[M], res: &mut Vec) {
            if a.is_empty() || b.is_empty() {
                res.fill(M::zero());
                return;
            }
            let res_len = a.len() + b.len() - 1;
            if res.len() < res_len {
                res.resize(res_len, M::zero());
            }
            self.multiply_fix_len(a, b, res);
        }
        pub fn multiply_fix_len(&mut self, a: &[M], b: &[M], res: &mut [M]) {
            let res_len = res.len();
            if a.len().min(b.len()) <= Self::BORDER_LEN {
                res.fill(M::zero());
                for (i, f) in a.iter().enumerate() {
                    for (j, s) in b.iter().enumerate() {
                        if i + j < res.len() {
                            res[i + j] += (*f) * (*s);
                        } else {
                            break;
                        }
                    }
                }
                return;
            }
            let mut size = 1;
            while size < res_len {
                size *= 2;
            }
            if self.root_power < size as u32 {
                panic!("unsuitable modulo");
            }
            copy(&mut self.aa, a, size);
            Self::fft(&mut self.aa[..size], false, self.root, self.root_power, size);
            if a == b {
                for i in self.aa[..size].iter_mut() {
                    *i *= *i;
                }
            } else {
                copy(&mut self.bb, b, size);
                Self::fft(&mut self.bb[..size], false, self.root, self.root_power, size);
                for (i, j) in self.aa[..size].iter_mut().zip(self.bb[..size].iter()) {
                    *i *= *j;
                }
            }
            Self::fft(&mut self.aa[..size], true, self.reverse_root, self.root_power, size);
            res.copy_from_slice(&self.aa[..res_len]);
        }
        pub fn multiply(&mut self, a: &[M], b: &[M]) -> Vec {
            if a.is_empty() || b.is_empty() {
                Vec::new()
            } else {
                let mut res = vec![M::zero(); a.len() + b.len() - 1];
                self.multiply_res(a, b, &mut res);
                res
            }
        }
        pub fn power(&mut self, a: &[M], exp: usize) -> Vec {
            let mut res = Vec::new();
            let mut temp = Vec::new();
            self.power_impl(a, exp, &mut res, &mut temp);
            res
        }
        fn power_impl(&mut self, a: &[M], exp: usize, res: &mut Vec, temp: &mut Vec) {
            if exp == 0 {
                res.push(M::one());
                return;
            }
            if exp % 2 == 0 {
                self.power_impl(a, exp / 2, temp, res);
                self.multiply_res(temp, temp, res);
            } else {
                self.power_impl(a, exp - 1, temp, res);
                self.multiply_res(temp, a, res);
            }
        }
        const BORDER_LEN: usize = 60;
        fn fft(a: &mut [M], invert: bool, root: M, root_power: u32, size_t: usize) {
            let mut j = 0usize;
            for i in 1..a.len() {
                let mut bit = a.len() >> 1;
                while j >= bit {
                    j -= bit;
                    bit >>= 1;
                }
                j += bit;
                if i < j {
                    a.swap(i, j);
                }
            }
            let mut len = 2;
            let mut len_t = 2;
            while len <= a.len() {
                let mut w_len = root;
                let mut i = len_t;
                while i < root_power {
                    w_len *= w_len;
                    i += i;
                }
                let half = len >> 1;
                for i in (0..a.len()).step_by(len) {
                    let mut w = M::one();
                    for j in 0..half {
                        let u = a[i + j];
                        let v = a[i + j + half] * w;
                        a[i + j] = u + v;
                        a[i + j + half] = u - v;
                        w *= w_len;
                    }
                }
                len <<= 1;
                len_t += len_t;
            }
            if invert {
                let inv_size = M::from(size_t as u32).inv().unwrap();
                for i in a {
                    *i *= inv_size;
                }
            }
        }
    }
    fn copy>(aa: &mut Vec, a: &[M], size: usize) {
if aa.len() < size {
let was_len = aa.len();
aa[..was_len.min(a.len())].copy_from_slice(&a[..was_len.min(a.len())]);
aa[was_len.min(a.len())..].fill(M::zero());
aa.reserve(size - aa.len());
aa.extend_from_slice(&a[was_len.min(a.len())..]);
aa.resize(size, M::zero());
} else {
aa[..a.len()].copy_from_slice(a);
aa[a.len()..size].fill(M::zero());
}
}
}
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
        pub trait AdditionMonoid: Add + AddAssign + Zero + Eq + Sized {}
        impl + AddAssign + Zero + Eq> AdditionMonoid for T {}
    pub trait AdditionMonoidWithSub: AdditionMonoid + Sub + SubAssign {}
    impl + SubAssign> AdditionMonoidWithSub for T {}
pub trait AdditionGroup: AdditionMonoidWithSub + Neg {}
impl> AdditionGroup for T {}
pub trait MultiplicationMonoid: Mul + MulAssign + One + Eq + Sized {}
impl + MulAssign + One + Eq> MultiplicationMonoid for T {}
pub trait IntegerMultiplicationMonoid: MultiplicationMonoid + Div<
    Output = Self,
> + Rem + DivAssign + RemAssign {}
impl<
    T: MultiplicationMonoid + Div + Rem + DivAssign
    + RemAssign,
> IntegerMultiplicationMonoid for T {}
pub trait MultiplicationGroup: MultiplicationMonoid + Div<
    Output = Self,
> + DivAssign + Invertible {}
impl<
    T: MultiplicationMonoid + Div + DivAssign + Invertible,
> MultiplicationGroup for T {}
pub trait SemiRing: AdditionMonoid + MultiplicationMonoid {}
impl SemiRing for T {}
pub trait SemiRingWithSub: AdditionMonoidWithSub + SemiRing {}
impl SemiRingWithSub for T {}
pub trait Ring: SemiRing + AdditionGroup {}
impl Ring for T {}
pub trait IntegerSemiRing: SemiRing + IntegerMultiplicationMonoid {}
impl IntegerSemiRing for T {}
pub trait IntegerSemiRingWithSub: SemiRingWithSub + IntegerSemiRing {}
impl IntegerSemiRingWithSub for T {}
pub trait IntegerRing: IntegerSemiRing + Ring {}
impl IntegerRing for T {}
pub trait Field: Ring + MultiplicationGroup {}
impl Field for T {}
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
        fn inv(&self) -> Option;
    }
}
pub mod ord {
    pub trait MinMax: PartialOrd {
        fn min_val() -> Self;
        fn max_val() -> Self;
    }
    macro_rules! min_max_integer_impl {
    ($($t:ident)+) => {
        $(impl MinMax for $t { fn min_val() -> Self { $t ::MIN } fn max_val() -> Self {
        $t ::MAX } })+
    };
}
    min_max_integer_impl!(i128 i64 i32 i16 i8 isize u128 u64 u32 u16 u8 usize);
}
pub mod primitive {
    pub trait Primitive: Copy {
        fn to(self) -> T;
        fn from(t: T) -> Self;
    }
    macro_rules! primitive_one {
    ($t:ident, $($u:ident)+) => {
        $(impl Primitive <$u > for $t { fn to(self) -> $u { self as $u } fn from(t : $u)
        -> Self { t as $t } })+
    };
}
    macro_rules! primitive {
    ($($t:ident)+) => {
        $(primitive_one!($t, u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);)+
    };
}
    primitive!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
}
}
pub mod number_ext {
    use crate::algo_lib::numbers::num_traits::algebra::{
        IntegerSemiRing, MultiplicationMonoid,
    };
    use crate::algo_lib::numbers::num_traits::as_index::AsIndex;
    use std::ops::Mul;
    pub trait Power {
        #[must_use]
        fn power(&self, exp: T) -> Self;
    }
    impl Power for S {
        fn power(&self, exp: T) -> Self {
            if exp == T::zero() {
                S::one()
            } else {
                let mut res = self.power(exp / (T::one() + T::one()));
                res *= res;
                if exp % (T::one() + T::one()) == T::one() {
                    res *= *self;
                }
                res
            }
        }
    }
    pub fn num_digs(mut copy: S) -> usize {
        let ten = S::from_index(10);
        let mut res = 0;
        while copy != S::zero() {
            copy /= ten;
            res += 1;
        }
        res
    }
    pub fn sum_digs(mut copy: S) -> S {
        let ten = S::from_index(10);
        let mut res = S::zero();
        while copy != S::zero() {
            res += copy % ten;
            copy /= ten;
        }
        res
    }
    pub fn digits(
        mut copy: S,
    ) -> impl Iterator {
        let ten = S::from_index(10);
        std::iter::from_fn(move || {
            if copy == S::zero() {
                None
            } else {
                let res = copy % ten;
                copy /= ten;
                Some(res)
            }
        })
    }
    pub trait Square {
        fn square(self) -> Self;
    }
    impl + Copy> Square for T {
    fn square(self) -> Self {
        self * self
    }
}
}
pub mod primes {
    pub mod prime {
        use crate::algo_lib::misc::random::{RandomTrait, StaticRandom};
        use crate::algo_lib::numbers::gcd::gcd;
        use crate::algo_lib::numbers::mod_int::ModInt64;
        use crate::algo_lib::numbers::num_traits::algebra::{One, Zero};
        use crate::algo_lib::numbers::num_traits::primitive::Primitive;
        use crate::algo_lib::numbers::number_ext::Power;
        use crate::{dynamic_value, when};
        pub fn is_prime(n: impl Primitive) -> bool {
            let n = n.to();
            if n <= 1 {
                return false;
            }
            let mut s = 0;
            let mut d = n - 1;
            while d % 2 == 0 {
                s += 1;
                d >>= 1;
            }
            if s == 0 {
                return n == 2;
            }
            dynamic_value!(IsPrimeModule : u64 = n);
            type Mod = ModInt64;
            for _ in 0..20 {
                let a = Mod::new(StaticRandom.gen_bound(n));
                if a == Mod::zero() {
                    continue;
                }
                if a.power(d) == Mod::one() {
                    continue;
                }
                let mut dd = d;
                let mut good = true;
                for _ in 0..s {
                    if a.power(dd) == -Mod::one() {
                        good = false;
                        break;
                    }
                    dd *= 2;
                }
                if good {
                    return false;
                }
            }
            true
        }
        pub fn next_prime(mut n: u64) -> u64 {
            if n <= 2 {
                return 2;
            }
            n += 1 - (n & 1);
            while !is_prime(n) {
                n += 2;
            }
            n
        }
        fn brent(n: u64, x0: u64, c: u64) -> u64 {
            dynamic_value!(ModVal : u64 = n);
            type Mod = ModInt64;
            let mut x = Mod::new(x0);
            let c = Mod::new(c);
            let mut g = 1;
            let mut q = Mod::one();
            let mut xs = Mod::zero();
            let mut y = Mod::zero();
            let m = 128i64;
            let mut l = 1;
            while g == 1 {
                y = x;
                for _ in 1..l {
                    x = x * x + c;
                }
                let mut k = 0;
                while k < l && g == 1 {
                    xs = x;
                    for _ in 0..m.min(l - k) {
                        x = x * x + c;
                        q *= y - x;
                    }
                    g = gcd(q.val(), n);
                    k += m;
                }
                l *= 2;
            }
            if g == n {
                loop {
                    xs = xs * xs + c;
                    g = gcd((xs - y).val(), n);
                    if g != 1 {
                        break;
                    }
                }
            }
            g
        }
        pub fn find_divisor(n: u64) -> u64 {
            when! {
        n == 1 => 1, n % 2 == 0 => 2, is_prime(n) => n, else => { loop { let res =
        brent(n, StaticRandom.gen_range(2..n), StaticRandom.gen_range(1..n),); if res !=
        n { return res; } } },
    }
        }
    }
}
pub mod unsigned_big_int {
    use crate::algo_lib::io::input::{Input, Readable};
    use crate::algo_lib::io::output::{Output, Writable};
    use crate::algo_lib::misc::extensions::replace_with::ReplaceWith;
    use crate::algo_lib::numbers::mod_int::convolution::convolution;
    use crate::algo_lib::numbers::num_traits::algebra::{One, Zero};
    use crate::algo_lib::string::str::StrReader;
    use std::cmp::Ordering;
    use std::fmt::{Debug, Display, Formatter};
    use std::ops::{Add, AddAssign, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};
    const DIGITS: usize = 9;
    const BASE: i32 = 10i32.pow(DIGITS as u32);
    #[derive(Eq, PartialEq, Clone)]
    pub struct UBigInt {
        z: Vec,
    }
    impl From<&[u8]> for UBigInt {
        fn from(value: &[u8]) -> Self {
            let mut at = value.len();
            #[allow(clippy::manual_div_ceil)]
            let mut res = Vec::with_capacity((at + DIGITS - 1) / DIGITS);
            while at > 0 {
                let mut cur = 0;
                let start = at.saturating_sub(DIGITS);
                for &c in &value[start..at] {
                    assert!(c.is_ascii_digit());
                    cur *= 10;
                    cur += (c - b'0') as i32;
                }
                res.push(cur);
                at = start;
            }
            Self { z: res }
        }
    }
    impl From for UBigInt {
        fn from(mut v: i32) -> Self {
            let mut z = Vec::new();
            while v > 0 {
                z.push(v % BASE);
                v /= BASE;
            }
            Self { z }
        }
    }
    impl Zero for UBigInt {
        fn zero() -> Self {
            Self::from(0)
        }
    }
    impl One for UBigInt {
        fn one() -> Self {
            Self::from(1)
        }
    }
    impl AddAssign for UBigInt {
        fn add_assign(&mut self, rhs: Self) {
            self.add_assign(&rhs);
        }
    }
    impl<'a> AddAssign<&'a Self> for UBigInt {
        fn add_assign(&mut self, rhs: &'a Self) {
            let mut carry = 0;
            let mut at = 0;
            if rhs.z.len() > self.z.len() {
                self.z.reserve(rhs.z.len() - self.z.len() + 1);
            }
            while at < rhs.z.len() || carry != 0 {
                if at == self.z.len() {
                    self.z.push(0);
                }
                self.z[at] += if at < rhs.z.len() { rhs.z[at] } else { 0 } + carry;
                if self.z[at] >= BASE {
                    self.z[at] -= BASE;
                    carry = 1;
                } else {
                    carry = 0;
                }
                at += 1;
            }
        }
    }
    impl Add for UBigInt {
        type Output = Self;
        fn add(mut self, rhs: Self) -> Self::Output {
            self += rhs;
            self
        }
    }
    impl<'a> Add<&'a Self> for UBigInt {
        type Output = Self;
        fn add(mut self, rhs: &'a Self) -> Self::Output {
            self += rhs;
            self
        }
    }
    impl<'a> SubAssign<&'a Self> for UBigInt {
        fn sub_assign(&mut self, rhs: &'a Self) {
            assert!(self.z.len() >= rhs.z.len());
            let mut carry = 0;
            for (i, j) in self.z.iter_mut().zip(rhs.z.iter()) {
                if *i >= j + carry {
                    *i -= j + carry;
                    carry = 0;
                } else {
                    *i += BASE - (j + carry);
                    carry = 1;
                }
            }
            if carry == 1 {
                let mut at = rhs.z.len();
                loop {
                    if self.z[at] == 0 {
                        self.z[at] = BASE - 1;
                    } else {
                        self.z[at] -= 1;
                        break;
                    }
                    at += 1;
                }
            }
            while !self.z.is_empty() && *self.z.last().unwrap() == 0 {
                self.z.pop();
            }
        }
    }
    impl SubAssign for UBigInt {
        fn sub_assign(&mut self, rhs: Self) {
            self.sub_assign(&rhs);
        }
    }
    impl<'a> Sub<&'a Self> for UBigInt {
        type Output = Self;
        fn sub(mut self, rhs: &'a Self) -> Self::Output {
            self -= rhs;
            self
        }
    }
    impl Sub for UBigInt {
        type Output = Self;
        fn sub(mut self, rhs: Self) -> Self::Output {
            self -= rhs;
            self
        }
    }
    impl MulAssign for UBigInt {
        fn mul_assign(&mut self, rhs: i32) {
            if rhs == 0 {
                *self = Self::zero();
                return;
            }
            let rhs = rhs as i64;
            let mut carry = 0;
            let base = BASE as i64;
            for i in self.z.iter_mut() {
                let val: i64 = (*i as i64) * rhs + carry;
                *i = (val % base) as i32;
                carry = val / base;
            }
            while carry > 0 {
                self.z.push((carry % base) as i32);
                carry /= base;
            }
        }
    }
    impl<'a> Mul<&'a UBigInt> for UBigInt {
        type Output = Self;
        fn mul(self, rhs: &'a Self) -> Self::Output {
            let c = convolution(&self.z, &rhs.z);
            let mut carry = 0;
            let mut res = Vec::new();
            for i in c {
                carry += i;
                let last = carry % BASE as i128;
                res.push(last as i32);
                carry /= BASE as i128;
            }
            while carry > 0 {
                res.push((carry % BASE as i128) as i32);
                carry /= BASE as i128;
            }
            while let Some(d) = res.last() {
                if *d == 0 {
                    res.pop();
                } else {
                    break;
                }
            }
            Self { z: res }
        }
    }
    impl Mul for UBigInt {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            self * &rhs
        }
    }
    impl MulAssign for UBigInt {
        fn mul_assign(&mut self, rhs: Self) {
            self.replace_with(|s| s * rhs);
        }
    }
    impl DivAssign for UBigInt {
        fn div_assign(&mut self, rhs: i32) {
            let rhs = rhs as i64;
            let mut carry = 0;
            let base = BASE as i64;
            for i in self.z.iter_mut().rev() {
                let val = carry + *i as i64;
                *i = (val / rhs) as i32;
                carry = (val % rhs) * base;
            }
            while let Some(d) = self.z.last() {
                if *d == 0 {
                    self.z.pop();
                } else {
                    break;
                }
            }
        }
    }
    impl Rem for &UBigInt {
        type Output = i32;
        fn rem(self, rhs: i32) -> Self::Output {
            let mut res = 0i64;
            let base = BASE as i64;
            for &i in self.z.iter().rev() {
                res *= base;
                res += i as i64;
                res %= rhs as i64;
            }
            res as i32
        }
    }
    impl Writable for UBigInt {
        fn write(&self, output: &mut Output) {
            if let Some(tail) = self.z.last() {
                tail.write(output);
                for &i in self.z.iter().rev().skip(1) {
                    format!("{:09}", i).write(output);
                }
            } else {
                0u32.write(output);
            }
        }
    }
    impl Display for UBigInt {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            if let Some(tail) = self.z.last() {
                write!(f, "{}", tail)?;
                for &i in self.z.iter().rev().skip(1) {
                    write!(f, "{:09}", i)?;
                }
            } else {
                write!(f, "0")?;
            }
            Ok(())
        }
    }
    impl Debug for UBigInt {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self)
        }
    }
    impl PartialOrd for UBigInt {
        fn partial_cmp(&self, other: &Self) -> Option {
            Some(self.cmp(other))
        }
    }
    impl Ord for UBigInt {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.z.len() != other.z.len() {
                return self.z.len().cmp(&other.z.len());
            }
            for (i, j) in self.z.iter().rev().zip(other.z.iter().rev()) {
                if i != j {
                    return i.cmp(j);
                }
            }
            Ordering::Equal
        }
    }
    impl Readable for UBigInt {
        fn read(input: &mut Input) -> Self {
            input.read_str().as_slice().into()
        }
    }
}
}
pub mod string {
    pub mod str {
        use crate::algo_lib::io::input::{Input, Readable};
        use crate::algo_lib::io::output::{Output, Writable};
        use crate::transparent_wrapper;
        use std::fmt::Display;
        use std::io::Write;
        use std::ops::{AddAssign, Deref, DerefMut};
        use std::str::from_utf8_unchecked;
        use std::vec::IntoIter;
        transparent_wrapper!(
    Str = Vec < u8 >, derive Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Default
);
        impl Str {
            pub fn new() -> Self {
                Self(Vec::new())
            }
            pub fn unwrap(self) -> Vec {
                self.0
            }
        }
        impl From> for Str {
        fn from(v: Vec) -> Self {
            Self(v)
        }
    }
    impl From<&[u8]> for Str {
        fn from(v: &[u8]) -> Self {
            Self(v.to_vec())
        }
    }
    impl From<&[u8; N]> for Str {
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
        type IntoIter = IntoIter;
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
    impl FromIterator for Str {
        fn from_iter>(iter: T) -> Self {
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
        fn read_str_vec(&mut self, n: usize) -> Vec;
        fn read_line(&mut self) -> Str;
        fn read_line_vec(&mut self, n: usize) -> Vec;
        fn read_lines(&mut self) -> Vec;
    }
    impl StrReader for Input {
        fn read_str(&mut self) -> Str {
            self.read()
        }
        fn read_str_vec(&mut self, n: usize) -> Vec {
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
        fn read_line_vec(&mut self, n: usize) -> Vec {
            let mut res = Vec::with_capacity(n);
            for _ in 0..n {
                res.push(self.read_line());
            }
            res
        }
        fn read_lines(&mut self) -> Vec {
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
}
}
}