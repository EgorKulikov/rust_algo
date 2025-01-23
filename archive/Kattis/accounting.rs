//{"name":"Accounting","group":"Kattis","url":"https://open.kattis.com/problems/bokforing","interactive":false,"timeLimit":2000,"tests":[{"input":"3 5\nSET 1 7\nPRINT 1\nPRINT 2\nRESTART 33\nPRINT 1\n","output":"7\n0\n33\n"},{"input":"5 7\nRESTART 5\nSET 3 7\nPRINT 1\nPRINT 2\nPRINT 3\nPRINT 4\nPRINT 5\n","output":"5\n5\n7\n5\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let q = input.read_size();

    let mut map = DefaultHashMap::new(0);
    for _ in 0..q {
        let command = input.read_str();

        match command.as_slice() {
            b"SET" => {
                let i = input.read_size();
                let x = input.read_int();
                map[i] = x;
            }
            b"PRINT" => {
                let i = input.read_size();
                out.print_line(map[i]);
            }
            b"RESTART" => {
                let x = input.read_int();
                map = DefaultHashMap::new(x);
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
