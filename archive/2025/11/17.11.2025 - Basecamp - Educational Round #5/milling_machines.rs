//{"name":"Milling machines","group":"Eolymp - Basecamp - Educational Round #5","url":"https://eolymp.com/en/compete/saaq21a9v514tbj0spj7bmgjpk/problem/3","interactive":false,"timeLimit":1000,"tests":[{"input":"2 1\n3 4\n4 4 4\n4 2 3\n2 3 0\n","output":"2 1 4\n2 1 3\n"},{"input":"1 3\n10 100\n11 22 33 44 55 66 77 88 99 100\n1 100 1 100 1 100 1 100 1 100\n58 58 58 58 58 58 58 58 58 58\n42 42 42 42 42 42 42 42 66 42\n","output":"11 0 33 0 42 0 42 0 34 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let w = input.read_size();
    let s = input.read_size();
    let x = input.read_size();
    let y = input.read_size();
    let p = input.read_size_table(w, x);
    let m = input.read_size_table(s, x);

    let fm = Vec::with_gen(x, |i| *m.col(i).max().unwrap());
    for i in 0..w {
        out.print_line_iter(p.row(i).enumerate().map(|(j, &v)| v.min(y - fm[j])));
    }
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
