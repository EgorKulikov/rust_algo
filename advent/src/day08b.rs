use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::{Input, Readable};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::string::string::Str;
use std::collections::HashMap;

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

    let mut ans = 0usize;

    let convert = |s: &Str| {
        let mut res = 0u8;
        for c in s.iter() {
            res.set_bit((c - b'a') as usize);
        }
        res
    };

    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        let mut base = inp.read_vec::<Str>(10);
        let mut map = HashMap::new();
        let mut v = [0u8; 10];
        for s in base.iter() {
            let val = convert(s);
            if s.len() == 2 {
                map.insert(val, 1);
                v[1] = val;
            } else if s.len() == 3 {
                map.insert(val, 7);
                v[7] = val;
            } else if s.len() == 4 {
                map.insert(val, 4);
                v[4] = val;
            } else if s.len() == 7 {
                map.insert(val, 8);
                v[8] = val;
            }
        }

        for s in base.iter() {
            let val = convert(s);
            if s.len() == 5 && (val & v[1]) == v[1] {
                map.insert(val, 3);
                v[3] = val;
            }
            if s.len() == 6 && (val & v[4]) == v[4] {
                map.insert(val, 9);
                v[9] = val;
            }
        }

        for s in base.iter() {
            let val = convert(s);
            if s.len() == 6 && (val & v[1]) == v[1] && map.get(&val) == None {
                map.insert(val, 0);
                v[0] = val;
            }
            if s.len() == 5 && (val & v[9]) == val && map.get(&val) == None {
                map.insert(val, 5);
                v[5] = val;
            }
        }

        for s in base.iter() {
            let val = convert(s);
            if s.len() == 6 && map.get(&val) == None {
                map.insert(val, 6);
                v[6] = val;
            }
            if s.len() == 5 && map.get(&val) == None {
                map.insert(val, 2);
                v[2] = val;
            }
        }

        inp.next_token();
        let mut cur = 0usize;
        for _ in 0..4 {
            let s: Str = inp.read();
            let val = convert(&s);
            cur *= 10;
            cur += map[&val] as usize;
        }
        ans += cur;
    }
    // for _ in 0..80 {
    //     vals.push(vals.len());
    // }
    println!("{}", ans);
}
