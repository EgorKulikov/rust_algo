//{"name":"Jabuke","group":"Kattis","url":"https://open.kattis.com/problems/jabuke","interactive":false,"timeLimit":1000,"tests":[{"input":"1 1\n5 1\n3 3\n4\n3 1\n3 2\n3 3\n3 4\n","output":"4.0\n3\n"},{"input":"3 2\n5 4\n1 6\n3\n2 4\n3 5\n4 3\n","output":"6.0\n3\n"},{"input":"2 6\n5 1\n7 8\n5\n1 4\n3 5\n6 4\n6 5\n4 7\n","output":"15.5\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::geometry::point::Point;
use algo_lib::geometry::polygon::Polygon;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::IntoReal;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let triangle: Vec<Point<i64>> = input.read_vec(3);
    let n = input.read_size();
    let points: Vec<Point<i64>> = input.read_vec(n);

    let triangle = Polygon::new(triangle);
    out.set_precision(1);
    let area = triangle.double_area().abs();
    out.print_line(area.into_real() / 2);
    out.print_line(points.iter_filter(|p| triangle.contains(*p)).count());
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
