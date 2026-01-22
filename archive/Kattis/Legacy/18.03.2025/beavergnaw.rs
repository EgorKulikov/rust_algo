//{"name":"Beavergnaw","group":"Kattis","url":"https://open.kattis.com/problems/beavergnaw","interactive":false,"timeLimit":1000,"tests":[{"input":"10 250\n20 2500\n25 7000\n50 50000\n0 0\n","output":"8.054498576\n14.774938880\n13.115314879\n30.901188723\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::bin_search::search_real;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let d = input.read_real();
    let v = input.read_real();
    if d == Real(0.) && v == Real(0.) {
        return;
    }

    let total: Real = Real::PI / 4 * d * d * d;

    out.print_line(search_real(Real(0.), d, |dd| {
        let cone = Real::PI / 24 * (d * d * d - dd * dd * dd);
        let cylinder = Real::PI / 4 * dd * dd * dd;
        total <= v + 2 * cone + cylinder
    }));
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
