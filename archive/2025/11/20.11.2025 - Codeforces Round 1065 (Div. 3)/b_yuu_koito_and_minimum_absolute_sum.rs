//{"name":"B. Yuu Koito and Minimum Absolute Sum","group":"Codeforces - Codeforces Round 1065 (Div. 3)","url":"https://codeforces.com/contest/2171/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n4\n2 -1 7 1\n4\n-1 2 4 -1\n8\n2 -1 1 5 11 12 1 -1\n3\n-1 -1 -1\n3\n2 5 4\n2\n-1 5\n","output":"1\n2 0 7 1\n0\n0 2 4 0\n0\n2 0 1 5 11 12 1 2\n0\n0 0 0\n2\n2 5 4\n0\n5 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_long_vec(n);

    if a[0] == -1 {
        if a[n - 1] == -1 {
            a[0] = 0;
            a[n - 1] = 0;
        } else {
            a[0] = a[n - 1];
        }
    } else if a[n - 1] == -1 {
        a[n - 1] = a[0];
    }
    for i in 0..n {
        if a[i] == -1 {
            a[i] = 0;
        }
    }
    out.print_line((a[n - 1] - a[0]).abs());
    out.print_line(a);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
    eprint!("\x1B[0m");
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive | TaskType::RunTwice => true,
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
