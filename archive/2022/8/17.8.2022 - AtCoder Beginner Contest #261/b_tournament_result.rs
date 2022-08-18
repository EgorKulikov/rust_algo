//{"name":"B - Tournament Result","group":"AtCoder - AtCoder Beginner Contest 261","url":"https://atcoder.jp/contests/abc261/tasks/abc261_b","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n-WWW\nL-DD\nLD-W\nLDW-\n","output":"incorrect\n"},{"input":"2\n-D\nD-\n","output":"correct\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTournamentResult"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let a = input.read_table::<char>(n, n);

    for i in 0..n {
        for j in 0..i {
            let target = match a[(i, j)] {
                'W' => 'L',
                'L' => 'W',
                'D' => 'D',
                _ => unreachable!(),
            };
            if a[(j, i)] != target {
                out_line!("incorrect");
                return;
            }
        }
    }
    out_line!("correct");
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
