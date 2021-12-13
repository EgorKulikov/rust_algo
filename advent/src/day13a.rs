use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::{Input, Readable};
use std::collections::HashSet;

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

    let mut dots = Vec::new();
    loop {
        let string = inp.read_line();
        if string.is_empty() {
            break;
        }
        let tokens = string.split(",").collect_vec();
        assert!(tokens.len() == 2);
        let x = tokens[0].parse::<i32>().unwrap();
        let y = tokens[1].parse::<i32>().unwrap();
        dots.push((x, y));
    }

    inp.next_token();
    inp.next_token();
    let fold: String = inp.read();
    let tokens = fold.split("=").collect_vec();
    assert!(tokens.len() == 2);
    let at = tokens[1].parse::<i32>().unwrap();
    if tokens[0] == "x" {
        for (x, _) in dots.iter_mut() {
            assert_ne!(*x, at);
            if *x > at {
                *x = 2 * at - *x;
            }
        }
    } else {
        for (_, y) in dots.iter_mut() {
            assert_ne!(*y, at);
            if *y > at {
                *y = 2 * at - *y;
            }
        }
    }

    let ans = dots.into_iter().collect::<HashSet<_>>().len();
    println!("{}", ans);
}
