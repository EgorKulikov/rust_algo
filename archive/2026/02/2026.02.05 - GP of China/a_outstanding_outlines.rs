//{"name":"A. Outstanding Outlines","group":"Universal Cup - GP of China","url":"https://contest.ucup.ac/contest/3295/problem/16328","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n-2 1\n0 0\n0 2\n2/1 0/1\n-2 1\n0 0\n0 2\n1/1 0/1\n0 1\n-2 1\n0 2\n1/2 7/4\n0 1\n-2 1\n0 2\n-3/2 3/4\n","output":"Yes\nNo\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::geometry::point::Point;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::rational::Rational;
use algo_lib::scan;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    type P = Point<Rational<i128>>;

    let a = P::new(
        Rational::new_int(input.read()),
        Rational::new_int(input.read()),
    );
    let b = P::new(
        Rational::new_int(input.read()),
        Rational::new_int(input.read()),
    );
    let c = P::new(
        Rational::new_int(input.read()),
        Rational::new_int(input.read()),
    );
    scan!(input, "@/@ @/@", x1: i128, x2: i128, y1: i128, y2: i128);
    let d = P::new(Rational::new(x1, x2), Rational::new(y1, y2));

    let ab = a.line(b);
    if ab.contains(d) {
        out.print_line(true);
        return;
    }
    let cc = ab.parallel(c);
    if cc.contains(d) {
        out.print_line(true);
        return;
    }

    if (ab.value(d) >= Rational::zero()) == (cc.value(d) >= Rational::zero()) {
        out.print_line(false);
        return;
    }

    let check = |c: P| -> bool {
        for (a, b) in [(a, b), (b, a)] {
            let dx = b.x - a.x;
            let dy = b.y - a.y;

            let line = a.line(c);

            let mut left = -1_000_000_000;
            let mut right = 1_000_000_000;

            let val = |t: i128| -> Rational<i128> {
                let p = P::new(
                    d.x + dx * Rational::new_int(t),
                    d.y + dy * Rational::new_int(t),
                );
                line.value(p)
            };
            let left_val = val(left) >= Rational::zero();
            while left < right {
                let mid = left + (right - left + 1) / 2;
                if (val(mid) >= Rational::zero()) == left_val {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }
            for i in left - 1..=left + 1 {
                if val(i) == Rational::zero() {
                    return true;
                }
            }
        }
        false
    };
    if check(c) {
        out.print_line(true);
        return;
    }
    if ab.is_perpendicular(a.line(c)) {
        let c2 = cc.intersect(ab.perpendicular(b));
        let left = a.line(c);
        let right = b.line(c2);
        if check(c2) && (left.value(d) >= Rational::zero()) == (right.value(d) >= Rational::zero())
        {
            out.print_line(true);
            return;
        }
    }
    if ab.is_perpendicular(b.line(c)) {
        let c2 = cc.intersect(ab.perpendicular(a));
        let left = a.line(c2);
        let right = b.line(c);
        if check(c2) && (left.value(d) >= Rational::zero()) == (right.value(d) >= Rational::zero())
        {
            out.print_line(true);
            return;
        }
    }
    out.print_line(false);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
