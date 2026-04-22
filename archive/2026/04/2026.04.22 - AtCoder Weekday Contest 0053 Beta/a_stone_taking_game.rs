//{"name":"A - Stone Taking Game","group":"AtCoder - AtCoder Weekday Contest 0053 Beta","url":"https://atcoder.jp/contests/awc0053/tasks/awc0053_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 2 1\n","output":"Aoki\n"},{"input":"2\n3 4\n","output":"Takahashi\n"},{"input":"8\n7 1 4 2 6 3 5 8\n","output":"Aoki\n"},{"input":"20\n1000000000 999999999 123456789 987654321 500000000 400000000 300000000 200000000 100000000 999999998 7654321 8765432 135791357 246802468 111111111 222222222 333333333 444444444 555555555 666666667\n","output":"Takahashi\n"},{"input":"1\n1000000000\n","output":"Aoki\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    if a.copy_sum() % 2 == 0 {
        out.print_line("Aoki");
    } else {
        out.print_line("Takahashi");
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
