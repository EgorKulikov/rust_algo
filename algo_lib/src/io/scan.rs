use crate::io::input::Input;
use crate::io::input::Readable;

#[macro_export]
macro_rules! scan {
    ($input: expr, $s: expr, $($v:ident: $t: ty),*) => {
        scan!($input, $s, '@', $($v: $t),*);
    };
    ($input: expr, $s: expr, $sp: expr, $($v:ident: $t: ty),*) => {
        let mut parse = |pattern: &str, special: char| -> std::collections::VecDeque<Vec<u8>> {
            let mut res = std::collections::VecDeque::new();
            let mut last_special = false;

            fn parse_special(input: &mut Input, c: char) -> Vec<u8> {
                let mut cur = Vec::new();
                loop {
                    let next = input.get();
                    if c == '\n' {
                        if let Some(next) = next {
                            match next {
                                b'\r' => {
                                    if input.peek() == Some(b'\n') {
                                        input.get();
                                    }
                                    break;
                                }
                                b'\n' => break,
                                _ => cur.push(next),
                            }
                        } else {
                            break;
                        }
                    } else {
                        let next = next.unwrap();
                        if next == c as u8 {
                            break;
                        } else {
                            cur.push(next);
                        }
                    }
                }
                cur
            }

            for c in pattern.chars() {
                if c == special {
                    assert!(!last_special);
                    last_special = true;
                } else {
                    if last_special {
                        res.push_back(parse_special($input, c));
                    } else {
                        let next = $input.get();
                        if c == '\n' {
                            if let Some(next) = next {
                                if next == b'\r' {
                                    if $input.peek() == Some(b'\n') {
                                        $input.get();
                                    }
                                } else {
                                    assert_eq!(next, b'\n');
                                }
                            }
                        } else {
                            assert_eq!(c as u8, next.unwrap());
                        }
                    }
                    last_special = false;
                }
            }
            if last_special {
                res.push_back(parse_special($input, '\n'));
            }
            res
        };

        let mut res = parse($s, $sp);
        $(
            let cur = res.pop_front().unwrap();
            let mut input = Input::slice(cur.as_slice());
            let $v: $t = input.read();
            assert!(input.is_exhausted());
        )*
    };
}

#[macro_export]
macro_rules! str_scan {
    ($input: expr, $s: expr, $($v:ident: $t: ty),*) => {
        str_scan!($input, $s, '@', $($v: $t),*);
    };
    ($input: expr, $s: expr, $sp: expr, $($v:ident: $t: ty),*) => {
        let mut input = Input::slice($input);
        $crate::scan!(&mut input, $s, $sp, $($v: $t),*);
    };
}

pub trait Parse<T: Readable> {
    fn parse(&self) -> T;
}

impl<T: Readable> Parse<T> for [u8] {
    fn parse(&self) -> T {
        str_scan!(self, "@", x: T);
        x
    }
}
