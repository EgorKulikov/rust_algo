//{"name":"D - Union of Interval","group":"AtCoder - Tokio Marine & Nichido Fire Insurance Programming Contest 2022（AtCoder Beginner Contest 256)","url":"https://atcoder.jp/contests/abc256/tasks/abc256_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n10 20\n20 30\n40 50\n","output":"10 30\n40 50\n"},{"input":"3\n10 40\n30 60\n20 50\n","output":"10 60\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DUnionOfInterval"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let mut segs = input.read_usize_pair_vec(n);

    segs.sort_unstable();
    let mut start = 0;
    let mut end = 0;
    let mut ans = Vec::new();
    for (f, t) in segs {
        if f > end {
            if start != 0 {
                ans.push((start, end));
            }
            start = f;
        }
        end.maxim(t);
    }
    ans.push((start, end));
    output().print_per_line(&ans);
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
