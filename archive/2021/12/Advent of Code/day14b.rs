use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::{Input, Readable};
use algo_lib::string::string::Str;
use std::mem::swap;

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

    let start: Str = inp.read();
    let mut insertion = Arr2d::new(26, 26, 26u8);
    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        let pair: Str = inp.read();
        assert_eq!(pair.len(), 2);
        inp.next_token();
        let target: char = inp.read();
        insertion[((pair[0] - b'A') as usize, (pair[1] - b'A') as usize)] = (target as u8) - b'A';
    }

    let mut letters = vec![0u64; 26];
    let mut state = Arr2d::new(26, 26, 0u64);
    let mut last = 26;
    for c in start.iter() {
        letters[(c - b'A') as usize] += 1;
        if last != 26 {
            state[(last as usize, (c - b'A') as usize)] += 1;
        }
        last = c - b'A';
    }
    let mut next = Arr2d::new(26, 26, 0u64);
    for _ in 0..40 {
        next.fill(0);
        for i in 0usize..26 {
            for j in 0usize..26 {
                if insertion[(i, j)] != 26 {
                    next[(i, insertion[(i, j)] as usize)] += state[(i, j)];
                    next[(insertion[(i, j)] as usize, j)] += state[(i, j)];
                    letters[insertion[(i, j)] as usize] += state[(i, j)];
                }
            }
        }
        swap(&mut state, &mut next);
    }
    println!(
        "{}",
        letters.iter().max().unwrap() - letters.iter().filter(|v| **v != 0).min().unwrap()
    );
}
