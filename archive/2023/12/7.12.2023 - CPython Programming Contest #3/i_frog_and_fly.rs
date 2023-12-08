//{"name":"I. Frog and fly","group":"CPython.uz - CPython Programming Contest #3","url":"https://cpython.uz/competitions/contests/contest/326/problem/I","interactive":false,"timeLimit":1000,"tests":[{"input":"8 -6 8 60 0 7\n","output":"Poor frog\n"},{"input":"8 -6 8 60 0 10\n","output":"Poor fly0.00000\n"},{"input":"8 -6 8 60 0 8\n","output":"Poor fly6.00000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IFrogAndFly"}}}

use algo_lib::geometry::point::Point;
use algo_lib::geometry::segment::Segment;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::real::{IntoReal, Real, RealTrait};
use algo_lib::numbers::rational::{Rational, ToRational};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let x1 = input.read_long().rat();
    let y1 = input.read_long().rat();
    let x2 = input.read_long().rat();
    let y2 = input.read_long().rat();
    let x = input.read_long().rat();
    let y = input.read_long().rat();
    let l = input.read_long().rat();

    // let fly = Segment::new(Point::new(x1, y1), Point::new(x2, y2));
    let frog = Point::new(x, y);
    if l * l >= Point::new(x1, y1).square_dist_point(frog) {
        out.print_line("Poor fly");
        out.print_line("0.00000");
        return;
    }
    let x = x.num().into_real();
    let y = y.num().into_real();
    let x1 = x1.num().into_real();
    let y1 = y1.num().into_real();
    let x2 = x2.num().into_real();
    let y2 = y2.num().into_real();
    let l = l.num().into_real();
    let frog = Point::new(x, y);
    let fly = Segment::new(Point::new(x1, y1), Point::new(x2, y2));
    let dist = fly.dist_point(frog);
    if dist > l + Real::epsilon() {
        out.print_line("Poor frog");
        return;
    }
    let fly = fly.line();
    let perp = fly.perpendicular(frog);
    let mid = perp.intersect(fly);
    let rem = (l * l - mid.square_dist_point(frog)).sqrt();
    let ans = Point::new(x1, y1).dist_point(mid) - rem;
    out.print_line("Poor fly");
    out.print_line(format!("{:.5}", ans.0));
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test(run, tester::check);
}
//END MAIN
