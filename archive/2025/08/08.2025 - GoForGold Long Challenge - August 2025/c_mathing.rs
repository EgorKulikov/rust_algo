//{"name":"C. Mathing","group":"Codeforces - GoForGold Long Challenge - August 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/636036/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n14 32\n5 10\n","output":"3 2 4\n-1 -1 -1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_int();
    let b = input.read_int();

    if a > b || (b - a) % 3 != 0 {
        out.print_line("-1 -1 -1");
        return;
    }
    let xyz = (b - a) / 3;
    let y2z = a - xyz;
    if y2z < 0 {
        out.print_line("-1 -1 -1");
        return;
    }
    let min_z = (y2z - xyz).max(0);
    let max_z = (y2z / 2).min(xyz);
    if min_z > max_z {
        out.print_line("-1 -1 -1");
        return;
    }
    out.print_line((max_z - min_z + 1, min_z, max_z));
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
