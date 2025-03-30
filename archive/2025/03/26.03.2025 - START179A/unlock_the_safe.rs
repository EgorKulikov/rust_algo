//{"name":"Unlock The Safe","group":"CodeChef - START179A","url":"https://www.codechef.com/START179A/problems/UNLOCKSAFE","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n3 2\n1 2 3\n2 1 4\n3 7\n1 2 1\n2 1 9\n2 1\n5 6\n5 6\n2 9\n5 6\n5 6\n4 10\n4 6 1 7\n6 5 8 9\n4 12\n4 6 1 7\n6 5 8 9\n","output":"No\nYes\nNo\nYes\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_int();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);

    let mut min = 0;
    let mut min_delta = 9;
    for i in 0..n {
        let cur = (a[i] - b[i]).abs();
        let alt = 9 - cur;
        let delta = cur.min(alt);
        min += delta;
        min_delta.minim(cur.max(alt) - delta);
    }
    if min % 2 == k % 2 {
        out.print_line(min <= k);
        return;
    }
    min += min_delta;
    out.print_line(min <= k);
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
