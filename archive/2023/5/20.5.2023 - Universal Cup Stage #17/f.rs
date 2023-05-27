//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_bool_output, BoolOutput};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let p = input.read_size_vec(n).dec_by_one();

    let mut count = 0;
    for i in 0..n {
        if i + 1 < n {
            if p[i + 1] == p[i] + 1 {
                count += 1;
            }
            if p[i] == p[i + 1] + 1 {
                count += 1;
            }
        }
        if i + 2 < n {
            if p[i] == p[i + 2] + 1 {
                count += 1;
            }
            if p[i + 2] == p[i] + 1 {
                count += 1;
            }
        }
    }
    out_line!(count == n - 1);
}

pub(crate) fn run(mut input: Input) -> bool {
    set_bool_output(BoolOutput::YesNo);

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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
