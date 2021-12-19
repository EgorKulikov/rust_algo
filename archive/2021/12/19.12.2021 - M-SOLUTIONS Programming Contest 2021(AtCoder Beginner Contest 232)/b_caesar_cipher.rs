//{"name":"B - Caesar Cipher","group":"AtCoder - M-SOLUTIONS Programming Contest 2021(AtCoder Beginner Contest 232)","url":"https://atcoder.jp/contests/abc232/tasks/abc232_b","interactive":false,"timeLimit":2000,"tests":[{"input":"abc\nijk\n","output":"Yes\n"},{"input":"z\na\n","output":"Yes\n"},{"input":"ppq\nqqp\n","output":"No\n"},{"input":"atcoder\natcoder\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCaesarCipher"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::string::Str;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let s: Str = input.read();
    let t: Str = input.read();

    let delta = if s[0] <= t[0] {
        t[0] - s[0]
    } else {
        t[0] + 26 - s[0]
    };
    for (c, d) in s.into_iter().zip(t.into_iter()) {
        let c_delta = if c <= d { d - c } else { d + 26 - c };
        if delta != c_delta {
            out_line!("No");
            return;
        }
    }
    out_line!("Yes");
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
