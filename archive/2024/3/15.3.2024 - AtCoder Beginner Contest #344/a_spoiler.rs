//{"name":"A - Spoiler","group":"AtCoder - \tToyota Programming Contest 2024#3（AtCoder Beginner Contest 344）","url":"https://atcoder.jp/contests/abc344/tasks/abc344_a","interactive":false,"timeLimit":2000,"tests":[{"input":"atcoder|beginner|contest\n","output":"atcodercontest\n"},{"input":"|spoiler|\n","output":"\n"},{"input":"||xyz\n","output":"xyz\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASpoiler"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_str();

    let mut ans = Str::new();
    let mut spoiler = false;
    for c in s {
        if c == b'|' {
            spoiler = !spoiler;
        } else if !spoiler {
            ans.push(c);
        }
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
