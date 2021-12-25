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
        let mut input = Input::new(&mut b);
        input.into_iter().collect_vec()
    }
}

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    let mut vals: Vec<usize> = inp.read_list();
    // for _ in 0..80 {
    //     vals.push(vals.len());
    // }
    vals.sort();
    let mut ans = 0;
    let n = vals.len();
    for i in 0..n / 2 {
        ans += vals[n / 2] - vals[i];
    }
    for i in n / 2..n {
        ans += vals[i] - vals[n / 2];
    }
    println!("{}", vals.len());
}
