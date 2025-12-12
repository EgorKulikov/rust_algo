//{"name":"day12","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::eol::EolVec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::scan;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut shapes = Vec::new();
    let mut sizes = Vec::new();
    for _ in 0..6 {
        input.skip_whitespace();
        input.read_line();
        let cur = input.read_char_table(3, 3);
        sizes.push(cur.copy_count(b'#'));
        shapes.push(cur);
    }
    input.skip_whitespace();

    let mut fit = 0;
    let mut no_fit = 0;
    let mut unknown = 0;
    while !input.is_empty() {
        scan!(input, "@x@: @", w: usize, h: usize, qty: EolVec<usize>);

        if qty.copy_sum() <= (w / 3) * (h / 3) {
            fit += 1;
        } else if qty
            .copy_enumerate()
            .map(|(i, q)| sizes[i] * q)
            .sum::<usize>()
            > w * h
        {
            no_fit += 1;
        } else {
            unknown += 1;
        }
    }
    out.print_line((fit, no_fit, unknown));
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
