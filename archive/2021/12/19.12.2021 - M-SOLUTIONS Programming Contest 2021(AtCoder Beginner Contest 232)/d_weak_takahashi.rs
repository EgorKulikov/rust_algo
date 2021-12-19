//{"name":"D - Weak Takahashi","group":"AtCoder - M-SOLUTIONS Programming Contest 2021(AtCoder Beginner Contest 232)","url":"https://atcoder.jp/contests/abc232/tasks/abc232_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n.#..\n..#.\n..##\n","output":"4\n"},{"input":"1 1\n.\n","output":"1\n"},{"input":"5 5\n.....\n.....\n.....\n.....\n.....\n","output":"9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DWeakTakahashi"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let h = input.read();
    let w = input.read();
    let c = input.read_table::<char>(h, w);

    let mut visited = Arr2d::new(h, w, false);
    visited[(0, 0)] = true;
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if !visited[(i, j)] {
                continue;
            }
            ans.maxim(i + j + 1);
            if i + 1 < h && c[(i + 1, j)] == '.' {
                visited[(i + 1, j)] = true;
            }
            if j + 1 < w && c[(i, j + 1)] == '.' {
                visited[(i, j + 1)] = true;
            }
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
