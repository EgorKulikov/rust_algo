//{"name":"A. Smallest Permutation","group":"SeriousOJ - Educational Round 1","url":"https://judge.eluminatis-of-lu.com/contest/686fe616d425270007014c27/1210","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2\n3\n4\n6\n10\n","output":"2 1\n2 3 1\n2 1 4 3\n2 1 4 3 6 5\n2 1 4 3 6 5 8 7 10 9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut ans = Vec::with_capacity(n);
    for i in (0..n.saturating_sub(3)).step_by(2) {
        ans.push(i + 1);
        ans.push(i);
    }
    if n % 2 == 1 {
        ans.push(n - 2);
        ans.push(n - 1);
        ans.push(n - 3);
    } else {
        ans.push(n - 1);
        ans.push(n - 2);
    }
    out.print_line(ans.inc());
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
