//{"name":"B. Deck of Cards","group":"Codeforces - Educational Codeforces Round 183 (Rated for Div. 2)","url":"https://codeforces.com/contest/2145/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 2\n01\n3 2\n22\n1 1\n2\n7 5\n01201\n","output":"-++-\n???\n-\n--?+?--\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let s = input.read_str();

    if n == k {
        out.print_line(Str::from(vec![b'-'; n]));
        return;
    }
    let mut ans = Str::from(vec![b'+'; n]);
    let zero = s.copy_count(b'0');
    let one = s.copy_count(b'1');
    let two = s.copy_count(b'2');
    ans[..zero].fill(b'-');
    ans[n - one..].fill(b'-');
    ans[zero..zero + two].fill(b'?');
    ans[n - one - two..n - one].fill(b'?');
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
