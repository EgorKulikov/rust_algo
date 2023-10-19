//{"name":"E1. Holidays #1","group":"CPython.uz - CPython Beginner Contest #37","url":"https://cpython.uz/competitions/contests/contest/312/problem/E1","interactive":false,"timeLimit":1000,"tests":[{"input":"07-07-2022\n","output":"Yes\n"},{"input":"07-07-2020\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1Holidays1"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::scan;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    scan!(input, "@-@-@", d: usize, m: usize, y: usize);

    out.set_bool_output(BoolOutput::YesNo);
    let ans = match (d, m) {
        (2, 2) | (7, 7) | (30, 12) => y >= 2021,
        (19, 4) => y >= 2023,
        _ => false,
    };
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
            for i in 0usize..t {
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
