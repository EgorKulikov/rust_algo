//{"name":"Sequence","group":"Eolymp - Basecamp - Educational Round #10","url":"https://eolymp.com/en/compete/lce8ibks056gvennio8eubldr0/problem/4","interactive":false,"timeLimit":10000,"tests":[{"input":"4 3\n","output":"256\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_size();
    let mut n = input.read_size();

    if n == 1 {
        out.print_line(a);
        return;
    }
    let mut at = vec![None; 10_000];
    at[a % 10000] = Some(1);
    let mut cur = 1;
    let mut val = a % 10000;
    n -= 1;
    while n > 0 {
        val = (val * val) % 10000;
        cur += 1;
        n -= 1;
        if let Some(prev) = at[val] {
            n %= cur - prev;
        } else {
            at[val] = Some(cur);
        }
    }
    out.print_line(val);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
