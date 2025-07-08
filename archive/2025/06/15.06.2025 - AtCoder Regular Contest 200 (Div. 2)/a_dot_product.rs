//{"name":"A - Dot Product","group":"AtCoder - AtCoder Regular Contest 200 (Div. 2)","url":"https://atcoder.jp/contests/arc200/tasks/arc200_a","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n3\n3 1 4\n1 5 1\n3\n4 4 4\n7 7 7\n4\n20 25 6 15\n31 41 59 26\n","output":"Yes\n4 -5 1\nNo\nYes\n45 -10 -40 11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::rational::Rational;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);

    let first = Rational::new(a[0], b[0]);
    for i in 1..n {
        let cur = Rational::new(a[i], b[i]);
        if cur != first {
            let mut x = vec![0; n];
            if a[0] * b[i] > a[i] * b[0] {
                x[0] = a[i] + b[i];
                x[i] = -(a[0] + b[0]);
            } else {
                x[0] = -(a[i] + b[i]);
                x[i] = a[0] + b[0];
            }
            out.print_line(true);
            out.print_line(x);
            return;
        }
    }
    out.print_line(false);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
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
