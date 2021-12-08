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
        for c in s {
            res.set_bit(c - b'a');
        }
        res
    };

    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }

        for _ in 0..=10 {
            inp.next_token();
        }

        for _ in 0..4 {
            let s: Str = inp.read();
            let l = s.len();
            if l == 2 || l == 4 || l == 3 || l == 7 {
                ans += 1;
            }
        }
    }
    // for _ in 0..80 {
    //     vals.push(vals.len());
    // }
    println!("{}", ans);
}
