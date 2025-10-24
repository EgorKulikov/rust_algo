//{"name":"Racing Around the Alphabet","group":"Kattis","url":"https://open.kattis.com/problems/racingalphabet","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nWINNING ISN'T EVERYTHING IT'S THE ONLY THING\nWINNERS DON'T WAIT FOR CHANCES THEY TAKE THEM\nPAIN IS ONLY TEMPORARY BUT VICTORY IS FOREVER\n","output":"187.6156641641\n190.4108599662\n193.1036536692\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::Real;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_line();

    let order = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ '";
    let mut ans = 0;
    for (a, b) in s.consecutive_iter_copy() {
        let p1 = order.iter().position(|&c| c == a).unwrap();
        let p2 = order.iter().position(|&c| c == b).unwrap();
        let d = p1.abs_diff(p2);
        ans += d.min(28 - d);
    }
    out.print_line(Real::PI * 60 / 15 / 28 * ans + s.len());
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
