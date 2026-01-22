//{"name":"Texture Analysis","group":"Kattis","url":"https://open.kattis.com/problems/textureanalysis","interactive":false,"timeLimit":1000,"tests":[{"input":"*.*.*.*.*.*.*.*.*\n*..*.*.*.*.*.*.*.*\n*..*..*\n*\n***\n*.**\nEND\n","output":"1 EVEN\n2 NOT EVEN\n3 EVEN\n4 EVEN\n5 EVEN\n6 NOT EVEN\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    if s.as_slice() == b"END" {
        return;
    }
    if s.len() == 1 {
        out.print_line((test_case, "EVEN"));
        return;
    }
    let pos = s
        .iter_enumerate()
        .filter(|&(_, c)| c == b'*')
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let diff = pos
        .consecutive_iter_copy()
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();
    out.print_line((
        test_case,
        if diff.copy_count(diff[0]) == diff.len() {
            "EVEN"
        } else {
            "NOT EVEN"
        },
    ));
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
