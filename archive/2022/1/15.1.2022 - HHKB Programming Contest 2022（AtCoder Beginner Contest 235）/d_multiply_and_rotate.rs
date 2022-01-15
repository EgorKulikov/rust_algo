//{"name":"D - Multiply and Rotate","group":"AtCoder - HHKB Programming Contest 2022（AtCoder Beginner Contest 235）","url":"https://atcoder.jp/contests/abc235/tasks/abc235_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 72\n","output":"4\n"},{"input":"2 5\n","output":"-1\n"},{"input":"2 611\n","output":"12\n"},{"input":"2 767090\n","output":"111\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMultiplyAndRotate"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::number_ext::{NumDigs, Power};
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input) {
    let a = input.read_usize();
    let n = input.read_usize();

    let mut ans = vec![None; 10.power(n.num_digs())];
    ans[1] = Some(0);
    let mut queue = VecDeque::new();
    queue.push_back(1);
    while let Some(i) = queue.pop_front() {
        let cur = ans[i].unwrap();
        if i * a < ans.len() && ans[i * a].is_none() {
            ans[i * a] = Some(cur + 1);
            queue.push_back(i * a);
        }
        let ten = 10.power(i.num_digs());
        let next = i / 10 + (i % 10) * (ten / 10);
        if i % 10 != 0 && next < ans.len() && ans[next].is_none() {
            ans[next] = Some(cur + 1);
            queue.push_back(next);
        }
    }
    out_line!(ans[n]);
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
