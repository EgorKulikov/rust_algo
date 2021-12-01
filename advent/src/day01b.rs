use algo_lib::io::input::Input;

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    let mut ans = 0usize;
    let mut last = Vec::new();
    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        let cur: u16 = inp.read();
        if last.len() >= 3 && last[last.len() - 3] < cur {
            ans += 1;
        }
        last.push(cur);
    }
    println!("{}", ans);
}
