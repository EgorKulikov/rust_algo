//{"name":"Marching Orders","group":"CodeChef - START211A","url":"https://www.codechef.com/START211A/problems/MRCHORD","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 2 3\n3\n3 2 1\n5\n3 4 2 3 4\n6\n4 5 6 1 2 3\n","output":"Yes\nNo\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    let mut start = 0;
    while start < n {
        if start + 2 >= n {
            out.print_line(false);
            return;
        }
        let next = a[start].max(a[start + 1]).max(a[start + 2]);
        if next < start + 2 {
            out.print_line(false);
            return;
        }
        for i in start..=next {
            let delta = next - i;
            let rem = n - delta;
            let target = next - 2 + (rem + 2) % 3;
            if a[i] != target {
                out.print_line(false);
                return;
            }
        }
        start = next + 1;
        while start < n {
            let mut steps = (start - next).upper_div(2);
            let pos = start - 2 * steps;
            if pos != next {
                steps += 1;
            }
            let rem = n - steps;
            let target = next - 2 + (rem + 2) % 3;
            if a[start] != target {
                break;
            }
            start += 1;
        }
    }
    out.print_line(true);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
        TaskType::Classic | TaskType::RunTwice => input.is_empty(),
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
