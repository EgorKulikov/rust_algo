//{"name":"F - ABCBA","group":"AtCoder - UNIQUE VISION Programming Contest 2025 Spring (AtCoder Beginner Contest 398)","url":"https://atcoder.jp/contests/abc398/tasks/abc398_f","interactive":false,"timeLimit":2000,"tests":[{"input":"ABC\n","output":"ABCBA\n"},{"input":"Z\n","output":"Z\n"},{"input":"TREE\n","output":"TREERT\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};
use algo_lib::string::string_algorithms::palindromes::Palindromes;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    let op = s.odd_palindromes();
    let ep = s.even_palindromes();

    for i in (1..=s.len()).rev() {
        if i % 2 == 0 {
            if ep[s.len() - i / 2] >= i / 2 {
                out.print(&s);
                let mut pref = Str::from(&s[..s.len() - i]);
                pref.reverse();
                out.print_line(pref);
                return;
            }
        } else {
            if op[s.len() - i / 2 - 1] >= i / 2 + 1 {
                out.print(&s);
                let mut pref = Str::from(&s[..s.len() - i]);
                pref.reverse();
                out.print_line(pref);
                return;
            }
        }
    }
    unreachable!();
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
