//{"name":"T727432 [语言月赛 202601] 数字游戏 II","group":"Luogu","url":"https://www.luogu.com.cn/problem/T727432?contestId=304401","interactive":false,"timeLimit":1000,"tests":[{"input":"3 0 4 1\n4 1 2 0\n1 0 3 2\n2 3 1 0\n","output":"3 2 4 1\n4 1 2 3\n1 4 3 2\n2 3 1 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let t = input.read_int_table(4, 4);

    let mut rec = RecursiveFunction3::new(
        |rec, t: Arr2d<i32>, r: usize, c: usize| -> Option<Arr2d<i32>> {
            if r == 4 {
                Some(t)
            } else if c == 4 {
                rec.call(t, r + 1, 0)
            } else if t[(r, c)] != 0 {
                rec.call(t, r, c + 1)
            } else {
                for v in 1..=4 {
                    let mut good = true;
                    for i in 0..4 {
                        if t[(r, i)] == v || t[(i, c)] == v {
                            good = false;
                        }
                    }
                    for i in r - r % 2..r - r % 2 + 2 {
                        for j in c - c % 2..c - c % 2 + 2 {
                            if t[(i, j)] == v {
                                good = false;
                            }
                        }
                    }
                    if !good {
                        continue;
                    }
                    let mut t = t.clone();
                    t[(r, c)] = v;
                    if let Some(res) = rec.call(t, r, c + 1) {
                        return Some(res);
                    }
                }
                None
            }
        },
    );
    out.print_line(rec.call(t, 0, 0));
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
