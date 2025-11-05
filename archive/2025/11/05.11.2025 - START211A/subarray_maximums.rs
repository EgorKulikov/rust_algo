//{"name":"Subarray Maximums","group":"CodeChef - START211A","url":"https://www.codechef.com/START211A/problems/MXPERST","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n101\n3\n011\n4\n1011\n5\n10101\n","output":"2 3 1\n-1\n4 1 3 2\n4 5 2 3 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    if s[0] == b'0' || s[n - 1] == b'0' {
        out.print_line(-1);
        return;
    }
    let mut left = vec![1];
    let mut right = Vec::new();
    for i in 1..n {
        if s[i] == b'1' {
            left.push(i + 1);
            left.extend_from_slice(&right);
            right.clear();
        } else {
            right.push(i + 1);
        }
    }
    out.print_line(left);
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
        TaskType::Classic | TaskType::RunTwice => input.is_empty(),
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
