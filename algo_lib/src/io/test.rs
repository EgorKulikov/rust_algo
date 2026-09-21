use crate::io::input::Input;
use crate::io::output::Output;
use crate::misc::random::{Random, RandomTrait};
use crate::string::str::StrReader;
use std::io::Read;

/// Reader that hands out at most `chunk` bytes per `read`, to stress refills.
struct Trickle {
    data: Vec<u8>,
    at: usize,
    chunk: usize,
}

impl Read for Trickle {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = buf.len().min(self.chunk).min(self.data.len() - self.at);
        buf[..n].copy_from_slice(&self.data[self.at..self.at + n]);
        self.at += n;
        Ok(n)
    }
}

fn inputs(data: &[u8]) -> Vec<Input> {
    let mut res = vec![Input::slice(data)];
    for chunk in [1, 3, 7, 8, 9, 64, 1 << 20] {
        res.push(Input::delegate(Trickle {
            data: data.to_vec(),
            at: 0,
            chunk,
        }));
    }
    res
}

#[test]
fn integers_all_lengths() {
    let mut text = String::new();
    let mut expected: Vec<i128> = Vec::new();
    let mut rng = Random::new_with_seed(5);
    for digits in 1..=39 {
        for sign in [1i128, -1] {
            let mut v: i128 = 0;
            for _ in 0..digits {
                v = v
                    .checked_mul(10)
                    .and_then(|v| v.checked_add(rng.gen_range(0..10i128)))
                    .unwrap_or(v);
            }
            let v = v * sign;
            expected.push(v);
            let sep = match expected.len() % 5 {
                0 => "\n",
                1 => " ",
                2 => "\r\n",
                3 => "   ",
                _ => "\t",
            };
            text += &format!("{v}{sep}");
        }
    }
    for extreme in [
        i128::MIN,
        i128::MAX,
        i64::MIN as i128,
        i64::MAX as i128,
        0,
        -0,
        u64::MAX as i128,
    ] {
        expected.push(extreme);
        text += &format!("{extreme} ");
    }
    text += "+17";
    expected.push(17);
    for mut input in inputs(text.as_bytes()) {
        for &e in &expected {
            let got: i128 = input.read();
            assert_eq!(got, e);
        }
        assert!(input.is_empty());
    }
}

#[test]
fn integer_types() {
    let text = b"c 65535 4294967295 18446744073709551615 340282366920938463463374607431768211455 -128 -32768 -2147483648 -9223372036854775808 -170141183460469231731687303715884105728 12 0 7";
    for mut input in inputs(text) {
        assert_eq!(input.read::<u8>(), b'c');
        assert_eq!(input.read::<u16>(), 65535);
        assert_eq!(input.read_unsigned(), 4294967295);
        assert_eq!(input.read_u64(), u64::MAX);
        assert_eq!(input.read::<u128>(), u128::MAX);
        assert_eq!(input.read::<i8>(), -128);
        assert_eq!(input.read::<i16>(), -32768);
        assert_eq!(input.read_int(), i32::MIN);
        assert_eq!(input.read_long(), i64::MIN);
        assert_eq!(input.read_i128(), i128::MIN);
        assert_eq!(input.read_size(), 12);
        assert_eq!(input.read::<isize>(), 0);
        assert_eq!(input.read::<usize>(), 7);
        assert!(input.is_empty());
    }
}

#[test]
fn eol_tracking_and_lines() {
    for text in ["1 2\n3 4\r\nabc def\n5\n\n6", "1 2\n3 4\nabc def\n5\n\n6"] {
        for mut input in inputs(text.as_bytes()) {
            assert_eq!(input.read_int(), 1);
            assert!(!input.is_eol());
            assert_eq!(input.read_int(), 2);
            assert!(input.is_eol());
            assert_eq!(input.read_int_pair_vec(1), vec![(3, 4)]);
            assert!(input.is_eol());
            assert_eq!(input.read_line().as_slice(), b"abc def");
            assert_eq!(input.read_int(), 5);
            assert!(input.is_eol());
            assert_eq!(input.read_line().as_slice(), b"");
            assert_eq!(input.read_int(), 6);
            assert!(input.is_eol());
            assert!(input.is_empty());
        }
    }
}

#[test]
fn tokens_chars_and_mixed() {
    let text = b"  hello 42world x\n-7-8 9\n";
    for mut input in inputs(text) {
        assert_eq!(input.next_token(), Some(b"hello".to_vec()));
        assert_eq!(input.read_int(), 42);
        assert_eq!(input.read_str().as_slice(), b"world");
        assert_eq!(input.read_char(), b'x');
        assert_eq!(input.read_int(), -7);
        assert_eq!(input.read_int(), -8);
        assert_eq!(input.read_int(), 9);
        assert!(input.is_empty());
        assert_eq!(input.next_token(), None);
    }
}

#[test]
fn large_input_across_buffers() {
    let n = 1_000_000;
    let mut rng = Random::new_with_seed(9);
    let vals: Vec<i64> = (0..n)
        .map(|_| (rng.gen_u128() as i64) >> rng.gen_range(0..64u32))
        .collect();
    let mut text = String::new();
    for (i, v) in vals.iter().enumerate() {
        text += &v.to_string();
        text += if i % 7 == 6 { "\n" } else { " " };
    }
    for mut input in [
        Input::slice(text.as_bytes()),
        Input::delegate(Trickle {
            data: text.clone().into_bytes(),
            at: 0,
            chunk: 65536,
        }),
    ] {
        let got = input.read_long_vec(n);
        assert_eq!(got, vals);
        assert!(input.is_empty());
    }
}

fn written<T: crate::io::output::Writable>(v: T) -> String {
    let mut buf = Vec::new();
    {
        let mut out = Output::buf(&mut buf);
        out.print(v);
        out.flush();
    }
    String::from_utf8(buf).unwrap()
}

#[test]
fn write_integers() {
    for v in [
        0i128,
        1,
        9,
        10,
        99,
        100,
        12345678,
        123456789,
        i64::MAX as i128,
        i64::MIN as i128,
        u64::MAX as i128,
        i128::MAX,
        i128::MIN,
        -1,
        -10,
        -1000000007,
    ] {
        assert_eq!(written(v), v.to_string());
        if let Ok(x) = i64::try_from(v) {
            assert_eq!(written(x), x.to_string());
        }
        if let Ok(x) = i32::try_from(v) {
            assert_eq!(written(x), x.to_string());
        }
        if let Ok(x) = u64::try_from(v) {
            assert_eq!(written(x), x.to_string());
            assert_eq!(written(x as u128), x.to_string());
        }
        if let Ok(x) = u32::try_from(v) {
            assert_eq!(written(x), x.to_string());
            assert_eq!(written(x as usize), x.to_string());
        }
        if let Ok(x) = u16::try_from(v) {
            assert_eq!(written(x), x.to_string());
        }
        if let Ok(x) = i8::try_from(v) {
            assert_eq!(written(x), x.to_string());
        }
    }
    assert_eq!(written(u128::MAX), u128::MAX.to_string());
    let mut rng = Random::new_with_seed(3);
    for _ in 0..20000 {
        let v = (rng.gen_u128() as i64) >> rng.gen_range(0..64u32);
        assert_eq!(written(v), v.to_string());
        let u = rng.gen_u128() >> rng.gen_range(0..128u32);
        assert_eq!(written(u), u.to_string());
    }
}

#[test]
fn write_many_lines_matches_format() {
    let n = 300_000;
    let mut rng = Random::new_with_seed(4);
    let vals: Vec<(i64, u32)> = (0..n)
        .map(|_| {
            (
                rng.gen_u128() as i64 >> rng.gen_range(0..64u32),
                rng.gen_u128() as u32,
            )
        })
        .collect();
    let mut expected = String::new();
    for &(a, b) in &vals {
        expected += &format!("{a} {b}\n");
    }
    let mut buf = Vec::new();
    {
        let mut out = Output::buf(&mut buf);
        for &(a, b) in &vals {
            out.print_line((a, b));
        }
        out.print_per_line(&vals[..10]);
        out.print_line_iter(vals.iter().map(|p| p.0));
        out.flush();
    }
    for &(a, b) in &vals[..10] {
        expected += &format!("{a} {b}\n");
    }
    expected += &vals
        .iter()
        .map(|p| p.0.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    expected += "\n";
    assert_eq!(String::from_utf8(buf).unwrap(), expected);
}

/// Reader fed through a channel: `read` blocks until the other side sends the
/// next message, like stdin of an interactive problem.
struct Interactive {
    rcv: std::sync::mpsc::Receiver<Vec<u8>>,
    cur: Vec<u8>,
    at: usize,
}

impl Read for Interactive {
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

/// Runs `f` over an input that has received `messages` and nothing else, the
/// sender still being open. `None` means `f` blocked waiting for more data.
fn interactive<T: Send + 'static>(
    messages: &[&[u8]],
    f: impl FnOnce(&mut Input) -> T + Send + 'static,
) -> Option<T> {
    let (snd, rcv) = std::sync::mpsc::channel();
    let (res_snd, res_rcv) = std::sync::mpsc::channel();
    for m in messages {
        snd.send(m.to_vec()).unwrap();
    }
    std::thread::spawn(move || {
        let mut input = Input::delegate(Interactive {
            rcv,
            cur: Vec::new(),
            at: 0,
        });
        let _ = res_snd.send(f(&mut input));
    });
    let res = res_rcv.recv_timeout(std::time::Duration::from_secs(2)).ok();
    drop(snd);
    res
}

#[test]
fn integer_read_does_not_wait_for_data_past_its_terminator() {
    assert_eq!(interactive(&[b"5\n"], |input| input.read_int()), Some(5));
    assert_eq!(
        interactive(&[b"5 1234567890\n"], |input| (
            input.read_int(),
            input.read_long()
        )),
        Some((5, 1234567890))
    );
    assert_eq!(
        interactive(&[b"-7\r\n"], |input| (input.read_int(), input.is_eol())),
        Some((-7, true))
    );
}

#[test]
fn integer_split_across_interactive_messages() {
    assert_eq!(
        interactive(&[b"12", b"34\n"], |input| input.read_int()),
        Some(1234)
    );
    assert_eq!(
        interactive(&[b"1\r", b"\n2\n"], |input| (
            input.read_int(),
            input.is_eol(),
            input.read_int()
        )),
        Some((1, true, 2))
    );
}

#[test]
fn read_line_keeps_last_char_of_unterminated_final_line() {
    for mut input in inputs(b"x y\r\n\nabc") {
        assert_eq!(input.read_line().as_slice(), b"x y");
        assert_eq!(input.read_line().as_slice(), b"");
        assert_eq!(input.read_line().as_slice(), b"abc");
        assert!(input.is_exhausted());
    }
    for mut input in inputs(b"7\nab\n") {
        assert_eq!(input.read_int(), 7);
        assert_eq!(input.read_line().as_slice(), b"ab");
    }
}
