//{"name":"A - Packing Sweets into Boxes","group":"AtCoder - AtCoder Weekday Contest 0042 Beta","url":"https://atcoder.jp/contests/awc0042/tasks/awc0042_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 5\n1 2\n4 0\n3 5\n","output":"4\n"},{"input":"4 3\n0 0\n1 2\n3 0\n2 5\n","output":"5\n"},{"input":"8 7\n2 3\n7 0\n6 6\n0 5\n14 1\n8 13\n0 0\n20 4\n","output":"15\n"},{"input":"15 10\n100 0\n0 1\n9 9\n10 10\n123 456\n999999999 1\n500000000 500000000\n17 23\n0 0\n42 58\n1000000000 1000000000\n314159265 271828182\n5 4\n19 0\n0 999999999\n","output":"558598835\n"},{"input":"1 1000000000\n0 0\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_long();
    let ab = input.read_long_pair_vec(n);

    let mut ans = 0;
    for (a, b) in ab {
        ans += (a + b).upper_div(k);
    }
    out.print_line(ans);
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
