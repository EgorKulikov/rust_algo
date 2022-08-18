//{"name":"D - \"redocta\".swap(i,i+1)","group":"AtCoder - freee Programming Contest 2022（AtCoder Beginner Contest 264）","url":"https://atcoder.jp/contests/abc264/tasks/abc264_d","interactive":false,"timeLimit":2000,"tests":[{"input":"catredo\n","output":"8\n"},{"input":"atcoder\n","output":"0\n"},{"input":"redocta\n","output":"21\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRedoctaswapii1"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let mut s = input.read_vec::<char>(7);

    let target = "atcoder";
    let mut ans = 0;
    for (i, c) in target.chars().enumerate() {
        for j in i..7 {
            if s[j] == c {
                ans += j - i;
                for k in (i..j).rev() {
                    s.swap(k, k + 1);
                }
                break;
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
