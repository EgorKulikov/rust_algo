use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    let mut map = Arr2d::new(1000, 1000, 0usize);
    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        let cur: String = inp.read();
        let from = cur
            .split(",")
            .map(|s| s.to_string().parse::<usize>().unwrap())
            .collect_vec();
        let x0 = from[0];
        let y0 = from[1];
        inp.next_token();
        let cur: String = inp.read();
        let to = cur
            .split(",")
            .map(|s| s.to_string().parse::<usize>().unwrap())
            .collect_vec();
        let x1 = to[0];
        let y1 = to[1];
        if x0 == x1 {
            for i in y0.min(y1)..=y0.max(y1) {
                map[(x0, i)] += 1;
            }
        } else if y0 == y1 {
            for i in x0.min(x1)..=x0.max(x1) {
                map[(i, y0)] += 1;
            }
        } else if x1 as isize - x0 as isize == y1 as isize - y0 as isize {
            for i in x0.min(x1)..=x0.max(x1) {
                map[(i, i - x0.min(x1) + y0.min(y1))] += 1;
            }
        } else {
            for i in x0.min(x1)..=x0.max(x1) {
                map[(i, x0.max(x1) - i + y0.min(y1))] += 1;
            }
        }
    }
    let res = map.iter().filter(|v| **v >= 2).count();
    println!("{}", res);
}
