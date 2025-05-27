//{"name":"I. Bingo 3","group":"Universal Cup - The 3rd Universal Cup. Stage 37: Wuhan","url":"https://contest.ucup.ac/contest/2025/problem/10744","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3 5\n4 10\n5 2\n1 1\n","output":"Yes\n5 1 2\n3 6 4\n7 8 9\nYes\n10 1 2 3\n4 11 5 6\n7 8 12 9\n13 14 15 16\nNo\nYes\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    if k < n || k > n * (n - 1) + 1 {
        out.print_line(false);
        return;
    }
    let mut ans = Arr2d::new(n, n, 0);
    let mut cur = 1;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if cur == k {
                ans[(0, 0)] = cur;
                cur += 1;
            }
            ans[(i, j)] = cur;
            cur += 1;
        }
    }
    if cur == k {
        ans[(0, 0)] = cur;
        cur += 1;
    }
    for i in 1..n {
        ans[(i, i)] = cur;
        cur += 1;
    }
    out.print_line(true);
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
