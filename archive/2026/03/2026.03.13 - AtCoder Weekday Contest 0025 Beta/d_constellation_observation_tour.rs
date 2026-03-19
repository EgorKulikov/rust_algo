//{"name":"D - Constellation Observation Tour","group":"AtCoder - AtCoder Weekday Contest 0025 Beta","url":"https://atcoder.jp/contests/awc0025/tasks/awc0025_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 1 3\n1 3 6\n","output":"2\n"},{"input":"4 2 4\n0 10 5 20\n","output":"1\n"},{"input":"5 3 1000000000\n1 4 6 9 15\n","output":"3\n"},{"input":"10 5 1000000000000000000\n-1000000000 -500000000 -100 0 50 100 200 500000000 999999999 1000000000\n","output":"5\n"},{"input":"2 1 1000000000000000000\n0 1\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_size() - 1;
    let q = input.read_size();
    let x = input.read_int_vec(n);

    let order = (0..n).collect::<Vec<_>>().sorted_by_key(|&i| x[i]);
    let mut pos = order.copy_find(s).unwrap();
    for i in 0..q {
        if i >= n && i % 2 == q % 2 {
            break;
        }
        if pos == 0 {
            pos = 1;
        } else if pos == n - 1 {
            pos = n - 2;
        } else {
            let left = x[order[pos]] - x[order[pos - 1]];
            let right = x[order[pos + 1]] - x[order[pos]];
            if left < right || left == right && order[pos - 1] < order[pos + 1] {
                pos -= 1;
            } else {
                pos += 1;
            }
        }
    }
    out.print_line(order[pos] + 1);
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
