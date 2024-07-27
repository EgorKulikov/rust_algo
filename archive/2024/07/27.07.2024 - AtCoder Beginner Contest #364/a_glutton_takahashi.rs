//{"name":"A - Glutton Takahashi","group":"AtCoder - Japan Registry Services (JPRS) Programming Contest 2024#2 (AtCoder Beginner Contest 364)","url":"https://atcoder.jp/contests/abc364/tasks/abc364_a","interactive":false,"timeLimit":2000,"tests":[{"input":"5\nsalty\nsweet\nsalty\nsalty\nsweet\n","output":"Yes\n"},{"input":"4\nsweet\nsalty\nsweet\nsweet\n","output":"Yes\n"},{"input":"6\nsalty\nsweet\nsweet\nsalty\nsweet\nsweet\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AGluttonTakahashi"}}}

use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str_vec(n);

    out.set_bool_output(BoolOutput::YesNo);
    for (s, t) in s.consecutive_iter().rev().skip(1).rev() {
        if s == &Str::from("sweet") && t == &Str::from("sweet") {
            out.print_line(false);
            return;
        }
    }
    out.print_line(true);
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
}
//END MAIN
