use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::{Input, Readable};
use algo_lib::misc::dirs::D4;
use algo_lib::string::string::Str;
use std::collections::VecDeque;

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

    let n = map.len();
    let m = map[0].len();
    let mut sizes = Vec::new();
    let mut visited = Arr2d::new(n, m, false);

    for i in 0..n {
        for j in 0..m {
            if visited[(i, j)] || map[i][j] == b'9' {
                continue;
            }
            let mut cur = 1usize;
            let mut q = VecDeque::new();
            q.push_back((i, j));
            visited[(i, j)] = true;
            while let Some((r, c)) = q.pop_front() {
                for (nr, nc) in D4::iter(r, c, n, m) {
                    if !visited[(nr, nc)] && map[nr][nc] != b'9' {
                        cur += 1;
                        visited[(nr, nc)] = true;
                        q.push_back((nr, nc));
                    }
                }
            }
            sizes.push(cur);
        }
    }

    let mut ans = 1usize;
    sizes.sort();
    sizes.reverse();
    for i in sizes.into_iter().take(3) {
        ans *= i;
    }

    println!("{}", ans);
}
