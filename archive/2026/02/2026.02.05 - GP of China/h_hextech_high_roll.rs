//{"name":"H. Hextech High-roll","group":"Universal Cup - GP of China","url":"https://contest.ucup.ac/contest/3295/problem/16335","interactive":false,"timeLimit":1000,"tests":[{"input":"1 2\n10\n1 100\n","output":"50.500000000\n"},{"input":"3 5\n3 4 7\n1 2 3 5 8\n","output":"7.400000000\n"},{"input":"2 4\n-1 -2\n-3 -4 -5 -6\n","output":"-1.000000000\n"},{"input":"5 14\n1 3 5 7 9\n-1 -1 0 0 2 2 4 4 6 6 8 8 10 10\n","output":"9.505494505\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{IntoReal, Real, RealReader};
use std::ops::Add;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let k = input.read_size();
    let n = input.read_size();
    let a = input.read_real_vec(k).sorted();
    let b = input.read_real_vec(n).sorted();

    if k == 1 {
        let alt = b.copy_fold(Real(0.), Real::add) / n;
        out.print_line(a[0].max(alt));
        return;
    }
    let mut prob = (k - 1).into_real() / n;
    let mut ans = Real(0.);
    let mut sum = Real(0.);
    for i in 0..=n + 1 - k {
        let val = (b[Back(i)] * (n - k + 1 - i) + sum) / (n - k + 1);
        ans += val.max(a[Back(0)]) * prob;
        prob *= n + 1 - k - i;
        prob /= n - i - 1;
        sum += b[Back(i)];
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
