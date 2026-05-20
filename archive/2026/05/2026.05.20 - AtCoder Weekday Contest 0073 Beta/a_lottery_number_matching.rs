//{"name":"A - Lottery Number Matching","group":"AtCoder - AtCoder Weekday Contest 0073 Beta","url":"https://atcoder.jp/contests/awc0073/tasks/awc0073_a","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n1 2 3 4 5\n2 7 4\n","output":"2\n"},{"input":"4 6\n100 200 300 400\n150 200 250 300 350 400\n","output":"3\n"},{"input":"10 8\n1000000000 999999999 123456789 987654321 555555555 111111111 222222222 333333333 444444444 666666666\n123456789 100000000 555555555 777777777 888888888 999999999 12345 67890\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let k = input.read_size();
    let m = input.read_size();
    let a = input.iter_size().take(k).collect::<FxHashSet<_>>();
    let b = input.read_size_vec(m);

    out.print_line(b.copy_filter(|i| a.contains(i)).count());
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
