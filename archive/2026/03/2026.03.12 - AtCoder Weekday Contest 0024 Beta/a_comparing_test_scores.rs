//{"name":"A - Comparing Test Scores","group":"AtCoder - AtCoder Weekday Contest 0024 Beta","url":"https://atcoder.jp/contests/awc0024/tasks/awc0024_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n80 65 80\n1 2\n2 1\n1 3\n3 1\n","output":"Yes\nNo\nNo\nNo\n"},{"input":"5 6\n72 85 60 85 90\n5 2\n2 4\n1 3\n4 2\n3 5\n2 1\n","output":"Yes\nNo\nYes\nNo\nNo\nYes\n"},{"input":"10 8\n45 78 92 65 100 38 72 0 55 100\n5 10\n10 5\n6 8\n8 6\n3 1\n1 2\n4 7\n9 4\n","output":"No\nNo\nYes\nNo\nYes\nNo\nNo\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_int_vec(n);

    for _ in 0..q {
        let a = input.read_size() - 1;
        let b = input.read_size() - 1;
        out.print_line(s[a] > s[b]);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
