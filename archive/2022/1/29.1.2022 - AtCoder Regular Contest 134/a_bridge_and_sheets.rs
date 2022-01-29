//{"name":"A - Bridge and Sheets","group":"AtCoder - AtCoder Regular Contest 134","url":"https://atcoder.jp/contests/arc134/tasks/arc134_a","interactive":false,"timeLimit":2000,"tests":[{"input":"2 10 3\n3 5\n","output":"2\n"},{"input":"5 10 3\n0 1 4 6 7\n","output":"0\n"},{"input":"12 1000000000 5\n18501490 45193578 51176297 126259763 132941437 180230259 401450156 585843095 614520250 622477699 657221699 896711402\n","output":"199999992\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ABridgeAndSheets"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::iter::once;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let l = input.read_long();
    let w = input.read_long();
    let a = input.read_long_vec(n);

    let mut cur = 0;
    let mut ans = 0;
    for i in a.into_iter().chain(once(l)) {
        if i > cur {
            ans += (i - cur - 1) / w + 1;
        }
        cur = i + w;
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
