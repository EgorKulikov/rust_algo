//{"name":"C. Monocarp's String","group":"Codeforces - Educational Codeforces Round 183 (Rated for Div. 2)","url":"https://codeforces.com/contest/2145/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5\nbbbab\n6\nbbaaba\n4\naaaa\n12\naabbaaabbaab\n5\naabaa\n","output":"3\n0\n-1\n2\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut pos = DefaultHashMap::new(Vec::new());
    let mut balance = 0;
    pos[balance].push(n);
    for i in (0..n).rev() {
        if s[i] == b'a' {
            balance += 1;
        } else {
            balance -= 1;
        }
        pos[balance].push(i);
    }

    let mut ans = n;
    let all = balance;
    for i in 0..n {
        if let Some(&p) = pos[balance - all].last() {
            ans.minim(p - i);
        }
        pos[balance].pop();
        if s[i] == b'a' {
            balance -= 1;
        } else {
            balance += 1;
        }
    }
    if ans == n {
        out.print_line(-1);
    } else {
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
