//{"name":"C. Good Trail","group":"Codeforces - GoForGold Long Challenge - May 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/611602/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n10\n23\n120\n200\n333\n","output":"0\n0\n0\n1\n30\n2\n30 61\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut ans = Vec::new();
    for mut i in (125..=n).step_by(125) {
        let mut res = 0;
        let mut x = i;
        while x > 0 {
            res += x / 5;
            x /= 5;
        }
        let mut e = 0;
        while i % 5 == 0 {
            e += 1;
            i /= 5;
        }
        for d in (1..=e - 2).rev() {
            ans.push(res - d);
        }
    }
    out.print_line(ans.len());
    if !ans.is_empty() {
        out.print_line(ans);
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
