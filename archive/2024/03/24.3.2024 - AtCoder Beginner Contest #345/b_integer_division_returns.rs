//{"name":"B - Integer Division Returns","group":"AtCoder - Monoxer Programming Contest 2024（AtCoder Beginner Contest 345）","url":"https://atcoder.jp/contests/abc345/tasks/abc345_b","interactive":false,"timeLimit":2000,"tests":[{"input":"27\n","output":"3\n"},{"input":"-13\n","output":"-1\n"},{"input":"40\n","output":"4\n"},{"input":"-20\n","output":"-2\n"},{"input":"123456789123456789\n","output":"12345678912345679\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BIntegerDivisionReturns"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut x = input.read_long();

    while x % 10 != 0 {
        x += 1;
    }
    out.print_line(x / 10);
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
