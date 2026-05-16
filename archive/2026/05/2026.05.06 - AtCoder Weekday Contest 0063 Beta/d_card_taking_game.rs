//{"name":"D - Card Taking Game","group":"AtCoder - AtCoder Weekday Contest 0063 Beta","url":"https://atcoder.jp/contests/awc0063/tasks/awc0063_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 1 2 5\n","output":"3\n"},{"input":"4\n-1 5 -3 2\n","output":"11\n"},{"input":"10\n8 -3 5 12 -7 2 9 -1 6 4\n","output":"15\n"},{"input":"20\n15 -8 23 4 -12 7 19 -3 11 6 -5 14 2 -9 18 1 -6 10 8 -2\n","output":"53\n"},{"input":"1\n-1000000000\n","output":"-1000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let v = input.read_long_vec(n);

    let mut mem = Memoization2d::new(n + 1, n + 1, |mem, from, to| -> i64 {
        if from == to {
            0
        } else {
            (v[from] - mem.call(from + 1, to)).max(v[to - 1] - mem.call(from, to - 1))
        }
    });
    out.print_line(mem.call(0, n));
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
