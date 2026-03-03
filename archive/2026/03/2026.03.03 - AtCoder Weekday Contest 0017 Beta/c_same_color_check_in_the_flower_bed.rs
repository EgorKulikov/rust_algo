//{"name":"C - Same Color Check in the Flower Bed","group":"AtCoder - AtCoder Weekday Contest 0017 Beta","url":"https://atcoder.jp/contests/awc0017/tasks/awc0017_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n1 1 2 2 2\n1 5\n1 3\n3 5\n","output":"3\n1\n2\n"},{"input":"8 5\n3 3 3 1 2 2 1 1\n1 8\n2 5\n1 4\n5 8\n3 6\n","output":"4\n1\n2\n2\n1\n"},{"input":"15 8\n10 10 5 5 5 3 3 7 7 7 7 1 1 2 2\n1 15\n1 5\n5 10\n10 15\n3 7\n2 14\n6 8\n1 2\n","output":"9\n3\n3\n3\n3\n7\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let c = input.read_int_vec(n);

    let s = c
        .consecutive_iter_copy()
        .map(|(a, b)| (a == b) as i32)
        .collect::<Vec<_>>();
    let p = s.partial_sums();
    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size() - 1;
        out.print_line(p[r] - p[l]);
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
