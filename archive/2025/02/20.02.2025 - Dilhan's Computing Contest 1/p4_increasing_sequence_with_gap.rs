//{"name":"P4 - Increasing Sequence With Gap","group":"DMOJ - Dilhan's Computing Contest 1","url":"https://dmoj.ca/problem/dcc1p4","interactive":false,"timeLimit":5000,"tests":[{"input":"10 10\n4 -1 -1 23 -1 54 -1 -1 63 -1\n","output":"3\n"},{"input":"10 5\n4 -1 74 23 643 54 33 5 63 -1\n","output":"35\n"},{"input":"10 6\n-1 5 -1 4 -1 3 -1 2 -1 1\n","output":"Infinity\n"},{"input":"10 6\n1 5 2 4 3 3 4 2 5 1\n","output":"0\n"},{"input":"10 7\n1 5 2 4 3 3 4 2 5 1\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::bin_search::search_last_false;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::VecDeque;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_long_vec(n);

    if a.copy_count(-1) + 1 >= k {
        out.print_line("Infinity");
        return;
    }
    let ans = search_last_false(-1i64, 1_000_000_000, |g| {
        if g == -1 {
            return false;
        }
        let mut v = VecDeque::new();
        let mut ng = 0;
        for i in a.copy_iter() {
            if i == -1 {
                v.push_front(i64::MIN / 2);
                ng += 1;
            } else {
                let pos = search_last_false(0, v.len(), |j| j > 0 && v[j - 1] > i - (ng + 1) * g);
                if pos == v.len() {
                    v.push_back(i - ng * g);
                } else {
                    v[pos].minim(i - ng * g);
                }
            }
        }
        v.len() < k
    });
    out.print_line(ans);
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
