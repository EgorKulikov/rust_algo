use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::types::recursive_function::{Callable3, RecursiveFunction3};

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    let mut vals = Vec::new();
    let mut len = 0;
    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        let cur: String = inp.read();
        len.maxim(cur.len());
        let mut val = 0u64;
        for (i, d) in cur.chars().rev().enumerate() {
            if d != '0' {
                val += 1u64 << i;
            }
        }
        vals.push(val);
    }
    let mut f = RecursiveFunction3::new(|f, mut step: usize, vals: Vec<u64>, target| -> u64 {
        if vals.len() == 1 {
            return vals[0];
        }
        step -= 1;
        let mut zeros = Vec::new();
        let mut ones = Vec::new();
        for i in vals {
            if (i >> step & 1) == 1 {
                ones.push(i);
            } else {
                zeros.push(i);
            }
        }
        if ones.is_empty() || !zeros.is_empty() && (ones.len() < zeros.len()) == target {
            f.call(step, zeros, target)
        } else {
            f.call(step, ones, target)
        }
    });
    let oxygen = f.call(len, vals.clone(), true);
    let co2 = f.call(len, vals, false);
    println!("{}", oxygen * co2);
}
