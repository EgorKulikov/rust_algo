//{"name":"D - Meeting Room Reservation","group":"AtCoder - AtCoder Weekday Contest 0034 Beta","url":"https://atcoder.jp/contests/awc0034/tasks/awc0034_d","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 5\n3 8\n1 3\n5 8\n2 6\n","output":"2\n"},{"input":"8\n1 4\n3 6\n2 4\n5 9\n5 7\n7 9\n9 10\n1 2\n","output":"5\n"},{"input":"12\n0 100\n50 150\n100 200\n200 500\n150 300\n300 600\n500 700\n600 800\n700 800\n750 900\n1000 1000000000\n999999999 1000000000\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let lr = input.read_size_pair_vec(n).sorted_by_key(|x| x.1);

    let mut at = 0;
    let mut ans = 0;
    for (l, r) in lr {
        if l >= at {
            ans += 1;
            at = r;
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
