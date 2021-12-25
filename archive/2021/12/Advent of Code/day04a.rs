use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::iter_ext::{IterExt, IterPartialEqExt};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    let mut vals = Vec::new();
    let n: String = inp.read();
    let n = n.replace(",", " ");
    let n = n.as_bytes().iter().cloned().collect_vec();
    let mut x = &n[..];
    let mut in1 = Input::new(&mut x);
    loop {
        in1.skip_whitespace();
        if in1.peek().is_none() {
            break;
        }
        let cur: usize = in1.read();
        vals.push(cur);
    }
    let mut after = vals.len();
    let mut score = 0usize;
    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        let cur = inp.read_table::<usize>(5, 5);
        let mut cand = vals.len();
        for i in 0..5 {
            let mut row_finish = 0usize;
            let mut col_finish = 0usize;
            for j in 0..5 {
                let v = cur[(i, j)];
                let pos = vals.iter().find(&v).unwrap();
                row_finish.maxim(pos);
                let v = cur[(j, i)];
                let pos = vals.iter().find(&v).unwrap();
                col_finish.maxim(pos);
            }
            let finish = row_finish.min(col_finish);
            cand.minim(finish);
        }
        if cand < after {
            after = cand;
            let last = vals[after];
            let mut sum = 0usize;
            for i in 0..5 {
                for j in 0..5 {
                    let v = cur[(i, j)];
                    let pos = vals.iter().find(&v).unwrap();
                    if pos > after {
                        sum += v;
                    }
                }
            }
            score = last * sum;
        }
    }
    println!("{}", score);
}
