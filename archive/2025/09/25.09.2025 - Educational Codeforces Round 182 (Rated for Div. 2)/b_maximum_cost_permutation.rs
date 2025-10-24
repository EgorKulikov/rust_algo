//{"name":"B. Maximum Cost Permutation","group":"Codeforces - Educational Codeforces Round 182 (Rated for Div. 2)","url":"https://codeforces.com/contest/2144/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5\n1 0 4 0 5\n3\n0 0 0\n4\n1 2 3 0\n3\n0 3 2\n","output":"3\n3\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut p = input.read_size_vec(n);

    if p.copy_count(0) == 1 {
        let s = p.copy_sum();
        for i in 0..n {
            if p[i] == 0 {
                p[i] = n * (n + 1) / 2 - s;
                break;
            }
        }
    }
    let mut first = None;
    let mut last = None;
    for i in 0..n {
        if p[i] != i + 1 {
            if first.is_none() {
                first = Some(i);
            }
            last = Some(i);
        }
    }
    if first == last {
        out.print_line(0);
    } else {
        out.print_line(last.unwrap() - first.unwrap() + 1);
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
