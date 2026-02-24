//{"name":"C - Team Formation","group":"AtCoder - AtCoder Weekday Contest 0012 Beta","url":"https://atcoder.jp/contests/awc0012/tasks/awc0012_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3 1\n1 100\n0 80\n0 90\n1 50\n0 70\n","output":"270\n"},{"input":"6 4 2\n1 500\n0 300\n1 400\n0 600\n1 200\n0 350\n","output":"1850\n"},{"input":"10 5 3\n1 1000000000\n0 999999999\n1 800000000\n0 750000000\n1 600000000\n0 500000000\n1 400000000\n0 300000000\n0 200000000\n0 100000000\n","output":"4149999999\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let m = input.read_size();
    let hp = input.read_long_pair_vec(n);

    let mut exp = Vec::new();
    let mut non_exp = Vec::new();
    for (h, p) in hp {
        if h == 1 {
            exp.push(p);
        } else {
            non_exp.push(p);
        }
    }
    exp.sort_unstable_by_key(|&x| -x);
    non_exp.sort_unstable_by_key(|&x| -x);
    if exp.len() < m || non_exp.len() < k - m {
        out.print_line(-1);
        return;
    }
    out.print_line(exp.copy_take(m).sum::<i64>() + non_exp.copy_take(k - m).sum::<i64>());
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
