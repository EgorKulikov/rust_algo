//{"name":"Lexicography","group":"Eolymp - Basecamp - Educational Round #6","url":"https://eolymp.com/en/compete/ikoi44ho994uj2rcj49h0l45v4/problem/6","interactive":false,"timeLimit":1000,"tests":[{"input":"3 2 2\nabcdef\n","output":"ad\nbc\nef\n"},{"input":"2 3 1\nabcabc\n","output":"aab\nbcc\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let l = input.read_size();
    let k = input.read_size() - 1;
    let s = input.read_vec::<u8>(n * l).sorted();

    let mut ans = Arr2d::new(n, l, 0);
    let mut start = 0;
    let mut at = 0;
    for i in 0..l {
        for j in start..=k {
            ans[(j, i)] = s[at];
            if j != start && s[at] != s[at - 1] {
                start = j;
            }
            at += 1;
        }
    }
    for i in 0..n {
        for j in 0..l {
            if ans[(i, j)] == 0 {
                ans[(i, j)] = s[at];
                at += 1;
            }
        }
    }
    out.print_table(&ans);
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
