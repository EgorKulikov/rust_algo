#![feature(map_first_last)]

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

    let mut grid = Vec::new();
    loop {
        inp.skip_whitespace();
        if inp.peek().is_none() {
            break;
        }
        grid.push(inp.read::<Str>());
    }

    let n = grid.len();
    let m = grid[0].len();
    let mut ans = 0;
    loop {
        let mut update = false;
        for i in 0..n {
            for j in 0..m {
                let nj = (j + 1) % m;
                if grid[i][j] == b'>' && grid[i][nj] == b'.' {
                    grid[i][j] = b'x';
                    grid[i][nj] = b'r';
                    update = true;
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == b'x' {
                    grid[i][j] = b'.';
                }
                if grid[i][j] == b'r' {
                    grid[i][j] = b'>';
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                let ni = (i + 1) % n;
                if grid[i][j] == b'v' && grid[ni][j] == b'.' {
                    grid[i][j] = b'x';
                    grid[ni][j] = b'd';
                    update = true;
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == b'x' {
                    grid[i][j] = b'.';
                }
                if grid[i][j] == b'd' {
                    grid[i][j] = b'v';
                }
            }
        }
        ans += 1;
        if !update {
            break;
        }
    }
    println!("{}", ans);
}
