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
    ($($t:ident)+) => {$(
        impl Readable for $t {
            fn read(input: &mut Input) -> Self {
                input.skip_whitespace();
                let mut c = input.get().unwrap();
                let sgn = match c {
                    b'-' => {
                        c = input.get().unwrap();
                        true
                    }
                    b'+' => {
                        c = input.get().unwrap();
                        false
                    }
                    _ => false,
                };
                let mut res = 0;
                loop {
                    assert!(c.is_ascii_digit());
                    res *= 10;
                    let d = (c - b'0') as $t;
                    if sgn {
                        res -= d;
                    } else {
                        res += d;
                    }
                    match input.get() {
                        None => break,
                        Some(ch) => {
                            if ch.is_ascii_whitespace() {
                                break;
                            } else {
                                c = ch;
                            }
                        }
                    }
                }
                res
            }
        }
    )+};
}

read_integer!(i8 i16 i32 i64 i128 isize u16 u32 u64 u128 usize);

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
