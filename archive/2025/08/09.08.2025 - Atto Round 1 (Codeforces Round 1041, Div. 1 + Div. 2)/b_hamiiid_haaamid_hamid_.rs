//{"name":"B. Hamiiid, Haaamid... Hamid?","group":"Codeforces - Atto Round 1 (Codeforces Round 1041, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2127/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3 1\n..#\n4 2\n....\n5 3\n##..#\n6 4\n#...#.\n","output":"1\n1\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_size() - 1;
    let s = input.read_str();

    fn check(s: Str, x: usize) -> usize {
        let mut left = 0;
        for i in 0..x {
            if s[i] == b'#' {
                left = i + 1;
            }
        }
        let mut right = 0;
        for i in ((x + 1)..s.len()).rev() {
            if s[i] == b'#' {
                right = s.len() - i;
            }
        }
        left.min(right)
    }
    let mut ans = 0;
    if x != 0 {
        let mut s = s.clone();
        s[x - 1] = b'#';
        ans.maxim(check(s, x));
    }
    if x != n - 1 {
        let mut s = s.clone();
        s[x + 1] = b'#';
        ans.maxim(check(s, x));
    }
    out.print_line(ans + 1);
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
