//{"name":"problem_j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::two_sat::TwoSat;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let good = input.read_size_pair_vec(k);
    let bad = input.read_vec::<Vec<(usize, usize)>>(k);

    let mut grid = Arr2d::new(n, m, None);
    for i in 0..k {
        grid[good[i].dec()] = Some((i, true));
        for b in bad[i].copy_iter() {
            grid[b.dec()] = Some((i, false));
        }
    }
    let mut left = 1;
    let mut right = m;
    let mut solution = vec![];
    let mut two_sat = TwoSat::new(k + 2 * n * m);
    while left < right {
        two_sat.clear();
        let mid = (left + right) / 2;
        for i in 0..k {
            if bad[i].is_empty() {
                two_sat.set_true(2 * n * m + i);
            }
        }
        for i in 0..n {
            for j in 0..m - 1 {
                two_sat.add_or(i * m + j + 1, false, i * m + j, true);
                two_sat.add_or(n * m + i * m + j, false, n * m + i * m + j + 1, true);
            }
            for j in 0..m {
                if let Some((id, is_true)) = grid[(i, j)] {
                    two_sat.add_or(2 * m * n + id, !is_true, i * m + j, true);
                    two_sat.add_or(2 * m * n + id, !is_true, n * m + i * m + j, true);
                    if j + mid < m {
                        two_sat.add_or(2 * m * n + id, !is_true, i * m + j + mid, false);
                    }
                    if j >= mid {
                        two_sat.add_or(2 * m * n + id, !is_true, n * m + i * m + j - mid, false);
                    }
                }
            }
        }
        if let Some(s) = two_sat.solve() {
            right = mid;
            solution.clear();
            for i in 0..k {
                if s[i + 2 * m * n] {
                    solution.push(i + 1);
                }
            }
        } else {
            left = mid + 1;
        }
    }
    out.print_line(left);
    out.print_line((solution.len(), solution));
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
