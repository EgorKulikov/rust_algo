//{"name":"A - Progressive Taxation Simulation","group":"AtCoder - AtCoder Weekday Contest 0049 Beta","url":"https://atcoder.jp/contests/awc0049/tasks/awc0049_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 100 10 23\n50\n100\n150\n","output":"5\n10\n21\n"},{"input":"4 200 30 10\n100\n200\n300\n1\n","output":"30\n60\n70\n0\n"},{"input":"5 5000 15 40\n0\n3000\n5000\n8000\n12345\n","output":"0\n450\n750\n1950\n3688\n"},{"input":"8 1000000000 5 99\n0\n1\n999999999\n1000000000\n1000000000\n999999997\n500000000\n123456789\n","output":"0\n0\n49999999\n50000000\n50000000\n49999999\n25000000\n6172839\n"},{"input":"6 1 0 0\n0\n1\n1000000000\n0\n0\n1\n","output":"0\n0\n0\n0\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let l = input.read_long();
    let p = input.read_long();
    let q = input.read_long();

    for _ in 0..n {
        let s = input.read_long();
        if s <= l {
            out.print_line(s * p / 100);
        } else {
            out.print_line((l * p + (s - l) * q) / 100);
        }
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
