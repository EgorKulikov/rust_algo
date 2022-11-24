//{"name":"A. Две перестановки","group":"Codeforces - Pinely Round 1 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1761/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 1 1\n2 1 2\n3 1 1\n4 1 1\n","output":"Yes\nNo\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADvePerestanovki"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_bool_output, BoolOutput};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize();
    let b = input.read_usize();

    set_bool_output(BoolOutput::YesNo);
    out_line!(n == a && n == b || n >= a + b + 2);
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
