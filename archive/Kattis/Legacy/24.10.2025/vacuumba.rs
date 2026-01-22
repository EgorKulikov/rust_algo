//{"name":"Vacuumba","group":"Kattis","url":"https://open.kattis.com/problems/vacuumba","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2\n0 10.5\n90 5\n1\n-45 10\n","output":"-5.000000 10.500000\n7.071068 7.071068\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut x = Real::zero();
    let mut y = Real::zero();
    let mut dx = Real::zero();
    let mut dy = Real::one();

    for _ in 0..n {
        let a = input.read_real().to_radians();
        let l = input.read_real();
        (dx, dy) = (dx * a.cos() - dy * a.sin(), dx * a.sin() + dy * a.cos());
        x += dx * l;
        y += dy * l;
    }
    out.print_line((x, y));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
