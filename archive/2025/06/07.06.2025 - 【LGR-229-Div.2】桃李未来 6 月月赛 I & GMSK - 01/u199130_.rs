//{"name":"U199130 外人","group":"Luogu","url":"https://www.luogu.com.cn/problem/U199130?contestId=233396","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 2 3\n3 1 2\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::geometry::point::Point;
use algo_lib::geometry::segment::{Segment, SegmentIntersectionResult};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::rational::Rational;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let y = input.read_long_vec(n);
    let s = input.read_long_vec(n);

    let mut ans = 0;
    for i in 0..n - 1 {
        let s1 = Segment::new(
            Point::new(Rational::new_int(y[i]), Rational::zero()),
            Point::new(Rational::new_int(y[i + 1]), Rational::one()),
        );
        let s2 = Segment::new(
            Point::new(Rational::new_int(s[i]), Rational::zero()),
            Point::new(Rational::new_int(s[i + 1]), Rational::one()),
        );
        if s1.intersect_segment(s2) != SegmentIntersectionResult::None {
            ans += 1;
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
