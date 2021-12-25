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

    for _ in 0..4 {
        inp.next_token();
    }
    let mut first: usize = inp.read();
    for _ in 0..4 {
        inp.next_token();
    }
    let mut second: usize = inp.read();

    let mut first_score = 0;
    let mut second_score = 0;
    let mut thrown = 0;
    let mut die = 1;
    let ans = loop {
        for _ in 0..3 {
            first += die % 10;
            if first > 10 {
                first -= 10;
            }
            thrown += 1;
            die += 1;
        }
        first_score += first;
        if first_score >= 1000 {
            break second_score * thrown;
        }
        for _ in 0..3 {
            second += die % 10;
            if second > 10 {
                second -= 10;
            }
            thrown += 1;
            die += 1;
        }
        second_score += second;
        if second_score >= 1000 {
            break first_score * thrown;
        }
    };

    println!("{}", ans);
}
