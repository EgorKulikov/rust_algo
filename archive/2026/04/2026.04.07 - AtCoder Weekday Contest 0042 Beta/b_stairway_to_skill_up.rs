//{"name":"B - Stairway to Skill Up","group":"AtCoder - AtCoder Weekday Contest 0042 Beta","url":"https://atcoder.jp/contests/awc0042/tasks/awc0042_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3 20\n2 3 5 8 1\n","output":"Yes\n"},{"input":"3 1 100\n5 10 20\n","output":"No\n"},{"input":"10 5 50\n3 7 2 8 4 15 6 1 12 9\n","output":"Yes\n"},{"input":"15 10 1000000000000000000\n999999999 1000000000 500000000 800000000 200000000 100000000 50000000 10 5 3 2 1 7 8 4\n","output":"No\n"},{"input":"1 1 1\n1\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_long();
    let t = input.read_long();
    let d = input.read_long_vec(n).sorted();

    let mut cur = s;
    for i in d {
        if cur >= i {
            cur += i;
        }
    }
    out.print_line(cur >= t);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
