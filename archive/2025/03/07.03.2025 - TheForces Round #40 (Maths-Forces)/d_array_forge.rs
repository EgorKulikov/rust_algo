//{"name":"D. Array Forge","group":"Codeforces - TheForces Round #40 (Maths-Forces)","url":"https://codeforces.com/gym/105767/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1 1 1\n1 1 2\n2 2 3\n3 4 5\n800000 900000 1000000\n1000000 1000000 1000000\n","output":"2\n6\n20\n384\n146046122\n463285462\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::inverse_factorials;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_utils::{factorials, PartialSums};

type Mod = ModIntF;
type PreCalc = (Vec<Mod>, Vec<Mod>);

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let x = input.read_size();
    let l = input.read_size();
    let r = input.read_size();
    out.print_line((data.0[r + 1] - data.0[l]) * (data.1[x + 1]));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = (
        factorials(1_000_001).partial_sums(),
        inverse_factorials(1_000_001).partial_sums(),
    );

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
