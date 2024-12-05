//{"name":"F - Pookie-Mon Again","group":"LightOJ","url":"https://lightoj.com/contest/injpc-2024-replay/arena/problem/6457","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n0 6\n9 0\n3 4\n0 4\n3 0\n0 15\n21 0\n9 12\n0 10\n7 0\n","output":"Yes 12\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FPookieMonAgain"}}}

use algo_lib::geometry::point::Point;
use algo_lib::geometry::segment::Segment;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a: Point<i64> = input.read();
    let b: Point<i64> = input.read();
    let c: Point<i64> = input.read();
    let d: Point<i64> = input.read();
    let e: Point<i64> = input.read();

    let ans = Segment::new(a, b).contains(c) && a != c && b != c;
    if ans {
        out.print_line((true, (c.x - d.x).abs() * (c.y - e.y).abs()));
    } else {
        out.print_line(false);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
