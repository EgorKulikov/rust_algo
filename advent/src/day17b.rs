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
        let input = Input::new(&mut b);
        input.into_iter().collect_vec()
    }
}

fn main() {
    let mut sin = std::io::stdin();
    let mut inp = Input::new(&mut sin);

    inp.next_token();
    inp.next_token();
    let mut read_range = || -> (i32, i32) {
        let s: String = inp.read();
        let tokens = s.split("=").collect_vec();
        assert_eq!(tokens.len(), 2);
        let s = tokens[1].to_owned().replace(",", "");
        let tokens = s.split("..").collect_vec();
        assert_eq!(tokens.len(), 2);
        (tokens[0].parse().unwrap(), tokens[1].parse().unwrap())
    };
    let (x_from, x_to) = read_range();
    let (y_from, y_to) = read_range();

    let mut ans = 0;
    for x in 1..=x_to {
        for y in y_from..=(-y_from) {
            let mut cx = 0;
            let mut cy = 0;
            let mut dx = x;
            let mut dy = y;
            loop {
                cx += dx;
                cy += dy;
                dx = 0.max(dx - 1);
                dy -= 1;
                if cx > x_to || cy < y_from || cx < x_from && dx == 0 {
                    break;
                }
                if (x_from..=x_to).contains(&cx) && (y_from..=y_to).contains(&cy) {
                    ans += 1;
                    break;
                }
            }
        }
    }
    println!("{}", ans);
}
