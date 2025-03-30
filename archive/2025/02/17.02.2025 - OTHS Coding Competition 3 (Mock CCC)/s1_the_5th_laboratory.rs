//{"name":"S1 - The 5th Laboratory","group":"DMOJ - OTHS Coding Competition 3 (Mock CCC)","url":"https://dmoj.ca/problem/othscc3p1","interactive":false,"timeLimit":3000,"tests":[{"input":"1 1 1\n2\n5 4 1\n1 2 -5\n","output":"2.5\n"},{"input":"1 1 1\n2\n1 2 1\n1 1 1\n","output":"0\n"},{"input":"0 0 0\n2\n0 7 4\n0 3 8\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let cx = input.read_real();
    let cy = input.read_real();
    let cz = input.read_real();
    let n = input.read_size();

    let mut ans = Real(f64::INFINITY);
    for _ in 0..n {
        let x = input.read_real();
        let y = input.read_real();
        let z = input.read_real();
        let dist = Real::hypot(cx - x, cy - y);
        let cur = if z < cz {
            cz - z + dist / 2
        } else {
            (z - cz) / 4 + Real(0.).max(dist - (z - cz) * 3 / 4) / 2
        };
        ans.minim(cur);
    }
    out.print_line(ans);
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
