//{"name":"B - Batters","group":"AtCoder - Tokio Marine & Nichido Fire Insurance Programming Contest 2022（AtCoder Beginner Contest 256)","url":"https://atcoder.jp/contests/abc256/tasks/abc256_b","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 1 3 2\n","output":"3\n"},{"input":"3\n1 1 1\n","output":"0\n"},{"input":"10\n2 2 4 1 1 1 4 2 2 1\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBatters"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    let mut ans = vec![0];
    for i in a {
        *ans.last_mut().unwrap() += 1;
        for _ in 0..i {
            ans.push(0);
        }
    }
    out_line!(ans.into_iter().rev().skip(4).sum::<i32>());
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
