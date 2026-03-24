//{"name":"A - Closeness of Fireworks","group":"AtCoder - AtCoder Weekday Contest 0032 Beta","url":"https://atcoder.jp/contests/awc0032/tasks/awc0032_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 5\n0 0 3 4\n0 0 6 0\n1 1 1 1\n","output":"2\n"},{"input":"2 1\n0 0 10 10\n5 5 0 0\n","output":"0\n"},{"input":"6 10\n0 0 3 4\n0 0 8 6\n0 0 11 0\n5 5 5 5\n-3 4 3 -4\n100 100 0 0\n","output":"4\n"},{"input":"10 1000000000\n0 0 600000000 800000000\n0 0 600000001 800000000\n1000000000 1000000000 -1000000000 -1000000000\n0 0 0 0\n-500000000 -500000000 -500000000 -500000000\n999999999 0 0 0\n0 0 0 1000000000\n100 200 300 400\n-1000000000 0 1000000000 0\n500000000 500000000 500000001 500000001\n","output":"7\n"},{"input":"1 1\n0 0 1 0\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::geometry::point::Point;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let r = input.read_long();
    let xypq = input.read_vec::<(i64, i64, i64, i64)>(n);

    let mut ans = 0;
    for (x, y, p, q) in xypq {
        let p1 = Point::new(x, y);
        let p2 = Point::new(p, q);
        if p1.square_dist_point(p2) <= r * r {
            ans += 1;
        }
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
