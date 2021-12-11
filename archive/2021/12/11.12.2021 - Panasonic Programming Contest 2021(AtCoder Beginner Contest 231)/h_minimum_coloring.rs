//{"name":"H - Minimum Coloring","group":"AtCoder - Panasonic Programming Contest 2021(AtCoder Beginner Contest 231)","url":"https://atcoder.jp/contests/abc231/tasks/abc231_h","interactive":false,"timeLimit":2000,"tests":[{"input":"2 3 6\n1 1 1\n1 2 10\n1 3 100\n2 1 1000\n2 2 10000\n2 3 100000\n","output":"1110\n"},{"input":"1 7 7\n1 2 200000000\n1 7 700000000\n1 4 400000000\n1 3 300000000\n1 6 600000000\n1 5 500000000\n1 1 100000000\n","output":"2800000000\n"},{"input":"3 3 8\n3 2 1\n3 1 2\n2 3 1\n2 2 100\n2 1 100\n1 3 2\n1 2 100\n1 1 100\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HMinimumColoring"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::hungarian_algorithm::hungarian_algorithm;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let h: usize = input.read();
    let w: usize = input.read();
    let n = input.read();
    let cells: Vec<(usize, usize, i64)> = input.read_vec(n);

    let mut rows = vec![std::i64::MAX; h];
    let mut cols = vec![std::i64::MAX; w];
    for (a, b, c) in cells.iter() {
        rows[*a - 1].minim(*c);
        cols[*b - 1].minim(*c);
    }
    let n = h.max(w);
    let mut a = Arr2d::generate(
        n,
        n,
        |i, j| if i < h { rows[i] } else { 0 } + if j < w { cols[j] } else { 0 },
    );
    for (r, b, c) in cells {
        a[(r - 1, b - 1)].minim(c);
    }

    out_line!(hungarian_algorithm(&a));
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
