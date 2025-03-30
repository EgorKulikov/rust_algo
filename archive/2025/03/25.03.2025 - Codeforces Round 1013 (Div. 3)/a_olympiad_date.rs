//{"name":"A. Olympiad Date","group":"Codeforces - Codeforces Round 1013 (Div. 3)","url":"https://codeforces.com/contest/2091/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n10\n2 0 1 2 3 2 5 0 0 1\n8\n2 0 1 2 3 2 5 0\n8\n2 0 1 0 3 2 5 0\n16\n2 3 1 2 3 0 1 9 2 1 0 3 5 4 0 3\n","output":"9\n0\n8\n15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let req = vec![3, 1, 2, 1, 0, 1, 0, 0, 0, 0];
    let mut have = vec![0; 10];
    for i in 0..n {
        have[a[i]] += 1;
        if req.copy_zip(&have).all(|(r, h)| r <= h) {
            out.print_line(i + 1);
            return;
        }
    }
    out.print_line(0);
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
