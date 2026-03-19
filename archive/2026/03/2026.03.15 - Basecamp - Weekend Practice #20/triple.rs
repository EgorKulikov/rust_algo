//{"name":"Triple","group":"Eolymp - Basecamp - Weekend Practice #20","url":"https://eolymp.com/en/compete/4pe1cne4r571j089budf65juq4/problem/2","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n4\n1 2 3 3\n","output":"0\n"},{"input":"1\n4\n1 2 3 4\n","output":"1\n"},{"input":"1\n3\n1 2 4\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_long_vec(n);

    let sum = s.copy_sum();
    let mut cur = 0;
    let mut seen = FxHashSet::default();
    let mut ans1 = false;
    for i in s.copy_iter() {
        cur += i;
        if cur == sum {
            break;
        }
        if cur % 2 == 0 && seen.contains(&(cur / 2)) {
            if 3 * cur / 2 == sum {
                out.print_line(0);
                return;
            }
            ans1 = true;
        }
        seen.insert(cur);
    }
    let mut seen2 = FxHashSet::default();
    cur = 0;
    for i in s.copy_rev() {
        cur += i;
        if cur == sum {
            break;
        }
        if cur % 2 == 0 && seen2.contains(&(cur / 2)) {
            ans1 = true;
        }
        if seen.contains(&cur) && 2 * cur < sum {
            ans1 = true;
        }
        seen2.insert(cur);
    }
    if ans1 {
        out.print_line(1);
    } else {
        out.print_line(2);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
