//{"name":"C - Catch the Last Train!","group":"AtCoder - AtCoder Weekday Contest 0014 Beta","url":"https://atcoder.jp/contests/awc0014/tasks/awc0014_c","interactive":false,"timeLimit":2000,"tests":[{"input":"10 30 3 2 1\n","output":"Yes\n"},{"input":"100 120 2 3 10\n","output":"No\n"},{"input":"1000000000 1000000000000000 1000000000 1 1\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::rational::Rational;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let g = input.read_i128();
    let m = input.read_i128();
    let d = input.read_i128();
    let k = input.read_i128();
    let v = input.read_i128();

    let can = if g >= d * k {
        Rational::new_int(k + g - d * k)
    } else {
        Rational::new(g, d)
    };
    out.print_line(can <= Rational::new(m - g, v));
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
