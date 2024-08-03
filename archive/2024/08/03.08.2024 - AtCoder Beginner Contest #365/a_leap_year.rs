//{"name":"A - Leap Year","group":"AtCoder - Toyota Programming Contest 2024#8（AtCoder Beginner Contest 365）","url":"https://atcoder.jp/contests/abc365/tasks/abc365_a","interactive":false,"timeLimit":2000,"tests":[{"input":"2023\n","output":"365\n"},{"input":"1992\n","output":"366\n"},{"input":"1800\n","output":"365\n"},{"input":"1600\n","output":"366\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ALeapYear"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let y = input.read_size();

    out.print_line(if y % 400 == 0 || (y % 4 == 0 && y % 100 != 0) {
        "366"
    } else {
        "365"
    });
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
