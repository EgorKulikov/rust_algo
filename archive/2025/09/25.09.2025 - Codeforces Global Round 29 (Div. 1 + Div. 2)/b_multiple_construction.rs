//{"name":"B. Multiple Construction","group":"Codeforces - Codeforces Global Round 29 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2147/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\n3\n1\n","output":"1 2 1 2\n1 3 1 2 3 2\n1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut ans = vec![0; 2 * n];
    let mut at = 0;
    for i in (2..=n - 1).rev().step_by(2) {
        ans[at] = i;
        ans[at + i] = i;
        at += 1;
    }
    at = n;
    if n >= 2 {
        for i in (2..=n - 2).rev().step_by(2) {
            ans[at] = i;
            ans[at + i] = i;
            at += 1;
        }
    }
    let mut found = false;
    for i in 0..n {
        if ans[i] == 0 && ans[i + n] == 0 {
            ans[i] = n;
            ans[i + n] = n;
            found = true;
            break;
        }
    }
    assert!(found);
    for i in 0..2 * n {
        if ans[i] == 0 {
            ans[i] = 1;
        }
    }
    out.print_line(ans);
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
