//{"name":"Hitting the Targets","group":"Kattis","url":"https://open.kattis.com/problems/hittingtargets","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nrectangle 1 1 10 5\ncircle 5 0 8\nrectangle -5 3 5 8\n5\n1 1\n4 5\n10 10\n-10 -1\n4 -3\n","output":"2\n3\n0\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::geometry::circle::Circle;
use algo_lib::geometry::point::Point;
use algo_lib::io::input::{Input, Readable};
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    enum Target {
        Rectangle(Point<i64>, Point<i64>),
        Circle(Circle<i64>),
    }
    impl Readable for Target {
        fn read(input: &mut Input) -> Self {
            let tp = input.read_str();
            match tp.as_slice() {
                b"rectangle" => {
                    let p1 = input.read();
                    let p2 = input.read();
                    Target::Rectangle(p1, p2)
                }
                b"circle" => {
                    let p = input.read();
                    let r = input.read();
                    Target::Circle(Circle::new(p, r))
                }
                _ => unreachable!(),
            }
        }
    }
    impl Target {
        fn is_hit(&self, p: Point<i64>) -> bool {
            match self {
                Target::Rectangle(p1, p2) => {
                    p1.x <= p.x && p.x <= p2.x && p1.y <= p.y && p.y <= p2.y
                }
                Target::Circle(c) => c.contains(p),
            }
        }
    }

    let n = input.read_size();
    let targets: Vec<Target> = input.read_vec(n);
    let m = input.read_size();
    for _ in 0..m {
        let p = input.read();
        out.print_line(targets.iter().filter(|t| t.is_hit(p)).count());
    }
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
