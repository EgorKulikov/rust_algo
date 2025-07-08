//{"name":"D. Matrix game","group":"Codeforces - Codeforces Round 1033 (Div. 2) and CodeNite 2025","url":"https://codeforces.com/contest/2120/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 1 5\n2 2 2\n90000 80000 70000\n","output":"1 1\n3 7\n299929959 603196135\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::One;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_size();
    let b = input.read_size();
    let k = input.read_size();

    type Mod = ModInt7;
    let n = (a - 1) * k + 1;
    let mut m = Mod::one();
    for i in 1..=a {
        m *= n - i + 1;
        m /= i;
    }
    m *= b - 1;
    m *= k;
    m += 1;
    out.print_line((Mod::from(n), m));
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
