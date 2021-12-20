use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::{Input, Readable};
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

    let map: Str = inp.read();
    let mut grid = Vec::new();

    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        grid.push(inp.read::<Str>());
    }

    let mut empty = 0usize;
    for _ in 0..2 {
        let mut next_grid = Vec::new();
        let n = grid.len();
        let m = grid[0].len();
        for i in 0..(n + 2) {
            let mut row = Str::new();
            for j in 0..(m + 2) {
                let mut index = 0;
                for dr in 0..3 {
                    for dc in 0..3 {
                        index *= 2;
                        let row = i + dr;
                        let col = j + dc;
                        if row < 2 || row >= n + 2 || col < 2 || col >= m + 2 {
                            index += empty;
                        } else {
                            index += if grid[row - 2][col - 2] == b'#' { 1 } else { 0 };
                        }
                    }
                }
                row += map[index];
            }
            next_grid.push(row);
        }
        grid = next_grid;
        empty = if map[511 * empty] == b'#' { 1 } else { 0 };
    }

    let mut ans = 0;
    for row in grid {
        for c in row {
            if c == b'#' {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
