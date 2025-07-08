//{"name":"C. Coloring Game","group":"Codeforces - Educational Codeforces Round 180 (Rated for Div. 2)","url":"https://codeforces.com/contest/2112/problem/C","interactive":false,"timeLimit":2500,"tests":[{"input":"6\n3\n1 2 3\n4\n1 1 2 4\n5\n7 7 7 7 7\n5\n1 1 2 2 4\n6\n2 3 3 4 5 5\n5\n1 1 1 1 3\n","output":"0\n0\n10\n2\n16\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut ans = 0;
    for i in 0..n {
        for j in 0..i {
            let max = a.lower_bound(&(a[i] + a[j])) - 1;
            let min = a.upper_bound(&(a[n - 1] - a[i] - a[j])).max(i + 1);
            if max >= min {
                ans += max - min + 1;
            }
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
