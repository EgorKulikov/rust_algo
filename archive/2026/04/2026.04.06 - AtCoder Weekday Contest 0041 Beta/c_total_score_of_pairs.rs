//{"name":"C - Total Score of Pairs","group":"AtCoder - AtCoder Weekday Contest 0041 Beta","url":"https://atcoder.jp/contests/awc0041/tasks/awc0041_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4 6\n1 3 5 7\n","output":"5\n"},{"input":"3 10\n1 2 3\n","output":"0\n"},{"input":"10 15\n3 7 1 9 5 8 2 6 10 4\n","output":"9\n"},{"input":"15 100\n50 50 50 50 50 50 50 50 50 50 50 50 50 50 50\n","output":"105\n"},{"input":"2 2000000000\n1000000000 1000000000\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_int();
    let a = input.read_int_vec(n).sorted();

    let mut at = n;
    let mut ans = 0;
    for i in 0..n {
        at.maxim(i + 1);
        while at - 1 > i && a[i] + a[at - 1] >= k {
            at -= 1;
        }
        ans += n - at;
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
