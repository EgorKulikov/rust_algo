//{"name":"Air Conditioner Temperature","group":"CodeChef - START52A","url":"https://www.codechef.com/START52A/problems-old/ACTEMP","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n30 35 25\n30 35 40\n30 35 35\n30 25 35\n","output":"Yes\nNo\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AirConditionerTemperature"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{output, BoolOutput};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let a = input.read_usize();
    let b = input.read_usize();
    let c = input.read_usize();

    output().bool_output = BoolOutput::YesNo;
    out_line!(a.max(c) <= b);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
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
