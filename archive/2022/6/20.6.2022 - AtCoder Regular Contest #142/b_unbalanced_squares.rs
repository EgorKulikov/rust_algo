//{"name":"B - Unbalanced Squares","group":"AtCoder - AtCoder Regular Contest 142","url":"https://atcoder.jp/contests/arc142/tasks/arc142_b","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n","output":"1 2\n3 4\n"},{"input":"3\n","output":"1 2 3\n5 4 6\n7 8 9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BUnbalancedSquares"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();

    let mut ans = Arr2d::new(n, n, 0);
    let mut next = 1;
    for i in (0..n).step_by(2) {
        for j in 0..n {
            ans[(i, j)] = next;
            next += 1;
        }
    }
    for i in (1..n).step_by(2) {
        for j in 0..n {
            ans[(i, j)] = next;
            next += 1;
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
