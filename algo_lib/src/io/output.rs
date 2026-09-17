use std::cmp::Reverse;
use std::fs::File;
use std::io::{StdoutLock, Write};

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
    Stdout(StdoutLock<'static>),
    File(File),
    Buf(&'s mut Vec<u8>),
    Delegate(Box<dyn Write + 'static>),
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
        Self::new(OutputDest::Stdout(std::io::stdout().lock()))
    }

    pub fn file(file: File) -> Self {
        Self::new(OutputDest::File(file))
    }

    pub fn delegate(delegate: impl Write + 'static) -> Self {
        Self::new(OutputDest::Delegate(Box::new(delegate)))
    }
}

impl Output<'_> {
    const DEFAULT_BUF_SIZE: usize = 1 << 16;

    pub fn flush(&mut self) {
        if self.at != 0 {
            let dest: &mut dyn Write = match &mut self.output {
                OutputDest::Stdout(stdout) => stdout,
                OutputDest::File(file) => file,
                OutputDest::Delegate(delegate) => delegate,
                OutputDest::Buf(buf) => {
                    buf.extend_from_slice(&self.buf[..self.at]);
                    self.at = 0;
                    return;
                }
            };
            dest.write_all(&self.buf[..self.at]).unwrap();
            dest.flush().unwrap();
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

    /// Guarantees `n` free bytes in the buffer.
    #[inline]
    fn reserve(&mut self, n: usize) {
        if self.at + n > self.buf.len() {
            self.flush();
        }
    }

    fn write_u64(&mut self, n: u64) {
        let len = n.checked_ilog10().map_or(1, |l| l as usize + 1);
        // Digits end at tmp[24]; the zero tail lets three fixed-size 8-byte
        // copies move the number without a variable-length memcpy.
        let mut tmp = [0u8; 48];
        Self::fill_digits(&mut tmp[..24], n);
        self.reserve(48);
        let src = tmp[24 - len..].as_ptr();
        let dst = self.buf[self.at..].as_mut_ptr();
        unsafe {
            std::ptr::copy_nonoverlapping(src, dst, 24);
        }
        self.at += len;
    }

    /// Writes `n` right-aligned into `slot` (at least 20 bytes), zero-padded
    /// on the left in whole 4-digit groups; digits above `slot.len()` are lost.
    #[inline]
    fn fill_digits(slot: &mut [u8], mut n: u64) {
        let mut pos = slot.len();
        loop {
            let q = n / 10_000;
            let r = (n - q * 10_000) as usize;
            pos -= 4;
            slot[pos..pos + 4].copy_from_slice(&DIGITS4[r]);
            n = q;
            if n == 0 {
                break;
            }
        }
    }

    fn write_u128(&mut self, n: u128) {
        if let Ok(n) = u64::try_from(n) {
            self.write_u64(n);
        } else {
            const SPLIT: u128 = 10_000_000_000_000_000_000;
            self.write_u128(n / SPLIT);
            let mut tmp = [b'0'; 24];
            Self::fill_digits(&mut tmp, (n % SPLIT) as u64);
            self.reserve(19);
            self.buf[self.at..self.at + 19].copy_from_slice(&tmp[5..]);
            self.at += 19;
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

static DIGITS4: [[u8; 4]; 10_000] = {
    let mut t = [[0u8; 4]; 10_000];
    let mut i = 0;
    while i < 10_000 {
        t[i] = [
            b'0' + (i / 1000) as u8,
            b'0' + (i / 100 % 10) as u8,
            b'0' + (i / 10 % 10) as u8,
            b'0' + (i % 10) as u8,
        ];
        i += 1;
    }
    t
};

macro_rules! write_unsigned_int {
    ($($t:ident)+) => {$(
        impl Writable for $t {
            #[inline]
            fn write(&self, output: &mut Output) {
                output.write_u64(*self as u64);
            }
        }
    )+};
}

macro_rules! write_signed_int {
    ($($t:ident)+) => {$(
        impl Writable for $t {
            #[inline]
            fn write(&self, output: &mut Output) {
                if *self < 0 {
                    output.put(b'-');
                }
                output.write_u64(self.unsigned_abs() as u64);
            }
        }
    )+};
}

write_unsigned_int!(u16 u32 u64 usize);
write_signed_int!(i8 i16 i32 i64 isize);

impl Writable for u128 {
    fn write(&self, output: &mut Output) {
        output.write_u128(*self);
    }
}

impl Writable for i128 {
    fn write(&self, output: &mut Output) {
        if *self < 0 {
            output.put(b'-');
        }
        output.write_u128(self.unsigned_abs());
    }
}

macro_rules! tuple_writable {
    ($name0:ident $($name:ident: $id:tt )*) => {
        impl<$name0: Writable, $($name: Writable,)*> Writable for ($name0, $($name,)*) {
            fn write(&self, out: &mut Output) {
                self.0.write(out);
                $(
                out.put(out.separator);
                self.$id.write(out);
                )*
            }
        }
    }
}

tuple_writable! {T}
tuple_writable! {T U:1}
tuple_writable! {T U:1 V:2}
tuple_writable! {T U:1 V:2 X:3}
tuple_writable! {T U:1 V:2 X:3 Y:4}
tuple_writable! {T U:1 V:2 X:3 Y:4 Z:5}
tuple_writable! {T U:1 V:2 X:3 Y:4 Z:5 A:6}
tuple_writable! {T U:1 V:2 X:3 Y:4 Z:5 A:6 B:7}
tuple_writable! {T U:1 V:2 X:3 Y:4 Z:5 A:6 B:7 C:8}

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
