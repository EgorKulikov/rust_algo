//{"name":"A. Mix Mex Max","group":"Codeforces - Atto Round 1 (Codeforces Round 1041, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2127/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n3\n-1 -1 -1\n5\n1 1 1 1 0\n6\n5 5 1 -1 -1 1\n4\n-1 -1 0 -1\n4\n-1 1 1 -1\n3\n3 3 -1\n5\n0 0 0 0 0\n7\n3 0 1 4 -1 2 3\n","output":"YES\nNO\nNO\nNO\nYES\nYES\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut x = -1;
    for i in 0..n {
        if a[i] != -1 {
            if x == -1 {
                x = a[i];
            } else if x != a[i] {
                out.print_line(false);
                return;
            }
        }
    }
    out.print_line(x != 0);
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
