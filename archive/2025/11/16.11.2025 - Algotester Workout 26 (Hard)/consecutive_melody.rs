//{"name":"Consecutive Melody","group":"Algotester","url":"https://algotester.com/en/ContestProblem/DisplayWithFile/147289","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n1 -1 2\n","output":"9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_long();
    let a = input.read_long_vec(n);

    let mut has_fixed = false;
    let mut fixed = 0;
    let mut len_fixed = 0;
    let mut any = 0;
    let mut ans = 0;
    for i in a {
        if i == -1 {
            if has_fixed {
                fixed += 1;
                if fixed <= k {
                    ans += len_fixed;
                }
            }
            ans += (2 * k + 2 - any) * (any + 1) / 2;
            any += 1;
            any.minim(k);
        } else {
            if has_fixed {
                fixed += 1;
                if fixed == i {
                    len_fixed += any + 1;
                } else {
                    fixed = i;
                    len_fixed = any.min(i) + 1;
                }
            } else {
                fixed = i;
                len_fixed = any.min(i) + 1;
                has_fixed = true;
            }
            ans += len_fixed;
            any = 0;
        }
    }
    out.print_line(ans);
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
