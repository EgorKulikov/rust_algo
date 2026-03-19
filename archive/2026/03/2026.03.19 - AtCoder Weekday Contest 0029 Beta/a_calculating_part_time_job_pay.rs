//{"name":"A - Calculating Part-Time Job Pay","group":"AtCoder - AtCoder Weekday Contest 0029 Beta","url":"https://atcoder.jp/contests/awc0029/tasks/awc0029_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 100 50 3\n5 2 3\n","output":"1400\n"},{"input":"3 200 100 10\n3 5 2\n","output":"2000\n"},{"input":"10 500 300 15\n20 10 15 8 25 14 16 5 30 12\n","output":"109300\n"},{"input":"20 1000 500 20\n25 18 31 20 10 15 22 28 19 30 5 12 21 27 16 8 24 31 14 20\n","output":"535500\n"},{"input":"1 1 1 1\n1\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_int();
    let b = input.read_int();
    let k = input.read_int();
    let c = input.read_int_vec(n);

    let mut ans = 0;
    for i in c {
        if i >= k {
            ans += (p + b) * i;
        } else {
            ans += p * i;
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
