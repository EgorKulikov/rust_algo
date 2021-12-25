use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    let mut s: String = inp.read();
    s = s.replace(",", " ");
    let mut bytes = s.as_bytes();
    let mut inp = Input::new(&mut bytes);
    let mut vals: Vec<usize> = inp.into_iter().collect_vec();
    let mut qty = vec![0u64; 9];
    for i in vals {
        qty[i] += 1;
    }
    for _ in 0..256 {
        let mut nqty = vec![0u64; 9];
        nqty[6] = qty[0];
        nqty[8] = qty[0];
        for i in 1usize..9 {
            nqty[i - 1] += qty[i];
        }
        qty = nqty;
    }
    let x: u64 = qty.iter().sum();
    println!("{}", x);
}
