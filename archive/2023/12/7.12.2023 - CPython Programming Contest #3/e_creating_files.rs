//{"name":"E. Creating files","group":"CPython.uz - CPython Programming Contest #3","url":"https://cpython.uz/competitions/contests/contest/326/problem/E","interactive":false,"timeLimit":1000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ECreatingFiles"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::random;
use algo_lib::string::str::StrReader;
use std::fs;
use std::path::PathBuf;

type PreCalc = ();

fn solve(input: &mut Input, _out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_str();

    fs::create_dir_all(s.to_string()).unwrap();
    for _ in 1..=100 {
        let name = random().gen().to_string();
        let file_path = PathBuf::from(s.to_string()).join(name);
        std::fs::write(file_path, "").unwrap();
    }
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
                solve(&mut input, &mut output, i, &pre_calc);
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
