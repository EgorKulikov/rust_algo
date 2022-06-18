//{"name":"A. Ориентированное увеличение","group":"Codeforces - Codeforces Round #800 (Div. 1)","url":"https://codeforces.com/contest/1693/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n2\n1 0\n4\n2 -1 -1 0\n4\n1 -4 3 0\n4\n1 -1 1 -1\n5\n1 2 3 4 -10\n7\n2 -1 1 -2 0 0 0\n1\n0\n","output":"No\nYes\nNo\nNo\nYes\nYes\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AOrientirovannoeUvelichenie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{output, BoolOutput};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_long_vec(n);

    output().bool_output = BoolOutput::YesNo;
    let mut sum = 0;
    let mut shoud_be_zero = false;
    for i in a {
        if shoud_be_zero && i != 0 {
            out_line!(false);
            return;
        }
        sum += i;
        if sum < 0 {
            out_line!(false);
            return;
        }
        if sum == 0 {
            shoud_be_zero = true;
        }
    }
    out_line!(sum == 0);
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
