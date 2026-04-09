//{"name":"B - Library Book Survey on the Bookshelf","group":"AtCoder - AtCoder Weekday Contest 0044 Beta","url":"https://atcoder.jp/contests/awc0044/tasks/awc0044_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n3 1 4 1 5\n1 3\n2 4\n1 5\n","output":"8\n6\n14\n"},{"input":"8 5\n10 20 30 40 50 60 70 80\n1 1\n3 6\n1 8\n4 7\n2 5\n","output":"10\n180\n360\n220\n140\n"},{"input":"10 8\n1000000000 999999999 123456789 987654321 500000000 250000000 750000000 100000000 999999999 1\n1 10\n1 1\n10 10\n3 7\n5 5\n1 5\n6 10\n2 9\n","output":"5711111109\n1000000000\n1\n2611111110\n500000000\n3611111109\n2100000000\n4711111108\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n);

    let s = a.partial_sums();
    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();
        out.print_line(s[r] - s[l]);
    }
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
