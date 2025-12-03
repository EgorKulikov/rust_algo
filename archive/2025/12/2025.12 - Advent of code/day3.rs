//{"name":"day3","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_lines();

    let mut ans = 0;
    let mut ans2 = 0;
    for s in s {
        let mut d2 = Vec::new();
        let mut d12 = Vec::new();
        for c in s.iter_rev() {
            let d = (c - b'0') as i64;
            if d2.len() < 2 {
                d2.push(d);
            } else {
                let mut cur = d;
                for i in d2.indices().rev() {
                    if cur >= d2[i] {
                        swap(&mut cur, &mut d2[i]);
                    } else {
                        break;
                    }
                }
            }
            if d12.len() < 12 {
                d12.push(d);
            } else {
                let mut cur = d;
                for i in d12.indices().rev() {
                    if cur >= d12[i] {
                        swap(&mut cur, &mut d12[i]);
                    } else {
                        break;
                    }
                }
            }
        }
        let mut cur2 = 0;
        for i in d2.iter_rev() {
            cur2 *= 10;
            cur2 += i;
        }
        ans += cur2;
        let mut cur12 = 0;
        for i in d12.iter_rev() {
            cur12 *= 10;
            cur12 += i;
        }
        ans2 += cur12;
    }
    out.print_line(ans);
    out.print_line(ans2);
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
