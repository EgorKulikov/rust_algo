//{"name":"Pervasive Heart Monitor","group":"Kattis","url":"https://open.kattis.com/problems/pervasiveheartmonitor","interactive":false,"timeLimit":1000,"tests":[{"input":"Lisa Marie Presley 90.2 104.3 110.1 118.7 122.3\n72.2 74 79.5 82.1 88.3 87.4 87.2 88.1 83.8 Bono\n","output":"109.120000 Lisa Marie Presley\n82.511111 Bono\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::io::scan::Parse;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::real::Real;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let line = input.read_line();
    let tokens = line.split(|x| *x == b' ').collect::<Vec<_>>();

    let mut name = Vec::new();
    let mut sum = Real::zero();
    let mut count = 0;
    for token in tokens {
        if token[0].is_ascii_digit() {
            sum += token.parse::<Real>();
            count += 1;
        } else {
            name.push(Str::from(token));
        }
    }
    out.print_line((sum / count, name));
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
