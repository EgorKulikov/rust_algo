//{"name":"Need for Speed","group":"Kattis","url":"https://open.kattis.com/problems/speed","interactive":false,"timeLimit":1000,"tests":[{"input":"3 5\n4 -1\n4 0\n10 3\n","output":"3.000000000\n"},{"input":"4 10\n5 3\n2 2\n3 6\n3 1\n","output":"-0.508653377\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::ops::Add;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let t = input.read_real();
    let segs: Vec<(Real, Real)> = input.read_vec(n);

    let mut left = -segs.copy_map(|(_, s)| s).min().unwrap();
    let mut right = left + segs.copy_map(|(d, _)| d).reduce(Real::add).unwrap();

    for _ in 0..100 {
        let mid = (left + right) / 2;
        let mut ct = Real(0.);
        for (d, s) in segs.copy_iter() {
            ct += d / (s + mid);
        }
        if ct < t {
            right = mid;
        } else {
            left = mid;
        }
    }
    out.print_line(left);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
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
//END MAIN
