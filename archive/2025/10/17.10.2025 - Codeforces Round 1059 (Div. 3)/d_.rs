//{"name":"D. Красивая перестановка","group":"Codeforces - Codeforces Round 1059 (Div. 3)","url":"https://codeforces.com/contest/2162/problem/D","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n\n\n3\n\n4\n\n5\n\n\n4\n\n8\n\n8\n\n7\n","output":"\n1 1 2\n\n2 1 2\n\n! 2 2\n\n1 2 4\n\n2 1 3\n\n2 3 4\n\n! 2 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut request = |t: usize, l: usize, r: usize| -> usize {
        out.print_line((t, l, r));
        out.flush();
        input.read_size()
    };
    let len = request(2, 1, n) - n * (n + 1) / 2;
    let mut left = 1;
    let mut right = n;
    while left < right {
        let mid = (left + right) / 2;
        let delta = request(2, 1, mid) - request(1, 1, mid);
        if delta > 0 && delta < len {
            output!(out, "! {} {}", mid - delta + 1, mid + len - delta);
            out.flush();
            return;
        } else if delta == 0 {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    output!(out, "! {} {}", left, left + len - 1);
    out.flush();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
