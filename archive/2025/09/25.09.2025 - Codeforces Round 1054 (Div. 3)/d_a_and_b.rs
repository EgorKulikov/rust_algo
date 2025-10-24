//{"name":"D. A and B","group":"Codeforces - Codeforces Round 1054 (Div. 3)","url":"https://codeforces.com/contest/2149/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\nabab\n6\nbababa\n7\nabababa\n2\nab\n1\nb\n","output":"1\n2\n2\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut a = Vec::new();
    let mut b = Vec::new();
    for i in 0..n {
        if s[i] == b'a' {
            a.push(i);
        } else {
            b.push(i);
        }
    }
    let sa = a.partial_sums();
    let sb = b.partial_sums();

    let mut ans = None;
    for s in [sa, sb] {
        for i in s.indices() {
            let j = s.len() - 1 - i;
            let left = s[i] - i * i.saturating_sub(1) / 2;
            let right = (2 * n - j - 1) * j / 2 - (s[s.len() - 1] - s[i]);
            ans.minim(left + right);
        }
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
