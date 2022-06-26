//{"name":"C - Robot Takahashi","group":"AtCoder - NS Solutions Corporation Programming Contest 2022（AtCoder Beginner Contest 257）","url":"https://atcoder.jp/contests/abc257/tasks/abc257_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n10101\n60 45 30 40 80\n","output":"4\n"},{"input":"3\n000\n1 2 3\n","output":"3\n"},{"input":"5\n10101\n60 50 50 50 60\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRobotTakahashi"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let s: Str = input.read();
    let w = input.read_int_vec(n);

    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&id| w[id]);
    let mut current = s.iter().filter(|&c| c == b'1').count();
    let mut ans = current;
    let mut last_w = 0;
    for i in order {
        if w[i] != last_w {
            ans.maxim(current);
        }
        last_w = w[i];
        if s[i] == b'0' {
            current += 1;
        } else {
            current -= 1;
        }
    }
    ans.maxim(current);
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
