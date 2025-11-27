//{"name":"God! Save me","group":"Eolymp - Basecamp - Educational Round #5","url":"https://eolymp.com/en/compete/saaq21a9v514tbj0spj7bmgjpk/problem/6","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3\n2 0.33\n-3 0.33\n-5 0.34\n3\n2 0.34\n-3 0.33\n-5 0.33\n3\n10 0.0\n-1 0.4\n-1 0.6\n","output":"Case 1: 10.151515\nCase 2: 9.764706\nCase 3: God! Save me\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::real::Real;
use std::io::Write;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let xp = input.read_vec::<(Real, Real)>(n);

    let mut a = Real::zero();
    let mut b = Real::zero();
    for (x, p) in xp {
        if x > 0 {
            b += p * x;
        } else {
            a += p;
            b += p * -x;
        }
    }
    if a == 1 {
        writeln!(out, "Case {}: God! Save me", test_case).unwrap();
    } else {
        writeln!(out, "Case {}: {:.6}", test_case, (b / (Real::one() - a)).0).unwrap();
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
