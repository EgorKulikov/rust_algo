//{"name":"T736385 弹珠方阵","group":"Luogu","url":"https://www.luogu.com.cn/problem/T736385?contestId=311027","interactive":false,"timeLimit":1000,"tests":[{"input":"3 4 3\n1 2 3 1\n2 3 1 4\n3 4 2 1\n","output":"6\n"},{"input":"5 5 10\n1 2 3 4 5\n4 3 6 4 2\n1 4 3 5 2\n6 1 3 2 3\n1 7 2 3 4\n","output":"Hrk\n"},{"input":"5 5 6\n1 2 3 4 5\n4 3 6 4 2\n1 4 3 5 2\n6 1 3 2 3\n1 7 2 3 4\n","output":"20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let d = input.read_size();
    let a = input.read_size_table(n, m);

    let mut ans = None;
    for i in 0..n {
        for j in 0..m {
            for k in j..m {
                let mut set = FxHashSet::default();
                for l in i..n {
                    for o in j..=k {
                        set.insert(a[(l, o)]);
                    }
                    if set.len() == d {
                        ans.maxim((k - j + 1) * (l - i + 1));
                    }
                }
            }
        }
    }
    if let Some(ans) = ans {
        out.print_line(ans);
    } else {
        out.print_line("Hrk");
    }
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
