//{"name":"G - Prefix Concatenation","group":"AtCoder - NS Solutions Corporation Programming Contest 2022（AtCoder Beginner Contest 257）","url":"https://atcoder.jp/contests/abc257/tasks/abc257_g","interactive":false,"timeLimit":2000,"tests":[{"input":"aba\nababaab\n","output":"3\n"},{"input":"atcoder\nac\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GPrefixConcatenation"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use algo_lib::string::string_algorithms::StringAlgorithms;

fn solve(input: &mut Input) {
    let s: Str = input.read();
    let t: Str = input.read();

    let s_len = s.len();
    let z = (s + b'#' + t).z_algorithm();
    let mut ans = 0;
    let mut up_to = 0;
    let mut next_up_to = 0;
    for (i, z) in z.into_iter().skip(s_len + 1).enumerate() {
        next_up_to.maxim(i + z);
        if i == up_to {
            ans += 1;
            if next_up_to == up_to {
                out_line!(-1);
                return;
            }
            up_to = next_up_to;
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
