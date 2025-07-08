//{"name":"C. Smilo and Minecraft","group":"Codeforces - Codeforces Round 1031 (Div. 2)","url":"https://codeforces.com/contest/2113/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 3 1\n#.#\ng.g\n2 3 2\n#.#\ng.g\n3 4 2\n.gg.\ng..#\ng##.\n","output":"2\n0\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
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
    let k = input.read_size();
    let grid = input.read_char_table(n, m);

    let mut num_gold = Arr2d::new(n + 1, m + 1, 0);
    for i in 0..n {
        for j in 0..m {
            num_gold[i + 1][j + 1] = num_gold[i][j + 1] + num_gold[i + 1][j] - num_gold[i][j]
                + if grid[(i, j)] == b'g' { 1 } else { 0 };
        }
    }
    let mut ans = 0;
    let gold = grid.copy_count(b'g');
    for i in 0..n {
        for j in 0..m {
            if grid[(i, j)] != b'.' {
                continue;
            }
            let x0 = i.saturating_sub(k - 1);
            let x1 = (i + k).min(n);
            let y0 = j.saturating_sub(k - 1);
            let y1 = (j + k).min(m);
            let gold_in_square =
                num_gold[(x1, y1)] + num_gold[(x0, y0)] - num_gold[(x0, y1)] - num_gold[(x1, y0)];
            ans.maxim(gold - gold_in_square);
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
