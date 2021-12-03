use algo_lib::io::input::Input;

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    let mut vals = Vec::new();
    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        let cur: String = inp.read();
        vals.push(cur.parse::<u64>().unwrap());
    }
    println!("{}", vals.len());
}
