//{"name":"Longest Palindrome","group":"CodeChef - Practice for ZCO and INOI","url":"https://www.codechef.com/practice/course/zco-inoi-problems/IARCSJUD/problems/LONGPALI","interactive":false,"timeLimit":1000,"tests":[{"input":"5\nabbba\n","output":"5\nabbba\n"},{"input":"12\nabcbcabbacba\n","output":"8\nbcabbacb\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};
use algo_lib::string::string_algorithms::palindromes::Palindromes;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let s = input.read_str();

    let mut ans = None;
    let ep = s.even_palindromes();
    for i in ep.indices() {
        ans.maxim((2 * ep[i], i - ep[i], i + ep[i]));
    }
    let op = s.odd_palindromes();
    for i in op.indices() {
        ans.maxim((2 * op[i] - 1, i + 1 - op[i], i + op[i]));
    }
    let (_, l, r) = ans.unwrap();
    out.print_line(r - l);
    out.print_line(Str::from(&s[l..r]));
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
