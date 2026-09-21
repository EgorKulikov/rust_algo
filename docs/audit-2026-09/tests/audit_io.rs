#![allow(clippy::all)]
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::eol::{EolStr, EolVec};
use algo_lib::io::input::{Input, Readable};
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::{BoolOutput, Output, Writable};
use algo_lib::numbers::real::{Real, RealReader};
use algo_lib::string::str::{Str, StrReader};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::fmt::Display;
use std::io::Read;
use std::str::FromStr;

/// Reader handing out data in chunks following a cyclic pattern of sizes.
struct Chunked {
    data: Vec<u8>,
    at: usize,
    pattern: Vec<usize>,
    idx: usize,
}

impl Chunked {
    fn new(data: &[u8], pattern: &[usize]) -> Self {
        Self {
            data: data.to_vec(),
            at: 0,
            pattern: pattern.to_vec(),
            idx: 0,
        }
    }
}

impl Read for Chunked {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let c = self.pattern[self.idx % self.pattern.len()];
        self.idx += 1;
        let n = buf.len().min(c).min(self.data.len() - self.at);
        buf[..n].copy_from_slice(&self.data[self.at..self.at + n]);
        self.at += n;
        Ok(n)
    }
}

fn patterns() -> Vec<Vec<usize>> {
    let mut res: Vec<Vec<usize>> = [1usize, 2, 3, 7, 8, 9, 10, 11, 16, 17, 1 << 30]
        .iter()
        .map(|&c| vec![c])
        .collect();
    res.push(vec![1, 8, 2, 9, 3, 7]);
    res.push(vec![5, 1, 1, 13]);
    res
}

fn inputs(data: &[u8]) -> Vec<Input> {
    let mut res = vec![Input::slice(data)];
    for p in patterns() {
        res.push(Input::delegate(Chunked::new(data, &p)));
    }
    res
}

const SEPS: [&str; 8] = [" ", "\n", "\r\n", "\t", "   ", "\n\n\n", " \r\n\r\n ", "\t \n"];

fn int_doc<T>(rng: &mut StdRng, vals: &[T]) -> String
where
    T: Display + Copy + PartialOrd + Default,
{
    let mut text = String::new();
    if rng.gen_bool(0.3) {
        text += SEPS[rng.gen_range(0..SEPS.len())];
    }
    for (i, v) in vals.iter().enumerate() {
        let s = v.to_string();
        let (sign, digits) = if let Some(d) = s.strip_prefix('-') {
            ("-", d.to_string())
        } else if rng.gen_bool(0.1) {
            ("+", s)
        } else {
            ("", s)
        };
        let zeros = if rng.gen_bool(0.15) {
            "0".repeat(rng.gen_range(1..12))
        } else {
            String::new()
        };
        text += sign;
        text += &zeros;
        text += &digits;
        if i + 1 != vals.len() || rng.gen_bool(0.5) {
            text += SEPS[rng.gen_range(0..SEPS.len())];
        }
    }
    text
}

macro_rules! int_test {
    ($name: ident, $t: ty, $signed: expr) => {
        #[test]
        fn $name() {
            let mut rng = StdRng::seed_from_u64(1);
            // extremes and around powers of ten
            let mut special: Vec<$t> = vec![<$t>::MIN, <$t>::MAX, 0 as $t, 1 as $t];
            let mut p: u128 = 1;
            loop {
                for d in [p.wrapping_sub(1), p, p + 1] {
                    if let Ok(x) = <$t>::try_from(d) {
                        special.push(x);
                        if $signed {
                            special.push((0 as $t).wrapping_sub(x));
                        }
                    }
                }
                match p.checked_mul(10) {
                    Some(np) => p = np,
                    None => break,
                }
            }
            for iter in 0..300 {
                let n = rng.gen_range(1..25);
                let vals: Vec<$t> = (0..n)
                    .map(|_| {
                        if rng.gen_bool(0.2) {
                            special[rng.gen_range(0..special.len())]
                        } else {
                            let x: $t = rng.gen();
                            x >> rng.gen_range(0..<$t>::BITS)
                        }
                    })
                    .collect();
                let text = int_doc(&mut rng, &vals);
                // sanity: str::parse agrees with the intended values
                for (tok, v) in text.split_ascii_whitespace().zip(vals.iter()) {
                    assert_eq!(<$t>::from_str(tok).unwrap(), *v);
                }
                for (k, mut input) in inputs(text.as_bytes()).into_iter().enumerate() {
                    for (i, &v) in vals.iter().enumerate() {
                        let got: $t = input.read();
                        assert_eq!(got, v, "iter {iter} input {k} idx {i} text {text:?}");
                    }
                    assert!(input.is_empty(), "iter {iter} input {k} text {text:?}");
                }
            }
            // "-0"
            if $signed {
                for mut input in inputs(b"-0 -000 +0") {
                    assert_eq!(input.read::<$t>(), 0 as $t);
                    assert_eq!(input.read::<$t>(), 0 as $t);
                    assert_eq!(input.read::<$t>(), 0 as $t);
                    assert!(input.is_exhausted());
                }
            }
        }
    };
}

int_test!(parse_i8, i8, true);
int_test!(parse_i16, i16, true);
int_test!(parse_i32, i32, true);
int_test!(parse_i64, i64, true);
int_test!(parse_isize, isize, true);
int_test!(parse_i128, i128, true);
int_test!(parse_u16, u16, false);
int_test!(parse_u32, u32, false);
int_test!(parse_u64, u64, false);
int_test!(parse_usize, usize, false);
int_test!(parse_u128, u128, false);

#[test]
fn split_at_every_offset() {
    let text = b"  -12345678901234567 +42\r\n007 -0\n18446744073709551615\t-9223372036854775808 \r\n\r\n 99999999 100000000 -170141183460469231731687303715884105728";
    let n = text.len();
    for p in 0..=n {
        for q in p..=n {
            let pattern = [p.max(1), (q - p).max(1), 1 << 30];
            // when p == 0 or q == p the pattern degenerates, still a valid chunking
            let mut input = Input::delegate(Chunked::new(text, &pattern));
            assert_eq!(input.read::<i64>(), -12345678901234567);
            assert_eq!(input.read::<i32>(), 42);
            assert!(input.is_eol());
            assert_eq!(input.read::<usize>(), 7);
            assert_eq!(input.read::<i8>(), 0);
            assert!(input.is_eol());
            assert_eq!(input.read::<u64>(), u64::MAX);
            assert!(!input.is_eol());
            assert_eq!(input.read::<i64>(), i64::MIN);
            assert_eq!(input.read::<u32>(), 99999999);
            assert_eq!(input.read::<u32>(), 100000000);
            assert_eq!(input.read::<i128>(), i128::MIN);
            assert!(input.is_empty());
            assert!(input.is_exhausted());
            assert_eq!(input.peek(), None);
            assert_eq!(input.get(), None);
        }
    }
}

// ---------------------------------------------------------------- interactive

/// Reader over a channel: blocks when no message is available, like a pipe
/// from an interactive judge.
struct ChannelReader {
    rcv: std::sync::mpsc::Receiver<Vec<u8>>,
    cur: Vec<u8>,
    at: usize,
}

impl Read for ChannelReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.at == self.cur.len() {
            self.cur = match self.rcv.recv() {
                Ok(v) => v,
                Err(_) => return Ok(0),
            };
            self.at = 0;
        }
        let n = buf.len().min(self.cur.len() - self.at);
        buf[..n].copy_from_slice(&self.cur[self.at..self.at + n]);
        self.at += n;
        Ok(n)
    }
}

fn interactive_read<T: Send + 'static>(
    message: &[u8],
    f: impl FnOnce(&mut Input) -> T + Send + 'static,
) -> Option<T> {
    let (snd, rcv) = std::sync::mpsc::channel();
    let (res_snd, res_rcv) = std::sync::mpsc::channel();
    snd.send(message.to_vec()).unwrap();
    std::thread::spawn(move || {
        let mut input = Input::delegate(ChannelReader {
            rcv,
            cur: Vec::new(),
            at: 0,
        });
        let _ = res_snd.send(f(&mut input));
    });
    // the "judge" now waits for the answer before sending anything else
    let res = res_rcv.recv_timeout(std::time::Duration::from_secs(2)).ok();
    drop(snd);
    res
}

#[test]
fn interactive_int_does_not_block() {
    assert_eq!(interactive_read(b"5\n", |i| i.read_int()), Some(5));
}

#[test]
fn interactive_other_reads_do_not_block() {
    assert_eq!(
        interactive_read(b"abc\n", |i| i.read_str().unwrap()),
        Some(b"abc".to_vec())
    );
    assert_eq!(
        interactive_read(b"abc def\n", |i| i.read_line().unwrap()),
        Some(b"abc def".to_vec())
    );
    assert_eq!(interactive_read(b"x\n", |i| i.read_char()), Some(b'x'));
    assert_eq!(
        interactive_read(b"1.5\n", |i| i.read_real().0),
        Some(1.5)
    );
    // long enough message: >= 10 bytes after the number start
    assert_eq!(
        interactive_read(b"5 1234567890\n", |i| i.read_int()),
        Some(5)
    );
}

#[test]
fn interactive_int_pair_blocks_on_second() {
    assert_eq!(
        interactive_read(b"5 1234567890\n", |i| (i.read_int(), i.read_long())),
        Some((5, 1234567890))
    );
}

// ---------------------------------------------------------------- lines

#[test]
fn read_line_last_line_without_newline() {
    let mut input = Input::slice(b"abc");
    assert_eq!(input.read_line().as_slice(), b"abc");
}

#[test]
fn read_lines_various() {
    for text in ["a b\n\nc d \r\n\r\nlast\n", "a b\r\n\r\nc d \n\nlast\r\n"] {
        for (k, mut input) in inputs(text.as_bytes()).into_iter().enumerate() {
            assert_eq!(input.read_line().as_slice(), b"a b", "{k}");
            assert_eq!(input.read_line().as_slice(), b"", "{k}");
            assert_eq!(input.read_line().as_slice(), b"c d ", "{k}");
            assert_eq!(input.read_line().as_slice(), b"", "{k}");
            assert_eq!(input.read_line().as_slice(), b"last", "{k}");
            assert!(input.is_exhausted(), "{k}");
        }
    }
}

#[test]
fn int_then_line() {
    for text in ["3\nabc def\n12 \nxyz\n", "3\r\nabc def\r\n12 \r\nxyz\r\n"] {
        for (k, mut input) in inputs(text.as_bytes()).into_iter().enumerate() {
            assert_eq!(input.read_int(), 3, "{k}");
            assert_eq!(input.read_line().as_slice(), b"abc def", "{k}");
            assert_eq!(input.read_int(), 12, "{k}");
            // the single space was consumed as the terminator, rest of line is empty
            assert_eq!(input.read_line().as_slice(), b"", "{k}");
            assert_eq!(input.read_line().as_slice(), b"xyz", "{k}");
            assert!(input.is_exhausted(), "{k}");
        }
    }
}

#[test]
fn str_then_line_crlf() {
    for text in ["tok\r\nline one\r\n", "tok\nline one\n"] {
        for (k, mut input) in inputs(text.as_bytes()).into_iter().enumerate() {
            assert_eq!(input.read_str().as_slice(), b"tok", "{k}");
            assert_eq!(input.read_line().as_slice(), b"line one", "{k}");
            assert!(input.is_exhausted(), "{k}");
        }
    }
}

// ---------------------------------------------------------------- buffer boundaries

const BUF: usize = 1 << 21;

#[test]
fn number_straddles_internal_buffer_end() {
    // prefix consumed as ints, then numbers placed so they straddle the 2^21 boundary
    let tail = "-9223372036854775808 18446744073709551615\r\n-17 +000123456789012345 x\n";
    for k in 0..=44usize {
        for chunk in [1usize << 30, 65536, BUF - 1, BUF + 1] {
            let prefix_len = BUF - k;
            let mut text = String::with_capacity(prefix_len + tail.len());
            let mut cnt = 0;
            while text.len() + 2 <= prefix_len {
                text += "7 ";
                cnt += 1;
            }
            while text.len() < prefix_len {
                text.push('\n');
            }
            text += tail;
            let mut input = Input::delegate(Chunked::new(text.as_bytes(), &[chunk]));
            for _ in 0..cnt {
                assert_eq!(input.read::<u16>(), 7);
            }
            assert_eq!(input.read_long(), i64::MIN, "k={k}");
            assert_eq!(input.read_u64(), u64::MAX, "k={k}");
            assert!(input.is_eol());
            assert_eq!(input.read_int(), -17, "k={k}");
            assert_eq!(input.read_long(), 123456789012345, "k={k}");
            assert_eq!(input.read_char(), b'x');
            assert!(input.is_empty());
        }
    }
}

#[test]
fn input_of_exactly_buffer_size() {
    for len in [BUF - 2, BUF - 1, BUF, BUF + 1, BUF + 2, 2 * BUF - 1, 2 * BUF, 2 * BUF + 1] {
        for trailing_newline in [false, true] {
            // one huge token followed by a number that ends exactly at `len`
            let num = "1234567890123456789";
            let sep_and_num = num.len() + 1 + trailing_newline as usize;
            let mut text = vec![b'a'; len - sep_and_num];
            text.push(b' ');
            text.extend_from_slice(num.as_bytes());
            if trailing_newline {
                text.push(b'\n');
            }
            assert_eq!(text.len(), len);
            for mut input in [
                Input::slice(&text),
                Input::delegate(Chunked::new(&text, &[1 << 30])),
                Input::delegate(Chunked::new(&text, &[BUF])),
                Input::delegate(Chunked::new(&text, &[4093])),
            ] {
                let s = input.read_str();
                assert_eq!(s.len(), len - sep_and_num);
                assert!(s.iter().all(|&c| c == b'a'));
                assert_eq!(input.read_long(), 1234567890123456789);
                assert!(input.is_eol());
                assert!(input.is_exhausted());
                assert!(input.is_empty());
            }
            // all digits with leading zeros: a numeric token longer than the buffer
            let mut text = vec![b'0'; len - 4];
            text.extend_from_slice(b"4242");
            for mut input in [
                Input::slice(&text),
                Input::delegate(Chunked::new(&text, &[1 << 30])),
                Input::delegate(Chunked::new(&text, &[65536])),
            ] {
                assert_eq!(input.read_long(), 4242);
                assert!(input.is_exhausted());
            }
            // whitespace only, longer than the buffer, then a number
            let mut text = vec![b' '; len - 3];
            text.extend_from_slice(b"-42");
            for mut input in [
                Input::slice(&text),
                Input::delegate(Chunked::new(&text, &[1 << 30])),
            ] {
                assert_eq!(input.read_int(), -42);
                assert!(input.is_exhausted());
            }
        }
    }
}

// ---------------------------------------------------------------- other reads

#[test]
fn eof_behaviour() {
    for text in ["", " ", "\n", " \r\n \n\t"] {
        for (k, mut input) in inputs(text.as_bytes()).into_iter().enumerate() {
            assert!(input.is_empty(), "{k} {text:?}");
            assert!(input.is_exhausted());
            assert_eq!(input.peek(), None);
            assert_eq!(input.get(), None);
            assert_eq!(input.next_token(), None);
            assert!(input.is_eol());
            assert_eq!(input.iter_int().count(), 0);
        }
    }
    for (k, mut input) in inputs(b"1 \n").into_iter().enumerate() {
        assert!(!input.is_exhausted(), "{k}");
        assert!(!input.is_empty());
        assert_eq!(input.peek(), Some(b'1'));
        assert_eq!(input.read_int(), 1);
        assert!(!input.is_exhausted(), "{k}"); // trailing "\n" remains
        assert!(input.is_empty());
        assert!(input.is_exhausted());
    }
    for (k, mut input) in inputs(b"\r\nx").into_iter().enumerate() {
        assert_eq!(input.peek(), Some(b'\n'), "{k}");
        assert_eq!(input.get(), Some(b'\n'));
        assert_eq!(input.get(), Some(b'x'));
        assert_eq!(input.get(), None);
    }
}

#[test]
fn mixed_reads() {
    let text = b"3 2\n1 2\n2 3\n3 1\n#.#\n.#.\nabc -1.5e3 .25 -.5 1e-3 123456789.123456789\n5 1 2 3 4 5\n1 2 3\n4 5 6\n7 x 9.5 foo\n10 20 30\r\n40\n";
    for (k, mut input) in inputs(text).into_iter().enumerate() {
        let (n, m): (usize, usize) = input.read();
        assert_eq!((n, m), (3, 2), "{k}");
        let edges = input.read_size_pair_vec(n).dec();
        assert_eq!(edges, vec![(0, 1), (1, 2), (2, 0)]);
        let table = input.read_char_table(m, 3);
        assert_eq!(table[(0, 0)], b'#');
        assert_eq!(table[(0, 1)], b'.');
        assert_eq!(table[(1, 1)], b'#');
        assert_eq!(table[(1, 2)], b'.');
        assert_eq!(input.read_str().as_slice(), b"abc");
        assert_eq!(input.read_real().0, -1500.0);
        assert_eq!(input.read_real().0, 0.25);
        assert_eq!(input.read_real().0, -0.5);
        assert_eq!(input.read_real().0, 0.001);
        assert_eq!(input.read_real().0, 123456789.123456789);
        let v: Vec<i32> = input.read();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
        let t: Arr2d<i64> = input.read_long_table(2, 3);
        assert_eq!(t[(1, 0)], 4);
        assert_eq!(t[(1, 2)], 6);
        let tup: (i32, u8, Real, Str) = input.read();
        assert_eq!(tup.0, 7);
        assert_eq!(tup.1, b'x');
        assert_eq!(tup.2 .0, 9.5);
        assert_eq!(tup.3.as_slice(), b"foo");
        // the '\n' after foo was consumed by the Str read; read an eol-terminated vec
        let ev: EolVec<i32> = input.read();
        assert_eq!(ev.unwrap(), vec![10, 20, 30], "{k}");
        let arr: [i32; 1] = input.read();
        assert_eq!(arr, [40]);
        assert!(input.is_empty());
    }
}

#[test]
fn eol_vec_and_eol_str() {
    for text in ["1 2 3\n4\n5 6", "1 2 3\r\n4\r\n5 6\r\n"] {
        for (k, mut input) in inputs(text.as_bytes()).into_iter().enumerate() {
            let a: EolVec<i64> = input.read();
            let b: EolVec<i64> = input.read();
            let c: EolVec<i64> = input.read();
            assert_eq!(a.unwrap(), vec![1, 2, 3], "{k} {text:?}");
            assert_eq!(b.unwrap(), vec![4], "{k} {text:?}");
            assert_eq!(c.unwrap(), vec![5, 6], "{k} {text:?}");
        }
    }
    for (k, mut input) in inputs(b"2\nhello world\nfoo bar\n").into_iter().enumerate() {
        let n = input.read_size();
        let lines: Vec<EolStr> = input.read_vec(n);
        assert_eq!(lines[0].as_slice(), b"hello world", "{k}");
        assert_eq!(lines[1].as_slice(), b"foo bar", "{k}");
    }
}

#[test]
fn input_iterator() {
    for text in ["1 2 3", "1 2 3\n", " 1\n2\r\n3 \n\n"] {
        for (k, mut input) in inputs(text.as_bytes()).into_iter().enumerate() {
            let v: Vec<i32> = input.iter_int().collect();
            assert_eq!(v, vec![1, 2, 3], "{k} {text:?}");
        }
    }
}

#[test]
fn eol_vec_trailing_space() {
    let mut input = Input::slice(b"1 2 \n3\n");
    let a: EolVec<i64> = input.read();
    assert_eq!(a.unwrap(), vec![1, 2]);
}

#[test]
fn read_int_past_eof() {
    // contract violation, but document the behaviour: must panic cleanly
    let res = std::panic::catch_unwind(|| {
        let mut input = Input::slice(b"1 ");
        assert_eq!(input.read_int(), 1);
        let x = input.read_int();
        (x, input.is_exhausted())
    });
    eprintln!("{res:?}");
}

// ---------------------------------------------------------------- output

fn written_with(f: impl FnOnce(&mut Output)) -> String {
    let mut buf = Vec::new();
    {
        let mut out = Output::buf(&mut buf);
        f(&mut out);
        out.flush();
    }
    String::from_utf8(buf).unwrap()
}

fn written<T: Writable>(v: T) -> String {
    written_with(|out| out.print(v))
}

macro_rules! out_int_test {
    ($name: ident, $t: ty, $signed: expr) => {
        #[test]
        fn $name() {
            let mut rng = StdRng::seed_from_u64(2);
            let mut special: Vec<$t> = vec![<$t>::MIN, <$t>::MAX, 0 as $t, 1 as $t];
            let mut p: u128 = 1;
            loop {
                for d in [p.wrapping_sub(1), p, p + 1, p * 2, p * 9] {
                    if let Ok(x) = <$t>::try_from(d) {
                        special.push(x);
                        if $signed {
                            special.push((0 as $t).wrapping_sub(x));
                        }
                    }
                }
                match p.checked_mul(10) {
                    Some(np) if np < u128::MAX / 10 => p = np,
                    _ => break,
                }
            }
            for &v in &special {
                assert_eq!(written(v), v.to_string());
            }
            // many values in one output (crosses the 64K buffer many times)
            let vals: Vec<$t> = (0..200_000)
                .map(|_| {
                    if rng.gen_bool(0.1) {
                        special[rng.gen_range(0..special.len())]
                    } else {
                        let x: $t = rng.gen();
                        x >> rng.gen_range(0..<$t>::BITS)
                    }
                })
                .collect();
            let got = written_with(|out| {
                for v in &vals {
                    out.print_line(*v);
                }
            });
            let mut expected = String::new();
            for v in &vals {
                expected += &v.to_string();
                expected.push('\n');
            }
            assert!(got == expected);
            // round trip through chunked input
            for pattern in [vec![1 << 30], vec![65536], vec![8191], vec![13, 1, 8]] {
                let mut input = Input::delegate(Chunked::new(got.as_bytes(), &pattern));
                for &v in &vals {
                    assert_eq!(input.read::<$t>(), v);
                }
                assert!(input.is_empty());
            }
            let got_line = written_with(|out| out.print_line(&vals));
            let mut input = Input::slice(got_line.as_bytes());
            let back: Vec<$t> = input.read_vec(vals.len());
            assert!(back == vals);
            assert!(input.is_empty());
        }
    };
}

out_int_test!(out_i8, i8, true);
out_int_test!(out_i16, i16, true);
out_int_test!(out_i32, i32, true);
out_int_test!(out_i64, i64, true);
out_int_test!(out_isize, isize, true);
out_int_test!(out_i128, i128, true);
out_int_test!(out_u16, u16, false);
out_int_test!(out_u32, u32, false);
out_int_test!(out_u64, u64, false);
out_int_test!(out_usize, usize, false);
out_int_test!(out_u128, u128, false);

#[test]
fn number_straddles_output_buffer_end() {
    const OBUF: usize = 1 << 16;
    let nums: [i128; 8] = [
        0,
        -7,
        12345,
        u64::MAX as i128,
        i64::MIN as i128,
        i128::MAX,
        i128::MIN,
        10_000_000_000_000_000_000,
    ];
    for k in 0..=80usize {
        for &v in &nums {
            let filler = vec![b'x'; OBUF - k];
            let got = written_with(|out| {
                out.print(Str::from(filler.clone()));
                out.print(v);
                out.put(b' ');
                if let Ok(x) = i64::try_from(v) {
                    out.print(x);
                }
                out.put(b' ');
                if let Ok(x) = u64::try_from(v) {
                    out.print(x);
                }
                out.print_line("end");
            });
            let mut expected = String::from_utf8(filler).unwrap();
            expected += &v.to_string();
            expected.push(' ');
            if let Ok(x) = i64::try_from(v) {
                expected += &x.to_string();
            }
            expected.push(' ');
            if let Ok(x) = u64::try_from(v) {
                expected += &x.to_string();
            }
            expected += "end\n";
            assert!(got == expected, "k={k} v={v}");
        }
    }
}

struct SharedWriter(std::sync::Arc<std::sync::Mutex<(Vec<u8>, usize)>>);
impl std::io::Write for SharedWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // accept only part of the data to check write_all semantics
        let n = buf.len().min(1000);
        self.0.lock().unwrap().0.extend_from_slice(&buf[..n]);
        Ok(n)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.lock().unwrap().1 += 1;
        Ok(())
    }
}

#[test]
fn delegate_output_and_flush() {
    let shared = std::sync::Arc::new(std::sync::Mutex::new((Vec::new(), 0usize)));
    let mut out = Output::delegate(SharedWriter(shared.clone()));
    out.print_line(5);
    assert!(shared.lock().unwrap().0.is_empty());
    out.flush();
    assert_eq!(shared.lock().unwrap().0.as_slice(), b"5\n");
    assert_eq!(shared.lock().unwrap().1, 1);
    let mut expected = b"5\n".to_vec();
    for i in 0..100_000u64 {
        out.print_line((i, i * i, -(i as i64)));
        expected.extend_from_slice(format!("{} {} {}\n", i, i * i, -(i as i64)).as_bytes());
    }
    out.flush();
    assert!(shared.lock().unwrap().0 == expected);
}

#[test]
fn misc_output() {
    assert_eq!(written(true), "YES");
    assert_eq!(written(false), "NO");
    for (bo, y, n) in [
        (BoolOutput::YesNo, "Yes", "No"),
        (BoolOutput::YesNoCaps, "YES", "NO"),
        (BoolOutput::PossibleImpossible, "Possible", "Impossible"),
        (BoolOutput::Custom("Alice", "Bob"), "Alice", "Bob"),
    ] {
        let got = written_with(|out| {
            out.set_bool_output(bo);
            out.print_line(true);
            out.print_line(false);
            out.print_line(vec![true, false]);
        });
        assert_eq!(got, format!("{y}\n{n}\n{y} {n}\n"));
    }
    assert_eq!(written(Some(5usize)), "5");
    assert_eq!(written(None::<usize>), "-1");
    assert_eq!(written(vec![Some(1), None, Some(3)]), "1 -1 3");
    assert_eq!(written((1, "a", 'c', b'd', -5i64, 7u128)), "1 a c d -5 7");
    assert_eq!(written(Vec::<i32>::new()), "");
    assert_eq!(written([1u32, 2, 3]), "1 2 3");
    assert_eq!(written(&[1u32, 2, 3][..]), "1 2 3");
    assert_eq!(written(vec![vec![1, 2], vec![3]]), "1 2 3");
    assert_eq!(written(String::from("hey")), "hey");
    assert_eq!(written(std::cmp::Reverse(-3)), "-3");
    assert_eq!(written(()), "");
    let got = written_with(|out| {
        out.print_per_line(&[1, 2, 3]);
        out.print_per_line(&[(1, 2), (3, 4)]);
        out.print_line_iter([7u64, 8].iter());
        out.print_iter(1..4);
        out.print_line(());
        out.print_per_line_iter((0..2).map(|x| (x, x + 1)));
        out.set_separator(b',');
        out.print_line((1, 2, vec![3, 4]));
        out.print_line(Arr2d::with_gen(2, 3, |i, j| i * 3 + j));
    });
    assert_eq!(got, "1\n2\n3\n1 2\n3 4\n7 8\n1 2 3\n0 1\n1 2\n1,2,3,4\n0,1,2\n3,4,5\n");
    let got = written_with(|out| {
        out.print_line(Arr2d::with_gen(2, 3, |i, j| (i * 3 + j) as i64 - 2));
        out.print_line(Arr2d::<i32>::new(0, 0, 0));
    });
    assert_eq!(got, "-2 -1 0\n1 2 3\n\n");
}

// ---------------------------------------------------------------- reals

fn expected_real(v: f64, precision: usize) -> String {
    let s = format!("{:.*}", precision, v);
    // "-0.000" should be printed without the sign, nothing else may change
    if s.starts_with('-') && s.bytes().all(|c| c == b'-' || c == b'0' || c == b'.') {
        s[1..].to_string()
    } else {
        s
    }
}

#[test]
fn real_output_fixed_precision_handpicked() {
    let got = written_with(|out| {
        out.set_precision(6);
        out.print_line(Real(0.9999996));
        out.print_line(Real(-0.0000001));
        out.print_line(Real(-0.0));
        out.print_line(Real(1e18));
        out.print_line(Real(-1.5));
        out.print_line(Real(99.9999995));
    });
    assert_eq!(got, "1.000000\n0.000000\n0.000000\n1000000000000000000.000000\n-1.500000\n100.000000\n");
}

#[test]
fn real_output_negative_ten() {
    let got = written_with(|out| {
        out.set_precision(6);
        out.print_line(Real(-10.0));
    });
    assert_eq!(got, "-10.000000\n");
}

#[test]
fn real_output_fixed_precision_random() {
    let mut rng = StdRng::seed_from_u64(3);
    let mut bad = Vec::new();
    for _ in 0..300_000 {
        let precision = rng.gen_range(0..=12usize);
        let scale = 10f64.powi(rng.gen_range(-8..10));
        let mut v: f64 = rng.gen_range(-1.0..1.0) * scale;
        if rng.gen_bool(0.3) {
            v = (v * 100.0).round() / 100.0;
        }
        if rng.gen_bool(0.2) {
            v = v.round();
        }
        let got = written_with(|out| {
            out.set_precision(precision);
            out.print(Real(v));
        });
        let expected = expected_real(v, precision);
        if got != expected {
            bad.push((v, precision, got, expected));
        }
    }
    bad.sort_by(|a, b| a.3.len().cmp(&b.3.len()));
    assert!(bad.is_empty(), "{} mismatches, e.g. {:?}", bad.len(), &bad[..bad.len().min(5)]);
}

#[test]
fn real_round_trip() {
    let mut rng = StdRng::seed_from_u64(4);
    let vals: Vec<f64> = (0..200_000)
        .map(|_| {
            let bits: u64 = rng.gen();
            let v = f64::from_bits(bits);
            if v.is_finite() {
                v
            } else {
                1.5
            }
        })
        .chain([0.0, -0.0, 1e300, -1e-300, f64::MAX, f64::MIN_POSITIVE, 5e-324])
        .collect();
    let text = written_with(|out| {
        for v in &vals {
            out.print_line(Real(*v));
        }
    });
    for pattern in [vec![1 << 30], vec![7], vec![4096]] {
        let mut input = Input::delegate(Chunked::new(text.as_bytes(), &pattern));
        for &v in &vals {
            let got = input.read_real().0;
            assert_eq!(got.to_bits(), v.to_bits(), "{v}");
        }
        assert!(input.is_empty());
    }
}

#[test]
fn str_round_trip() {
    let mut rng = StdRng::seed_from_u64(5);
    let toks: Vec<Vec<u8>> = (0..50_000)
        .map(|_| {
            let len = if rng.gen_bool(0.01) { rng.gen_range(1..200_000) } else { rng.gen_range(1..30) };
            (0..len).map(|_| rng.gen_range(33..=255u8)).collect()
        })
        .collect();
    let mut buf = Vec::new();
    {
        let mut out = Output::buf(&mut buf);
        for (i, t) in toks.iter().enumerate() {
            out.print(Str::from(t.clone()));
            out.print(SEPS[i % SEPS.len()]);
        }
        out.flush();
    }
    for pattern in [vec![1 << 30], vec![65536], vec![4099]] {
        let mut input = Input::delegate(Chunked::new(&buf, &pattern));
        for t in &toks {
            assert!(input.read_str().as_slice() == t.as_slice());
        }
        assert!(input.is_empty());
    }
}

// ---------------------------------------------------------------- scan / parse

#[test]
fn scan_macro() {
    use algo_lib::io::scan::Parse;
    use algo_lib::scan;
    for text in ["move 3 from 10 to -5\nname: bob\n", "move 3 from 10 to -5\r\nname: bob"] {
        for (k, mut input) in inputs(text.as_bytes()).into_iter().enumerate() {
            scan!(&mut input, "move @ from @ to @\nname: @", a: usize, b: u64, c: i32, d: Str);
            assert_eq!((a, b, c), (3, 10, -5), "{k}");
            assert_eq!(d.as_slice(), b"bob", "{k}");
            assert!(input.is_exhausted());
        }
    }
    assert_eq!(b"-123"[..].parse::<i32>(), -123);
    assert_eq!(b"18446744073709551615"[..].parse::<u64>(), u64::MAX);
    assert_eq!(b"abc"[..].parse::<Str>().as_slice(), b"abc");
}

// ---------------------------------------------------------------- tester checkers

#[test]
fn tester_checkers() {
    use tester::classic::{default_checker, default_checker_eps_abs, default_checker_eps_rel};
    let run = |f: fn(Input, Option<Input>, Input) -> Result<Option<i64>, String>, e: &str, a: &str| {
        f(Input::slice(b""), Some(Input::slice(e.as_bytes())), Input::slice(a.as_bytes())).is_ok()
    };
    assert!(run(default_checker, "1 2\n3\n", "1\r\n2 3"));
    assert!(run(default_checker, "", "  \n"));
    assert!(!run(default_checker, "1 2 3", "1 2"));
    assert!(!run(default_checker, "1 2", "1 2 3"));
    assert!(!run(default_checker, "1 2", "1 3"));
    assert!(!run(default_checker, "12", "1 2"));
    assert!(!run(default_checker, "yes", "YES"));
    assert!(!run(default_checker, "1.0", "1.00"));
    assert!(run(default_checker_eps_abs, "1.0 abc", "1.0000000001 abc"));
    assert!(!run(default_checker_eps_abs, "1.0", "1.00001"));
    assert!(!run(default_checker_eps_abs, "1.0", "nan"));
    assert!(!run(default_checker_eps_abs, "1.0", "inf"));
    assert!(!run(default_checker_eps_abs, "1.0 2", "1.0"));
    assert!(!run(default_checker_eps_abs, "1.0", "1.0 2"));
    assert!(!run(default_checker_eps_abs, "1e12", "1000000000000.1"));
    assert!(run(default_checker_eps_rel, "1e12", "1000000000000.1"));
    assert!(!run(default_checker_eps_rel, "1e12", "1000000002000"));
    assert!(!run(default_checker_eps_rel, "abc", "abd"));
    assert!(!run(default_checker_eps_rel, "0", "1e-8"));
    assert!(run(default_checker_eps_rel, "0", "-1e-10"));
}

// ---------------------------------------------------------------- fix verification on copies

fn read_line_fixed(input: &mut Input) -> Vec<u8> {
    let mut res = Vec::new();
    while let Some(c) = input.get() {
        if c == b'\n' {
            break;
        }
        res.push(c);
    }
    res
}

#[test]
fn read_line_fix_verification() {
    for text in ["a b\n\nc d \r\n\r\nlast\n", "a b\r\n\r\nc d \n\nlast", "a b\r\n\r\nc d \n\nlast\r"] {
        for (k, mut input) in inputs(text.as_bytes()).into_iter().enumerate() {
            assert_eq!(read_line_fixed(&mut input), b"a b", "{k}");
            assert_eq!(read_line_fixed(&mut input), b"", "{k}");
            assert_eq!(read_line_fixed(&mut input), b"c d ", "{k}");
            assert_eq!(read_line_fixed(&mut input), b"", "{k}");
            assert_eq!(read_line_fixed(&mut input), b"last", "{k}");
            assert!(input.is_exhausted(), "{k}");
        }
    }
}

/// Reduced copy of `Input` (delegate source, i64 only) with the proposed
/// `ensure_slow` fix: do not block for more data when the buffered data
/// already ends with a whitespace terminator.
struct MiniInput {
    reader: Box<dyn Read + Send>,
    buf: Vec<u8>,
    at: usize,
    buf_read: usize,
}

const SLACK: usize = 64;
const SWAR_ZEROS: u64 = 0x3030_3030_3030_3030;
const SWAR_LOW7: u64 = 0x7f7f_7f7f_7f7f_7f7f;
const SWAR_HIGH: u64 = 0x8080_8080_8080_8080;
const POW10: [u64; 9] = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000];

fn swar_digits(x: u64) -> u64 {
    let x = (x * 10 + (x >> 8)) & 0x00ff_00ff_00ff_00ff;
    let x = (x * 100 + (x >> 16)) & 0x0000_ffff_0000_ffff;
    (x * 10_000 + (x >> 32)) & 0xffff_ffff
}

impl MiniInput {
    fn new(reader: impl Read + Send + 'static, cap: usize) -> Self {
        Self {
            reader: Box::new(reader),
            buf: vec![0; cap + SLACK],
            at: 0,
            buf_read: 0,
        }
    }
    fn read_more(&mut self) -> usize {
        let cap = self.buf.len() - SLACK;
        let read = self.reader.read(&mut self.buf[self.buf_read..cap]).unwrap();
        self.buf_read += read;
        let end = (self.buf_read + SLACK).min(self.buf.len());
        self.buf[self.buf_read..end].fill(0);
        read
    }
    fn ensure_slow(&mut self, n: usize) {
        self.buf.copy_within(self.at..self.buf_read, 0);
        self.buf_read -= self.at;
        self.at = 0;
        // FIX: stop as soon as the data ends with ' ', '\t' or '\n'
        while self.buf_read < n
            && (self.buf_read == 0 || {
                let last = self.buf[self.buf_read - 1];
                last > b' ' || last == b'\r'
            })
            && self.read_more() != 0
        {}
        let end = (self.buf_read + SLACK).min(self.buf.len());
        self.buf[self.buf_read..end].fill(0);
    }
    fn refill_buffer(&mut self) -> bool {
        if self.at == self.buf_read {
            self.at = 0;
            self.buf_read = 0;
            self.read_more() != 0
        } else {
            true
        }
    }
    fn read_i64(&mut self) -> i64 {
        let mut at = self.at;
        loop {
            while at < self.buf_read && self.buf[at] <= b' ' {
                at += 1;
            }
            self.at = at;
            if at < self.buf_read {
                break;
            }
            assert!(self.refill_buffer(), "read past EOF"); // FIX for the stale `at`
            at = self.at;
        }
        let first = self.buf[at];
        let neg = first == b'-';
        if neg || first == b'+' {
            at += 1;
        }
        let mut res: u64 = 0;
        loop {
            if self.buf_read < at + 10 {
                self.at = at;
                if self.buf_read - self.at < 10 {
                    self.ensure_slow(10);
                }
                at = self.at;
            }
            let chunk = u64::from_le_bytes(self.buf[at..at + 8].try_into().unwrap());
            let x = chunk ^ SWAR_ZEROS;
            let non_digit = (((x & SWAR_LOW7) + 0x7676_7676_7676_7676) | x) & SWAR_HIGH;
            if non_digit == 0 {
                res = res.wrapping_mul(100_000_000).wrapping_add(swar_digits(x));
                at += 8;
            } else {
                let k = (non_digit.trailing_zeros() / 8) as usize;
                if k != 0 {
                    let v = swar_digits(x << (64 - 8 * k));
                    res = res.wrapping_mul(POW10[k]).wrapping_add(v);
                    at += k;
                }
                break;
            }
        }
        if at < self.buf_read {
            let c = self.buf[at];
            if c <= b' ' {
                at += 1;
                if c == b'\r' && at < self.buf_read && self.buf[at] == b'\n' {
                    at += 1;
                }
            }
        }
        self.at = at;
        if neg {
            res.wrapping_neg() as i64
        } else {
            res as i64
        }
    }
}

#[test]
fn interactive_fix_verification() {
    // interactive: several short messages, each answered before the next arrives
    let (snd, rcv) = std::sync::mpsc::channel();
    let (res_snd, res_rcv) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        let mut input = MiniInput::new(ChannelReader { rcv, cur: Vec::new(), at: 0 }, 1 << 16);
        for _ in 0..7 {
            res_snd.send(input.read_i64()).unwrap();
        }
    });
    let timeout = std::time::Duration::from_secs(2);
    snd.send(b"5\n".to_vec()).unwrap();
    assert_eq!(res_rcv.recv_timeout(timeout), Ok(5));
    snd.send(b"-17 42\n".to_vec()).unwrap();
    assert_eq!(res_rcv.recv_timeout(timeout), Ok(-17));
    assert_eq!(res_rcv.recv_timeout(timeout), Ok(42));
    snd.send(b"1234567890123 7\r\n".to_vec()).unwrap();
    assert_eq!(res_rcv.recv_timeout(timeout), Ok(1234567890123));
    assert_eq!(res_rcv.recv_timeout(timeout), Ok(7));
    // number split over two messages
    snd.send(b"  -12".to_vec()).unwrap();
    snd.send(b"34\n".to_vec()).unwrap();
    assert_eq!(res_rcv.recv_timeout(timeout), Ok(-1234));
    snd.send(b"9".to_vec()).unwrap();
    drop(snd);
    assert_eq!(res_rcv.recv_timeout(timeout), Ok(9));
    handle.join().unwrap();

    // and the chunked differential test still passes with the fix (tiny capacity too)
    let mut rng = StdRng::seed_from_u64(7);
    for _ in 0..3000 {
        let n = rng.gen_range(1..40);
        let vals: Vec<i64> = (0..n)
            .map(|_| {
                let x: i64 = rng.gen();
                x >> rng.gen_range(0..64)
            })
            .collect();
        let text = int_doc(&mut rng, &vals);
        for p in patterns() {
            for cap in [32usize, 100, 1 << 12] {
                let mut input = MiniInput::new(Chunked::new(text.as_bytes(), &p), cap);
                for &v in &vals {
                    assert_eq!(input.read_i64(), v, "{text:?} {p:?} {cap}");
                }
            }
        }
    }
}

fn with_precision_fixed(v: f64, precision: usize) -> String {
    let res = format!("{:.*}", precision, v);
    if res.starts_with('-') && !res.bytes().any(|c| (b'1'..=b'9').contains(&c)) {
        res[1..].to_string()
    } else {
        res
    }
}

#[test]
fn with_precision_fix_verification() {
    assert_eq!(with_precision_fixed(-10.0, 6), "-10.000000");
    assert_eq!(with_precision_fixed(-0.0, 6), "0.000000");
    assert_eq!(with_precision_fixed(-1e-9, 6), "0.000000");
    assert_eq!(with_precision_fixed(-1e-9, 0), "0");
    assert_eq!(with_precision_fixed(-100.05, 2), "-100.05");
}

#[test]
fn read_int_on_empty_input_panics() {
    // Before the SWAR rewrite this was `get().unwrap()` -> clean panic.
    let res = std::panic::catch_unwind(|| Input::slice(b"").read_int());
    assert!(res.is_err(), "read_int on empty input returned {:?}", res);
}
