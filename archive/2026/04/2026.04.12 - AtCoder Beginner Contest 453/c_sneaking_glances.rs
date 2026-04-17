//{"name":"C - Sneaking Glances","group":"AtCoder - AtCoder Beginner Contest 453","url":"https://atcoder.jp/contests/abc453/tasks/abc453_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2 5 2 2 1\n","output":"4\n"},{"input":"5\n100 1 2 3 4\n","output":"1\n"},{"input":"20\n1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1\n","output":"20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let l = input.read_long_vec(n);

    let mut ans = 0;
    for i in usize::iter_all(n) {
        let mut cur = 1;
        let mut times = 0;
        for j in 0..n {
            if i.is_set(j) {
                cur += 2 * l[j];
                if cur > 0 && cur - 2 * l[j] < 0 {
                    times += 1;
                }
            } else {
                cur -= 2 * l[j];
                if cur < 0 && cur + 2 * l[j] > 0 {
                    times += 1;
                }
            }
        }
        ans.maxim(times);
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
