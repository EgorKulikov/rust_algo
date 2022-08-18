//{"name":"Gritty Grid","group":"CodeChef - START52A","url":"https://www.codechef.com/START52A/problems-old/GRITGRID","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2 2 2 1\n2 2 1 1\n2 2 1 2\n2 3 3 3\n","output":"Yes\nNo\nYes\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GrittyGrid"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{output, BoolOutput};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let x = input.read_usize();
    let y = input.read_usize();

    let dist = n + m - 2;
    output().bool_output = BoolOutput::YesNo;
    out_line!((x + y) % 2 == 1 || x % 2 == dist % 2);
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
