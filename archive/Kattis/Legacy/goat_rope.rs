//{"name":"Goat Rope","group":"Kattis","url":"https://open.kattis.com/problems/goatrope","interactive":false,"timeLimit":1000,"tests":[{"input":"7 3 0 0 5 4\n","output":"2.0\n"},{"input":"6 0 0 2 7 6\n","output":"2.0\n"},{"input":"3 -4 -3 -1 -1 2\n","output":"5.0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::geometry::point::Point;
use algo_lib::geometry::segment::Segment;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let goat: Point<Real> = input.read();
    let x1 = input.read_real();
    let y1 = input.read_real();
    let x2 = input.read_real();
    let y2 = input.read_real();

    let mut segs = Vec::with_capacity(4);
    segs.push(Segment::new(Point::new(x1, y1), Point::new(x1, y2)));
    segs.push(Segment::new(Point::new(x1, y2), Point::new(x2, y2)));
    segs.push(Segment::new(Point::new(x2, y2), Point::new(x2, y1)));
    segs.push(Segment::new(Point::new(x2, y1), Point::new(x1, y1)));

    let mut ans = None;
    for seg in segs {
        ans.minim(seg.dist_point(goat));
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
