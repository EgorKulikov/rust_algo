use algo_lib::io::input::Input;

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    let mut ans = 0usize;
    let mut last: u16 = inp.read();
    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        let cur: u16 = inp.read();
        if cur > last {
            ans += 1;
        }
        last = cur;
    }
    println!("{}", ans);
}
