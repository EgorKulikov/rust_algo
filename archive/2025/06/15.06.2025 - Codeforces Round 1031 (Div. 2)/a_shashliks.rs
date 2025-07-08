//{"name":"A. Shashliks","group":"Codeforces - Codeforces Round 1031 (Div. 2)","url":"https://codeforces.com/contest/2113/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n10 3 4 2 1\n1 10 10 1 1\n100 17 5 2 3\n28 14 5 2 4\n277 5 14 1 3\n","output":"8\n0\n46\n10\n273\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let k = input.read_long();
    let a = input.read_long();
    let b = input.read_long();
    let x = input.read_long();
    let y = input.read_long();

    let mut ans = 0;
    for (a, b, x, y) in [(a, b, x, y), (b, a, y, x)] {
        let mut cur = 0;
        let mut temp = k;
        if temp >= a {
            let times = (temp - a) / x + 1;
            cur += times;
            temp -= times * x;
        }
        if temp >= b {
            let times = (temp - b) / y + 1;
            cur += times;
        }
        ans.maxim(cur);
    }
    out.print_line(ans);
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
