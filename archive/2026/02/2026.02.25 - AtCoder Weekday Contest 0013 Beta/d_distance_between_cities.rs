//{"name":"D - Distance Between Cities","group":"AtCoder - AtCoder Weekday Contest 0013 Beta","url":"https://atcoder.jp/contests/awc0013/tasks/awc0013_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n1 2\n4 6\n7 1\n","output":"22\n"},{"input":"2 1\n3\n10\n","output":"7\n"},{"input":"6 3\n1 0 3\n5 2 1\n-3 4 7\n2 -1 0\n6 3 5\n-1 2 4\n","output":"146\n"},{"input":"10 4\n100 -200 300 -400\n-150 250 -350 450\n200 -100 400 -300\n-250 150 -450 350\n300 -300 200 -200\n-100 400 -100 500\n50 -50 150 -150\n-350 350 -250 250\n400 -400 100 -100\n-200 200 -300 300\n","output":"62500\n"},{"input":"2 1\n-1000000\n1000000\n","output":"2000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_table(n, m);

    let mut ans = 0;
    for i in 0..m {
        let x = a.col(i).copied().collect::<Vec<_>>().sorted();
        for (j, (a, b)) in x.consecutive_iter_copy().enumerate() {
            ans += (b - a) * (j as i64 + 1) * (n as i64 - j as i64 - 1);
        }
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
