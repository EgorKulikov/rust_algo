//{"name":"A - Counting the Number of Successful Applicants","group":"AtCoder - AtCoder Weekday Contest 0021 Beta","url":"https://atcoder.jp/contests/awc0021/tasks/awc0021_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 60\n4 45 72 60 88\n3 59 60 61\n2 100 30\n","output":"6\n"},{"input":"5 70\n5 65 70 80 55 90\n3 69 71 70\n4 100 85 72 68\n2 50 49\n6 75 80 85 90 95 100\n","output":"14\n"},{"input":"10 50\n8 23 45 67 89 12 34 56 78\n5 50 50 50 49 51\n10 0 10 20 30 40 50 60 70 80 90\n3 100 100 100\n7 45 46 47 48 49 50 51\n4 25 75 25 75\n6 99 98 97 96 95 94\n2 0 100\n9 55 55 55 55 55 44 44 44 44\n5 1 2 3 4 5\n","output":"32\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let b = input.iter::<Vec<usize>>().take(n).collect::<Vec<_>>();

    let mut ans = 0;
    for b in b {
        for b in b {
            if b >= k {
                ans += 1;
            }
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
