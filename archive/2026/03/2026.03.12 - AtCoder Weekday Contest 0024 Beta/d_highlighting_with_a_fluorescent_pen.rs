//{"name":"D - Highlighting with a Fluorescent Pen","group":"AtCoder - AtCoder Weekday Contest 0024 Beta","url":"https://atcoder.jp/contests/awc0024/tasks/awc0024_d","interactive":false,"timeLimit":2000,"tests":[{"input":"10 3 2\n2\n5\n","output":"0 1 1 1 1 1 1 0 0 0\n"},{"input":"8 4 3\n1\n3\n2\n","output":"1 2 3 3 2 1 0 0\n"},{"input":"15 5 6\n1\n3\n7\n11\n2\n7\n","output":"1 2 3 3 3 2 3 2 2 2 3 1 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let w = input.read_size();
    let k = input.read_size();
    let l = input.read_size_vec(k).dec();

    let mut delta = vec![0; n + 1];
    for i in l {
        delta[i] += 1;
        delta[i + w] -= 1;
    }
    let mut cur = 0;
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        cur += delta[i];
        ans.push(cur);
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
