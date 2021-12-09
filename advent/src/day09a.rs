use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::{Input, Readable};
use algo_lib::misc::dirs::D4;
use algo_lib::string::string::Str;

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

    let mut map = Vec::new();

    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }

        map.push(inp.read::<Str>());
    }

    let mut ans = 0usize;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut low = true;
            for (r, c) in D4::iter(i, j, map.len(), map[i].len()) {
                if map[i][j] >= map[r][c] {
                    low = false;
                    break;
                }
            }
            if low {
                ans += (map[i][j] - b'0' + 1) as usize;
            }
        }
    }

    println!("{}", ans);
}
