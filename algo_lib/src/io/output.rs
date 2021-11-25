use std::fmt::Display;
use std::io::Write;

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
