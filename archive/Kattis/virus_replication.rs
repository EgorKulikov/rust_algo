//{"name":"Virus Replication","group":"Kattis","url":"https://open.kattis.com/problems/virus","interactive":false,"timeLimit":1000,"tests":[{"input":"AAAAA\nAGCGAA\n","output":"3\n"},{"input":"GTTTGACACACATT\nGTTTGACCACAT\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();
    let t = input.read_str();

    let mut commond_prefix = 0;
    for i in 0..s.len().min(t.len()) {
        if s[i] == t[i] {
            commond_prefix += 1;
        } else {
            break;
        }
    }
    let mut commond_suffix = 0;
    for i in 0..s.len().min(t.len()) {
        if s[Back(i)] == t[Back(i)] {
            commond_suffix += 1;
        } else {
            break;
        }
    }
    out.print_line(
        t.len()
            .saturating_sub((commond_prefix + commond_suffix).min(s.len())),
    );
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
