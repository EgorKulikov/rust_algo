//{"name":"E - Optimization of Delivery Routes","group":"AtCoder - AtCoder Weekday Contest 0017 Beta","url":"https://atcoder.jp/contests/awc0017/tasks/awc0017_e","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n0 0\n1 1\n","output":"4\n"},{"input":"4\n0 0\n1 0\n1 1\n0 1\n","output":"4\n"},{"input":"5\n0 0\n3 0\n3 4\n-1 3\n0 3\n","output":"46\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::geometry::point::Point;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let pts = input.read_vec::<Point<i64>>(n);

    let mut res = Arr2d::new(1 << n, n, i64::MAX);
    res[(1, 0)] = 0;
    for mask in usize::iter_all(n) {
        if !mask.is_set(0) {
            continue;
        }
        for j in 0..n {
            if !mask.is_set(j) || res[(mask, j)] == i64::MAX {
                continue;
            }
            for k in 1..n {
                if mask.is_set(k) {
                    continue;
                }
                let cand = res[(mask, j)] + pts[j].square_dist_point(pts[k]);
                res[(mask.with_bit(k), k)].minim(cand);
            }
        }
    }
    let mut ans = i64::MAX;
    for i in 1..n {
        ans.minim(res[(usize::all_bits(n), i)] + pts[i].square_dist_point(pts[0]));
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
