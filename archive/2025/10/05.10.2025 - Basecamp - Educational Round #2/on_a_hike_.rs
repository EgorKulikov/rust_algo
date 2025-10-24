//{"name":"On a hike!","group":"Eolymp - Basecamp - Educational Round #2","url":"https://basecamp.eolymp.com/en/compete/b0f0h1n10h679euo6gn2c7821o/problem/7","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n1 1\n","output":"0\n"},{"input":"2\n1 3\n4 4\n","output":"2\n"},{"input":"3\n0 0\n3 3\n0 3\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let xy = input.read_long_pair_vec(n);

    let mut sum_max = i64::MIN;
    let mut sum_min = i64::MAX;
    let mut diff_max = i64::MIN;
    let mut diff_min = i64::MAX;
    for (x, y) in xy {
        let sum = x + y;
        let diff = x - y;
        sum_max.maxim(sum);
        sum_min.minim(sum);
        diff_max.maxim(diff);
        diff_min.minim(diff);
    }

    let sum_delta = sum_max - sum_min;
    let diff_delta = diff_max - diff_min;
    let delta = sum_delta.max(diff_delta);
    out.print_line(delta.upper_div(2));
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
