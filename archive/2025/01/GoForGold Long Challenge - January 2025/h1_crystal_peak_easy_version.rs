//{"name":"H1. Crystal Peak (Easy Version)","group":"Codeforces - GoForGold Long Challenge - January 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/579716/problem/H1","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n0 0 1 0 0\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(2 * n - 1);

    let mut left = a.copy_min();
    let mut right = a.copy_max();
    while left < right {
        let mid = left + (right - left + 1) / 2;

        let mut res = 2;
        let val = |i: usize| -> i32 { (a[i] >= mid) as i32 };
        for i in 1..n {
            if val(n - 1 - i) == val(n - i) {
                res = val(n - i);
                break;
            }
            if val(n - 1 + i) == val(n + i - 2) {
                res = val(n + i - 2);
                break;
            }
        }
        if res == 2 {
            res = val(0);
        }
        if res == 0 {
            right = mid - 1;
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
