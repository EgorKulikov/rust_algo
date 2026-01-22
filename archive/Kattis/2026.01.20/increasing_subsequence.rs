//{"name":"Increasing Subsequence","group":"Kattis","url":"https://open.kattis.com/problems/increasingsubsequence","interactive":false,"timeLimit":1000,"tests":[{"input":"4 1 25 2 3\n4 1 2 2 3\n8 90 4 10000 2 18 60 172 99\n0\n","output":"3 1 2 3\n3 1 2 3\n4 2 18 60 99\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    if n == 0 {
        return;
    }
    let a = input.read_int_vec(n);

    let mut last = Vec::new();
    let mut res = vec![Vec::new()];
    for i in a {
        let pos = last.lower_bound(&i);
        if pos == last.len() {
            last.push(i);
            res.push(res[pos].clone());
        } else {
            last[pos] = i;
            res[pos + 1] = res[pos].clone();
        }
        res[pos + 1].push(i);
    }
    out.print_line((last.len(), &res[last.len()]));
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
