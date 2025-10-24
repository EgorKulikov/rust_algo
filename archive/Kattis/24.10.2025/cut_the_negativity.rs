//{"name":"Cut the Negativity","group":"Kattis","url":"https://open.kattis.com/problems/cutthenegativity","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n-1 1 -1 2\n9 -1 -1 -1\n-1 3 -1 4\n7 1 2 -1\n","output":"8\n1 2 1\n1 4 2\n2 1 9\n3 2 3\n3 4 4\n4 1 7\n4 2 1\n4 3 2\n\n"},{"input":"3\n-1 -1 -1\n15 -1 -1\n2 2 -1\n","output":"3\n2 1 15\n3 1 2\n3 2 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_table(n, n);

    let mut ans = Vec::new();
    for (i, j) in a.indices() {
        if a[(i, j)] != -1 {
            ans.push((i + 1, j + 1, a[(i, j)]));
        }
    }
    out.print_line(ans.len());
    out.print_per_line(&ans);
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
