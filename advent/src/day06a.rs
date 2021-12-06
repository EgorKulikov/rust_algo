use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    let mut s: String = inp.read();
    s = s.replace(",", " ");
    let mut bytes = s.as_bytes();
    let mut inp = Input::new(&mut bytes);
    let mut vals: Vec<u32> = inp.into_iter().collect_vec();
    for _ in 0..80 {
        let mut nvals = Vec::new();
        for i in vals {
            if i == 0 {
                nvals.push(6);
                nvals.push(8);
            } else {
                nvals.push(i - 1);
            }
        }
        vals = nvals;
    }
    println!("{}", vals.len());
}
