use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::{Input, Readable};

pub trait CommaList {
    fn read_list<T: Readable>(&mut self) -> Vec<T>;
}

impl CommaList for Input<'_> {
    fn read_list<T: Readable>(&mut self) -> Vec<T> {
        let mut s: String = self.read();
        s = s.replace(",", " ");
        let mut b = s.as_bytes();
        let input = Input::new(&mut b);
        input.into_iter().collect_vec()
    }
}

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    enum Number {
        Plain(u32),
        Complex(Box<Number>, Box<Number>),
    }

    impl Number {
        pub fn read(inp: &mut Input) -> Self {
            let val = inp.read::<char>();
            if val == '[' {
                let left = Box::new(Self::read(inp));
                inp.read::<char>();
                let right = Box::new(Self::read(inp));
                inp.read::<char>();
                Self::Complex(left, right)
            } else {
                Self::Plain(((val as u8) - b'0') as u32)
            }
        }

        fn split(&mut self) -> bool {
            match self {
                Number::Plain(val) => {
                    if *val >= 10 {
                        let val = *val;
                        *self = Self::Complex(
                            Box::new(Self::Plain(val / 2)),
                            Box::new(Self::Plain((val + 1) / 2)),
                        );
                        true
                    } else {
                        false
                    }
                }
                Number::Complex(left, right) => {
                    if left.split() {
                        true
                    } else if right.split() {
                        true
                    } else {
                        false
                    }
                }
            }
        }

        fn is_plain(&self) -> Option<u32> {
            match self {
                Number::Plain(val) => Some(*val),
                Number::Complex(_, _) => None,
            }
        }

        fn add_to_leftmost(&mut self, value: u32) {
            match self {
                Number::Plain(val) => {
                    *val += value;
                }
                Number::Complex(left, _) => {
                    left.add_to_leftmost(value);
                }
            }
        }

        fn add_to_rightmost(&mut self, value: u32) {
            match self {
                Number::Plain(val) => {
                    *val += value;
                }
                Number::Complex(_, right) => {
                    right.add_to_rightmost(value);
                }
            }
        }

        fn explode(&mut self, height: usize) -> Option<(Option<u32>, Option<u32>)> {
            match self {
                Number::Plain(_) => None,
                Number::Complex(left, right) => {
                    if height >= 4 {
                        let l = left.is_plain();
                        let r = right.is_plain();
                        if l.is_some() && r.is_some() {
                            let res = Some((l, r));
                            *self = Self::Plain(0);
                            return res;
                        }
                    }
                    if let Some((l, r)) = left.explode(height + 1) {
                        if r.is_some() {
                            right.add_to_leftmost(r.unwrap());
                        }
                        return Some((l, None));
                    }
                    if let Some((l, r)) = right.explode(height + 1) {
                        if l.is_some() {
                            left.add_to_rightmost(l.unwrap());
                        }
                        return Some((None, r));
                    }
                    None
                }
            }
        }

        pub fn reduce(&mut self) {
            loop {
                if self.explode(0).is_some() {
                    // println!("Explode {}", self.to_string());
                    continue;
                }
                if self.split() {
                    // println!("Split {}", self.to_string());
                    continue;
                }
                break;
            }
        }

        pub fn add(left: Self, right: Self) -> Self {
            let mut res = Self::Complex(left.into(), right.into());
            res.reduce();
            res
        }

        pub fn magnitude(&self) -> u32 {
            match self {
                Number::Plain(val) => *val,
                Number::Complex(left, right) => left.magnitude() * 3 + right.magnitude() * 2,
            }
        }

        pub fn to_string(&self) -> String {
            match self {
                Number::Plain(val) => val.to_string(),
                Number::Complex(left, right) => {
                    format!("[{},{}]", left.to_string(), right.to_string())
                }
            }
        }

        pub fn clone(&self) -> Self {
            match self {
                Number::Plain(val) => Number::Plain(*val),
                Number::Complex(left, right) => {
                    Number::Complex(left.clone().into(), right.clone().into())
                }
            }
        }
    }

    let mut res = Number::read(&mut inp);
    res.reduce();
    loop {
        inp.skip_whitespace();
        if inp.is_exhausted() {
            break;
        }
        res = Number::add(res, Number::read(&mut inp));
    }
    println!("{}", res.to_string());
    println!("{}", res.magnitude());
}
