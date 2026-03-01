//{"name":"D - Assignment of Cooking Tasks","group":"AtCoder - AtCoder Weekday Contest 0015 Beta","url":"https://atcoder.jp/contests/awc0015/tasks/awc0015_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3 500\n5 3 1\n2 4 6\n","output":"1000\n"},{"input":"5 4 1000\n10 3 7 1 8\n5 2 9 4\n","output":"4000\n"},{"input":"7 8 1000000000\n100 50 80 30 60 90 10\n20 40 60 80 100 55 35 75\n","output":"6000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let c = input.read_long();
    let a = input.read_int_vec(n).sorted();
    let b = input.read_int_vec(m).sorted();

    let events = Vec::with_gen(n + m, |i| if i < n { (a[i], 1) } else { (b[i - n], 0) }).sorted();
    let mut has = 0;
    let mut ans = 0;
    for (_, t) in events {
        if t == 0 {
            has += 1;
        } else if has > 0 {
            has -= 1;
            ans += c;
        }
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
