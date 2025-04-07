//{"name":"D. Arcology On Permafrost","group":"Codeforces - Teza Round 1 (Codeforces Round 1015, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2084/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n2 1 1\n5 2 2\n6 1 4\n8 2 2\n8 1 5\n11 3 3\n22 6 3\n17 2 2\n","output":"0 0\n0 1 0 0 0\n0 1 2 2 0 1\n0 2 1 0 1 0 8 1\n0 1 2 1000000000 1 0 1 2\n1 0 0 1 0 2 1 0 2 1 0\n0 2 1 0 2 1 0 3 2 1 0 2 1 0 2 1 0 2 1 0 2 1\n4 0 2 1 3 4 0 2 1 0 3 4 0 1 2 1 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();

    let a = (n / (m + 1)).min(n - k * m);
    let good = a * (m + 1);
    let mut rem = n - good;
    let mut ans = Vec::with_capacity(n);
    for i in 0..=m {
        for j in 0..a {
            ans.push(j);
        }
        if i != m {
            let space = rem / (m - i);
            for _ in 0..space {
                ans.push(0);
            }
            rem -= space;
        }
    }
    assert_eq!(ans.len(), n);
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
