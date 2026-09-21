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
    ($t: ty, $read_name: ident, $read_vec_name: ident) => {
        pub fn $read_name(&mut self) -> $t {
            self.read()
        }

        pub fn $read_vec_name(&mut self, len: usize) -> Vec<$t> {
            self.read_vec(len)
        }
    };

    ($t: ty, $read_name: ident, $read_vec_name: ident, $read_pair_vec_name: ident) => {
        read_impl!($t, $read_name, $read_vec_name);

        pub fn $read_pair_vec_name(&mut self, len: usize) -> Vec<($t, $t)> {
            self.read_vec(len)
        }
    };
}

impl Input {
    const DEFAULT_BUF_SIZE: usize = 1 << 21;
    /// Zero bytes kept after the readable part of `buf`, so the 8-byte
    /// SWAR loads in integer parsing never run off the allocation.
    const SLACK: usize = 64;
    const FIX_EOL: bool = true;

    pub fn slice(input: &[u8]) -> Self {
        let mut buf = input.to_vec();
        buf.resize(input.len() + Self::SLACK, 0);
        Self {
            input: InputSource::Slice,
            buf,
            at: 0,
            buf_read: input.len(),
            eol: true,
        }
    }

    pub fn stdin() -> Self {
        Self::new(InputSource::Stdin(std::io::stdin()))
    }

    pub fn file(file: File) -> Self {
        Self::new(InputSource::File(file))
    }

    pub fn delegate(reader: impl Read + Send + 'static) -> Self {
        Self::new(InputSource::Delegate(Box::new(reader)))
    }

    fn new(input: InputSource) -> Self {
        Self {
            input,
            buf: vec![0; Self::DEFAULT_BUF_SIZE + Self::SLACK],
            at: 0,
            buf_read: 0,
            eol: true,
        }
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.buf.len() - Self::SLACK
    }

    fn read_more(&mut self) -> usize {
        let cap = self.capacity();
        let target = &mut self.buf[self.buf_read..cap];
        let read = match &mut self.input {
            InputSource::Stdin(stdin) => stdin.read(target).unwrap(),
            InputSource::File(file) => file.read(target).unwrap(),
            InputSource::Delegate(reader) => reader.read(target).unwrap(),
            InputSource::Slice => 0,
        };
        self.buf_read += read;
        let end = (self.buf_read + Self::SLACK).min(self.buf.len());
        self.buf[self.buf_read..end].fill(0);
        read
    }

    /// Makes at least `n` unread bytes available at `self.at` unless the
    /// source is exhausted first; the bytes after `buf_read` are zero.
    #[inline]
    fn ensure(&mut self, n: usize) {
        if self.buf_read - self.at < n {
            self.ensure_slow(n);
        }
    }

    fn ensure_slow(&mut self, n: usize) {
        if self.token_ends_in_buffer() {
            // Reading on could block forever in an interactive problem: the
            // other side has sent everything it is going to send.
            return;
        }
        self.buf.copy_within(self.at..self.buf_read, 0);
        self.buf_read -= self.at;
        self.at = 0;
        while self.buf_read < n && !self.token_ends_in_buffer() && self.read_more() != 0 {}
        if self.buf_read < n {
            let end = (self.buf_read + Self::SLACK).min(self.buf.len());
            self.buf[self.buf_read..end].fill(0);
        }
    }

    /// Whether the unread bytes already contain the whitespace that ends the
    /// current token. A `\r` at the very end does not count, so that the `\n`
    /// of a `\r\n` pair is still fetched and consumed with it.
    fn token_ends_in_buffer(&self) -> bool {
        match self.buf[self.at..self.buf_read]
            .iter()
            .position(|&b| b <= b' ')
        {
            Some(pos) => self.buf[self.at + pos] != b'\r' || self.at + pos + 1 < self.buf_read,
            None => false,
        }
    }

    #[inline]
    pub fn get(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            let res = self.buf[self.at];
            self.at += 1;
            if Self::FIX_EOL && res == b'\r' {
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

    #[inline]
    pub fn peek(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            let res = self.buf[self.at];
            Some(if res == b'\r' { b'\n' } else { res })
        } else {
            None
        }
    }

    #[inline]
    pub fn skip_whitespace(&mut self) {
        loop {
            let buf = self.buf.as_ptr();
            let end = self.buf_read;
            let mut at = self.at;
            let mut eol = self.eol;
            while at < end {
                let b = unsafe { *buf.add(at) };
                if !b.is_ascii_whitespace() {
                    self.at = at;
                    self.eol = eol;
                    return;
                }
                eol = b == b'\n' || b == b'\r';
                at += 1;
            }
            self.at = at;
            self.eol = eol;
            if !self.refill_buffer() {
                return;
            }
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

    //noinspection RsSelfConvention
    pub fn is_empty(&mut self) -> bool {
        self.skip_whitespace();
        self.is_exhausted()
    }

    pub fn check_empty(&mut self) -> bool {
        match self.input {
            InputSource::Slice => self.is_empty(),
            _ => true,
        }
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
            self.buf_read = 0;
            self.read_more() != 0
        } else {
            true
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn is_eol(&mut self) -> bool {
        self.eol || self.is_exhausted()
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

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        for (i, c) in buf.iter_mut().enumerate() {
            match self.get() {
                Some(b) => *c = b,
                None => return Ok(i),
            }
        }
        Ok(buf.len())
    }
}

const SWAR_ZEROS: u64 = 0x3030_3030_3030_3030;
const SWAR_LOW7: u64 = 0x7f7f_7f7f_7f7f_7f7f;
const SWAR_HIGH: u64 = 0x8080_8080_8080_8080;
const POW10: [u64; 9] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
];

/// Value of the digits stored one per byte in `x` (first digit in the lowest
/// byte, every byte already reduced to 0..=9).
#[inline]
fn swar_digits(x: u64) -> u64 {
    let x = (x * 10 + (x >> 8)) & 0x00ff_00ff_00ff_00ff;
    let x = (x * 100 + (x >> 16)) & 0x0000_ffff_0000_ffff;
    (x * 10_000 + (x >> 32)) & 0xffff_ffff
}

#[cold]
#[inline(never)]
fn read_past_the_end() -> ! {
    panic!("read past the end of input");
}

macro_rules! read_integer {
    // `$signed` is a literal, so the sign handling const-folds away for
    // unsigned types. `$acc` is the unsigned accumulator (u64 or u128).
    ($signed: literal, $acc: ty, $($t:ident)+) => {$(
        impl Readable for $t {
            #[inline]
            fn read(input: &mut Input) -> Self {
                let mut at = input.at;
                loop {
                    let buf = input.buf.as_ptr();
                    while at < input.buf_read && unsafe { *buf.add(at) } <= b' ' {
                        at += 1;
                    }
                    input.at = at;
                    if at < input.buf_read {
                        break;
                    }
                    if !input.refill_buffer() {
                        read_past_the_end();
                    }
                    at = input.at;
                }
                let mut buf = input.buf.as_ptr();
                let first = unsafe { *buf.add(at) };
                let neg = $signed && first == b'-';
                if neg || first == b'+' {
                    at += 1;
                }
                let mut res: $acc = 0;
                loop {
                    if input.buf_read < at + 10 {
                        input.at = at;
                        input.ensure(10);
                        buf = input.buf.as_ptr();
                        at = input.at;
                    }
                    // Eight bytes at once: a byte is a digit iff (b ^ b'0') < 10.
                    let chunk = unsafe { (buf.add(at) as *const u64).read_unaligned() };
                    let x = u64::from_le(chunk) ^ SWAR_ZEROS;
                    let non_digit = (((x & SWAR_LOW7) + 0x7676_7676_7676_7676) | x) & SWAR_HIGH;
                    if non_digit == 0 {
                        res = res.wrapping_mul(100_000_000).wrapping_add(swar_digits(x) as $acc);
                        at += 8;
                    } else {
                        let k = (non_digit.trailing_zeros() / 8) as usize;
                        if k != 0 {
                            let v = swar_digits(x << (64 - 8 * k));
                            res = res.wrapping_mul(POW10[k] as $acc).wrapping_add(v as $acc);
                            at += k;
                        }
                        break;
                    }
                }
                // Consume one whitespace terminator (`\r\n` counts as one).
                if at < input.buf_read {
                    let c = unsafe { *buf.add(at) };
                    input.eol = c == b'\n' || c == b'\r';
                    if c <= b' ' {
                        at += 1;
                        if c == b'\r' && at < input.buf_read && unsafe { *buf.add(at) } == b'\n' {
                            at += 1;
                        }
                    }
                }
                input.at = at;
                if $signed && neg {
                    res.wrapping_neg() as $t
                } else {
                    res as $t
                }
            }
        }
    )+};
}

read_integer!(true, u64, i8 i16 i32 i64 isize);
read_integer!(false, u64, u16 u32 u64 usize);
read_integer!(true, u128, i128);
read_integer!(false, u128, u128);

macro_rules! tuple_readable {
    ($($name:ident)+) => {
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
