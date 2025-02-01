//{"name":"Closest Pair (Uniform)","group":"Kattis","url":"https://open.kattis.com/problems/closestpair1","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n1.12 0\n0 0.51\n3\n158 12\n123 15\n1859 -1489\n3\n21.12 -884.2\n18.18 43.34\n21.12 -884.2\n0\n","output":"0.0 0.51 1.12 0.00\n123 15 158 12.00\n21.12 -884.20 21.12 -884.20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::geometry::point::Point;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::{RandomTrait, StaticRandom};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::Real;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let ps: Vec<Point<Real>> = input.read_vec(n);
    if n == 0 {
        return;
    }

    let mut ans = None;
    for _ in 0..10 {
        let alpha = StaticRandom.gen_real() * Real::PI;
        let order = (0..n)
            .collect::<Vec<_>>()
            .sorted_by_key(|&i| ps[i].x * alpha.cos() - ps[i].y * alpha.sin());
        for i in 0..n {
            for j in i.saturating_sub(10)..=(i + 10).min(n - 1) {
                if i == j {
                    continue;
                }
                let d = ps[order[i]].dist_point(ps[order[j]]);
                ans.minim((d, (order[i], order[j])));
            }
        }
    }

    let (i, j) = ans.unwrap().1;
    out.print_line((ps[i], ps[j]));
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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

//START MAIN
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
//END MAIN
