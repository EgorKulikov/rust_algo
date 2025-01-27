//{"name":"K. Kingdom's Edge","group":"Codeforces - GoForGold Long Challenge - January 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/579716/problem/K","interactive":false,"timeLimit":1000,"tests":[{"input":"Judge     User\n\n1 5\n\n          1 1\n5\n\n          2 1\n","output":"5 is correctly present at the reported index = 1\n"},{"input":"Judge     User\n\n3 15\n\n          1 1\n17\n\n          1 2\n16\n\n          1 3\n\n15\n\n          2 3\n","output":"15 is correctly present at the reported index = 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_int();

    let mut query = |i: usize| -> i32 {
        out.print_line((1, i));
        out.flush();
        input.read_int()
    };
    let mut left = 0;
    let mut right = (if n % 2 == 0 { n + 2 } else { n + 1 }) / 2;
    while right - left > 1 {
        let mid = (left + right) / 2;
        let y = query(mid * 2);
        if y == x {
            out.print_line((2, mid * 2));
            return;
        }
        if y > x {
            right = mid;
        } else {
            left = mid;
        }
    }
    let z = query(left * 2 + 1);
    if z == x {
        out.print_line((2, left * 2 + 1));
        return;
    }
    let mut left = 0;
    let mut right = (if n % 2 == 0 { n + 2 } else { n + 1 }) / 2;
    while right - left > 1 {
        let mid = (left + right) / 2;
        let y = query(mid * 2);
        if y > z {
            right = mid;
        } else {
            left = mid;
        }
    }
    let z = query(left * 2 + 1);
    if z == x {
        out.print_line((2, left * 2 + 1));
    } else {
        out.print_line((2, 0));
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
