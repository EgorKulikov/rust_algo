//{"name":"C. Even Larger","group":"Codeforces - Codeforces Round 1045 (Div. 2)","url":"https://codeforces.com/contest/2134/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n4\n3 8 4 4\n4\n0 2 3 5\n2\n3 1\n5\n2 3 1 4 2\n4\n0 2 4 1\n5\n3 1 4 5 1\n11\n3 0 5 4 4 5 3 0 3 4 1\n12\n410748345 10753674 975233308 193331255 893457280 279719251 704970985 412553354 801228787 44181004 1000000000 3829103\n","output":"0\n1\n2\n0\n3\n6\n14\n4450984776\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_long_vec(n);

    let mut ans = 0;
    for i in (1..n).step_by(2) {
        if a[i - 1] > a[i] {
            ans += a[i - 1] - a[i];
            a[i - 1] = a[i];
        }
        if i + 1 < n && a[i + 1] + a[i - 1] > a[i] {
            ans += a[i + 1] + a[i - 1] - a[i];
            a[i + 1] = a[i] - a[i - 1];
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
