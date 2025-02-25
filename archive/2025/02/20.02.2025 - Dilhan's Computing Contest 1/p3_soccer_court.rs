//{"name":"P3 - Soccer Court","group":"DMOJ - Dilhan's Computing Contest 1","url":"https://dmoj.ca/problem/dcc1p3","interactive":false,"timeLimit":5000,"tests":[{"input":"4 6\n1 2 3 4 5 6\n2 3 4 5 6 7\n9 9 7 5 1 1\n8 8 7 7 6 6\n","output":"18\n"},{"input":"7 5\n10 10 10 10 10\n10 10 10 10 10\n10 10 10 10 10\n10 10 11 10 10\n10 10 10 10 10\n10 10 10 10 10\n10 10 10 10 10\n","output":"14\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let s = input.read_long_table(n, m);

    let mut p = Arr2d::new(n + 1, m + 1, 0);
    for i in 0..n {
        for j in 0..m {
            p[(i + 1, j + 1)] = p[(i + 1, j)] + p[(i, j + 1)] - p[(i, j)] + s[(i, j)];
        }
    }
    let mut ans = 0;
    for i in 1..m {
        for j in 1..=i.min(m - i) {
            let mut pos = FxHashMap::default();
            pos.insert(0, 0);
            for k in 1..=n {
                let left = p[(k, i)] - p[(0, i)] - p[(k, i - j)] + p[(0, i - j)];
                let right = p[(k, i + j)] - p[(0, i + j)] - p[(k, i)] + p[(0, i)];
                let key = right - left;
                if let Some(&v) = pos.get(&key) {
                    ans.maxim((k - v) * 2 * j);
                } else {
                    pos.insert(key, k);
                }
            }
        }
    }
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
