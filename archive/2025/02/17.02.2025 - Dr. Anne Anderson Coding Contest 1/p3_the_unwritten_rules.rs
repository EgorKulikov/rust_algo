//{"name":"P3 - The Unwritten Rules","group":"DMOJ - Dr. Anne Anderson Coding Contest 1","url":"https://dmoj.ca/problem/daacc1p3","interactive":false,"timeLimit":1000,"tests":[{"input":"2 2\n2\n1\n3\n4\n7\n6\n5\n8\n","output":"yes\n"},{"input":"2 1\n1\n3\n2\n4\n","output":"no\n"},{"input":"1 3\n3\n2\n5\n1\n4\n6\n","output":"no\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let order = input.read_size_vec(2 * n * m).dec();

    out.set_bool_output(BoolOutput::Custom("yes", "no"));
    for i in 0..n {
        let mut left = (i * 2 * m..i * 2 * m + m).collect::<Vec<_>>();
        let mut right = (i * 2 * m + m..i * 2 * m + 2 * m).rev().collect::<Vec<_>>();

        for &j in &order[i * 2 * m..i * 2 * m + 2 * m] {
            if left.last() == Some(&j) {
                left.pop();
            } else if right.last() == Some(&j) {
                right.pop();
            } else {
                out.print_line(false);
                return;
            }
        }
    }
    out.print_line(true);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
