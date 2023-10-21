use crate::collections::vec_ext::default::default_vec;
use std::io::{stderr, Stderr, Write};

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

pub struct Output<'s> {
    output: &'s mut dyn Write,
    buf: Vec<u8>,
    at: usize,
    auto_flush: bool,
    bool_output: BoolOutput,
}

impl<'s> Output<'s> {
    const DEFAULT_BUF_SIZE: usize = 4096;

    pub fn new(output: &'s mut dyn Write) -> Self {
        Self {
            output,
            buf: default_vec(Self::DEFAULT_BUF_SIZE),
            at: 0,
            auto_flush: false,
            bool_output: BoolOutput::YesNoCaps,
        }
    }

    pub fn new_with_auto_flush(output: &'s mut dyn Write) -> Self {
        Self {
            output,
            buf: default_vec(Self::DEFAULT_BUF_SIZE),
            at: 0,
            auto_flush: true,
            bool_output: BoolOutput::YesNoCaps,
        }
    }

    pub fn flush(&mut self) {
        if self.at != 0 {
            self.output.write_all(&self.buf[..self.at]).unwrap();
            self.output.flush().unwrap();
            self.at = 0;
        }
    }

    pub fn print<T: Writable>(&mut self, s: T) {
        s.write(self);
        self.maybe_flush();
    }

    pub fn print_line<T: Writable>(&mut self, s: T) {
        self.print(s);
        self.put(b'\n');
        self.maybe_flush();
    }

    pub fn put(&mut self, b: u8) {
        self.buf[self.at] = b;
        self.at += 1;
        if self.at == self.buf.len() {
            self.flush();
        }
    }

    pub fn maybe_flush(&mut self) {
        if self.auto_flush {
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

    pub fn set_bool_output(&mut self, bool_output: BoolOutput) {
        self.bool_output = bool_output;
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
        self.maybe_flush();
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

impl<T: Writable> Writable for [T] {
    fn write(&self, output: &mut Output) {
        output.print_iter_ref(self.iter());
    }
}

impl<T: Writable, const N: usize> Writable for [T; N] {
    fn write(&self, output: &mut Output) {
        output.print_iter_ref(self.iter());
    }
}

impl<T: Writable> Writable for &T {
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
    ($($t:ident)+) => {$(
        impl Writable for $t {
            fn write(&self, output: &mut Output) {
                self.to_string().write(output);
            }
        }
    )+};
}

write_to_string!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

macro_rules! tuple_writable {
    ($name0:ident $($name:ident: $id:tt )*) => {
        impl<$name0: Writable, $($name: Writable,)*> Writable for ($name0, $($name,)*) {
            fn write(&self, out: &mut Output) {
                self.0.write(out);
                $(
                out.put(b' ');
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

static mut ERR: Option<Stderr> = None;
pub fn err() -> Output<'static> {
    unsafe {
        if ERR.is_none() {
            ERR = Some(stderr());
        }
        Output::new_with_auto_flush(ERR.as_mut().unwrap())
    }
}
