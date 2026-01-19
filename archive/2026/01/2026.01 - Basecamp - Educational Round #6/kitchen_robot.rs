//{"name":"Kitchen Robot","group":"Eolymp - Basecamp - Educational Round #6","url":"https://eolymp.com/en/compete/ikoi44ho994uj2rcj49h0l45v4/problem/10","interactive":false,"timeLimit":1000,"tests":[{"input":"3 4\n2\n1 1\n2 3\n2 1\n","output":"5.60555127546399\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::geometry::point::Point;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let w = input.read_real();
    let l = input.read_real();
    let n = input.read_size();
    let pts = input.read_vec::<Point<Real>>(n + 1);

    let dist = Arr2d::with_gen(n + 1, n + 1, |i, j| {
        if i == n {
            return pts[i].dist_point(pts[j]);
        }
        let mut res = Real(f64::INFINITY);
        res.minim(pts[i].dist_point(Point::new(-pts[j].x, pts[j].y)));
        res.minim(pts[i].dist_point(Point::new(pts[j].x, -pts[j].y)));
        res.minim(pts[i].dist_point(Point::new(w * 2 - pts[j].x, pts[j].y)));
        res.minim(pts[i].dist_point(Point::new(pts[j].x, l * 2 - pts[j].y)));
        res
    });

    let mut mem = Memoization2d::new(n + 1, 1 << n, |mem, pos, mask| {
        let mut res = Real(f64::INFINITY);
        if mask == 0 {
            res.minim(pts[pos].x);
            res.minim(w - pts[pos].x);
            res.minim(pts[pos].y);
            res.minim(l - pts[pos].y);
        } else {
            for i in 0..n {
                if mask.is_set(i) {
                    res.minim(mem.call(i, mask.without_bit(i)) + dist[pos][i]);
                }
            }
        }
        res
    });
    out.print_line(mem.call(n, usize::all_bits(n)));
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
