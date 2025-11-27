//{"name":"So you want to be a 2^n-aire?","group":"Eolymp - Basecamp - Educational Round #5","url":"https://eolymp.com/en/compete/saaq21a9v514tbj0spj7bmgjpk/problem/10","interactive":false,"timeLimit":1000,"tests":[{"input":"1 0.5\n1 0.3\n2 0.6\n24 0.25\n0 0\n","output":"1.500\n1.357\n2.560\n230.138\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let t = input.read_real();

    if n == 0 {
        return;
    }

    let mut mem = Memoization1d::new(n + 1, |mem, pos| {
        if pos == n {
            Real::one()
        } else {
            let prize = mem.call(pos + 1) * 2;
            let start = (Real::one() / prize).max(t);
            let mut res = Real::zero();
            if start > t {
                res += start - t;
            }
            res += prize / 2;
            res -= prize / 2 * start * start;
            res /= Real::one() - t;
            res
        }
    });
    out.print_line(mem.call(0));
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    output.set_precision(3);
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
