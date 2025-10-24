//{"name":"Minimum Increasing Cycle","group":"Eolymp - Basecamp - Blitz Round #4","url":"https://eolymp.com/en/compete/7mk1e6onrl4pb69590dkne46j4/problem/5","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3 1 5\n7 1 2\n5 2 3\n0 1 2 3 4\n5 1 6\n2 2 4 1 3\n6 3 10\n8 15 3 9 1 12\n6 2 10\n10 7 3 1 8 12\n","output":"1\n0\n2\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _m = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n);

    let mut delta = 0;
    for i in (0..n).rev() {
        if a[i] < k {
            break;
        }
        if i + 1 < n && a[i + 1] < a[i] {
            break;
        }
        delta += 1;
    }
    let b = a.iter_filter(|&x| x < k).collect::<Vec<_>>();
    let mut v = Vec::new();
    for i in b {
        let pos = v.upper_bound(&i);
        if pos == v.len() {
            v.push(i);
        } else {
            v[pos] = i;
        }
    }
    out.print_line(n - delta - v.len());
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
