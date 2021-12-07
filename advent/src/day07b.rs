use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
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

    let mut vals: Vec<i64> = inp.read_list();
    // for _ in 0..80 {
    //     vals.push(vals.len());
    // }
    let n = vals.len();
    vals.sort();
    let to = vals[n - 1];
    let mut ans = i64::MAX;
    for i in 0..to {
        let mut cand = 0i64;
        for j in vals.iter() {
            cand += (i - j).abs() * ((i - j).abs() + 1) / 2;
        }
        ans.minim(cand);
    }
    println!("{}", ans);
}
