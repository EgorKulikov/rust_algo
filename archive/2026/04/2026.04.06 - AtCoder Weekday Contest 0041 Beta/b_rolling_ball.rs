//{"name":"B - Rolling Ball","group":"AtCoder - AtCoder Weekday Contest 0041 Beta","url":"https://atcoder.jp/contests/awc0041/tasks/awc0041_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3 5\n2 5 3 1\n4 1 10\n","output":"2\n1\n5\n"},{"input":"5 6\n3 3 3 3 3\n1 2 3 4 5\n","output":"1\n1\n6\n6\n6\n"},{"input":"7 10\n5 12 8 20 3 15 6 10 25\n10 20 5 30 1 8 100\n","output":"2\n9\n2\n10\n1\n2\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut w = input.read_int_vec(m - 1);
    let b = input.read_int_vec(n);

    for i in 0..m - 2 {
        let cand = w[i];
        w[i + 1].maxim(cand);
    }
    for i in b {
        out.print_line(w.upper_bound(&i) + 1);
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
