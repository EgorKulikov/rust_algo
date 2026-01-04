//{"name":"coderun21","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::geometry::circle::Circle;
use algo_lib::geometry::point::Point;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::{Random, RandomTrait};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{IntoReal, Real};
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let pts = input.read_vec::<Point<Real>>(n);

    let mut r = Random::new_with_seed(239);
    loop {
        let dx = r.gen_impl().into_real() / u64::MAX.into_real() * 2;
        let dy = r.gen_impl().into_real() / u64::MAX.into_real() * 2;

        let mut set = BTreeSet::default();
        let mut good = true;
        for p in pts.copy_iter() {
            let ry = ((p.y - dy) / 3.into_real().sqrt()).round();
            let rx = ((p.x - dx) / 2).round();

            let mut found = false;
            'outer: for i in -2..=2 {
                for j in -2..=2 {
                    let cy = 3.into_real().sqrt() * (ry + j) + dy;
                    let cx = if (ry + j) % 2 == 0 {
                        (rx + i).into_real() * 2 + dx
                    } else {
                        (rx + i).into_real() * 2 + 1 + dx
                    };
                    let circle = Circle::new(Point::new(cx, cy), 1.into_real());
                    if circle.contains(p) {
                        set.insert(Point::new(cx, cy));
                        found = true;
                        break 'outer;
                    }
                }
            }
            if !found {
                good = false;
                break;
            }
        }
        if good {
            out.print_line(true);
            out.print_line(set.len());
            out.print_per_line_iter(set.iter());
            return;
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
