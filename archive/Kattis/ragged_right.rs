//{"name":"Ragged Right","group":"Kattis","url":"https://open.kattis.com/problems/raggedright","interactive":false,"timeLimit":1000,"tests":[{"input":"some blocks\nof text line up\nwell on the right,\nbut\nsome don't.\n","output":"283\n"},{"input":"this line is short\nthis one is a bit longer\nand this is the longest of all.\n","output":"218\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::number_ext::Square;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut lines = input.read_lines();

    let max_len = lines.iter().map(|line| line.len()).max().unwrap();
    lines.pop();
    out.print_line(
        lines
            .iter_map(|x| (max_len - x.len()).square())
            .sum::<usize>(),
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

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::buf(&mut stdout);

    run(input, output);
}
//END MAIN
