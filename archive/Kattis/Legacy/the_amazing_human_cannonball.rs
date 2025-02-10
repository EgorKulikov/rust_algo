//{"name":"The Amazing Human Cannonball","group":"Kattis","url":"https://open.kattis.com/problems/humancannonball2","interactive":false,"timeLimit":1000,"tests":[{"input":"11\n19 45 20 9 12\n20 45 20 9 12\n25 45 20 9 12\n20 43 20 9 12\n20 47.5 20 9 12\n20 45 17 9 12\n20 45 24 9 12\n20 45 20 10 12\n20 45 20 9 11\n20 45 20 9.0 11.5\n20 45 18.1 9 12\n","output":"Not Safe\nSafe\nNot Safe\nNot Safe\nNot Safe\nNot Safe\nNot Safe\nNot Safe\nNot Safe\nSafe\nSafe\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TheAmazingHumanCannonball"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let v0 = input.read_real();
    let theta: Real = input.read_real() / 180 * Real::PI;
    let x = input.read_real();
    let h1 = input.read_real();
    let h2 = input.read_real();

    let t = x / (v0 * theta.cos());
    let y = v0 * t * theta.sin() - 0.5 * 9.81 * t * t;
    if h1 <= y - 1 && y + 1 <= h2 {
        out.print_line("Safe");
    } else {
        out.print_line("Not Safe");
    }
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}

#[cfg(test)]
mod tester;
//END MAIN
