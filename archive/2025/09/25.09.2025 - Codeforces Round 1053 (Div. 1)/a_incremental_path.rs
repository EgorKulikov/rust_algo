//{"name":"A. Incremental Path","group":"Codeforces - Codeforces Round 1053 (Div. 1)","url":"https://codeforces.com/contest/2150/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3 2\nBAB\n2 5\n3 4\nABA\n1 4 9 10\n5 2\nABABB\n1 7\n3 1\nBBA\n6\n1 4\nA\n1 3 4 1000000000\n","output":"4\n2 3 5 6\n7\n1 2 3 4 6 9 10\n7\n1 2 3 5 6 7 9\n3\n2 4 6\n5\n1 2 3 4 1000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let m = input.read_size();
    let s = input.read_str();
    let a = input.read_size_vec(m);

    let mut at = 1;
    let mut ans = a.into_iter().collect::<FxHashSet<_>>();
    for c in s {
        if c == b'A' {
            at += 1;
        } else {
            at += 1;
            while ans.contains(&at) {
                at += 1;
            }
        }
        ans.insert(at);
        if c == b'B' {
            while ans.contains(&at) {
                at += 1;
            }
        }
    }
    let ans = ans.into_iter().collect::<Vec<_>>().sorted();
    out.print_line(ans.len());
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
