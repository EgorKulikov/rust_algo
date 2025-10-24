//{"name":"F. Nezuko in the Clearing","group":"Codeforces - Codeforces Round 1054 (Div. 3)","url":"https://codeforces.com/contest/2149/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3 2\n1 1\n5 3\n2 4\n10 7\n","output":"3\n2\n4\n7\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_long();
    let d = input.read_long();

    let mut left = 1;
    let mut right = d + 1;

    while left < right {
        let mid = (left + right) / 2;
        let total_hp = h + mid - 1;
        let short = d / mid;
        let long = short + 1;
        let long_count = d % mid;
        let short_count = mid - long_count;
        let damage = short_count * short * (short + 1) / 2 + long_count * long * (long + 1) / 2;
        if damage >= total_hp {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    out.print_line(d + left - 1);
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
