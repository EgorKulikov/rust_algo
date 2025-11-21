//{"name":"Chef and String","group":"CodeChef - START213A","url":"https://www.codechef.com/START213A/problems/WENOTI","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3 2\nIAI\n5 1\nCILLI\n5 3\nCILLI\n7 2\nEXUNITE\n5 1000000000\nIIIII\n20 3\nILOVEEXUNWENOTIIIIII\n","output":"5\n3\n9\n3\n4999999999\n24\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let mut s = input.read_str();

    let mut last = None;
    for i in 0..n {
        if s[i] != b'I' {
            last = Some(s[i]);
        } else if let Some(c) = last {
            s[i] = c;
        }
    }
    for i in (0..n).rev() {
        if s[i] != b'I' {
            last = Some(s[i]);
        } else if let Some(c) = last {
            s[i] = c;
        }
    }
    let mut ans = 0;
    for (a, b) in s.consecutive_iter_copy() {
        if a == b {
            ans += k;
        }
    }
    if s[0] == s[Back(0)] {
        ans += k - 1;
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
