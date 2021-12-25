use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    let mut zeros = vec![0usize; 32];
    let mut ones = vec![0usize; 32];
    let mut len = 0;
    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        let cur: String = inp.read();
        len.maxim(cur.len());
        for (i, d) in cur.chars().rev().enumerate() {
            if d == '0' {
                zeros[i] += 1;
            } else {
                ones[i] += 1;
            }
        }
    }
    let mut gamma = 0u64;
    let mut eps = 0u64;
    for i in 0..len {
        if zeros[i] > ones[i] {
            eps += 1u64 << i;
        } else {
            gamma += 1u64 << i;
        }
    }
    println!("{}", gamma * eps);
}
