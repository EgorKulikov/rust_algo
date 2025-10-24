//{"name":"C. Trip Shopping","group":"Codeforces - Atto Round 1 (Codeforces Round 1041, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2127/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2 1\n1 7\n3 5\n3 2\n1 5 3\n6 2 4\n5 4\n1 16 10 10 16\n3 2 2 15 15\n4 1\n23 1 18 4\n19 2 10 3\n10 10\n4 3 2 100 4 1 2 4 5 5\n1 200 4 5 6 1 10 2 3 4\n","output":"8\n9\n30\n16\n312\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _k = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);

    let ab = a
        .into_iter()
        .zip(b.into_iter())
        .collect::<Vec<_>>()
        .sorted();
    let (a, b) = ab.detuple();
    let mut ans = 0;
    for i in 0..n {
        ans += (a[i] - b[i]).abs();
    }
    let mut delta = i64::MAX;
    for i in 1..n {
        let cur = (a[i] - b[i]).abs() + (a[i - 1] - b[i - 1]).abs();
        let x = vec![a[i], b[i], a[i - 1], b[i - 1]].sorted();
        let next = x[3] + x[2] - x[1] - x[0];
        delta.minim(next - cur);
    }
    out.print_line(ans + delta);
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
