//{"name":"C. Rabbits","group":"Codeforces - Codeforces Global Round 29 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2147/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"12\n4\n0100\n3\n000\n8\n11011011\n5\n00100\n1\n1\n5\n01011\n2\n01\n7\n0101011\n7\n1101010\n5\n11001\n4\n1101\n9\n001101100\n","output":"YES\nYES\nNO\nYES\nYES\nYES\nYES\nYES\nYES\nYES\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut start = n;

    for i in 0..n {
        if s[i] == b'0' && i > 0 && s[i - 1] == b'1' && (i == 1 || s[i - 2] == b'1') {
            start = i;
        }
        if start != n && (((i - start) % 2 == 0) ^ (s[i] == b'0')) {
            start = n;
        }
        if start != n && i + 1 < n && s[i + 1] == b'1' && (i + 2 == n || s[i + 2] == b'1') {
            let len = i - start;
            if len / 2 % 2 == 0 {
                out.print_line(false);
                return;
            }
        }
    }
    out.print_line(true);
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
