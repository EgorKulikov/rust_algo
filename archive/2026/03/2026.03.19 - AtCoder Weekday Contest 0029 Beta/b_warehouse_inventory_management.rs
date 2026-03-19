//{"name":"B - Warehouse Inventory Management","group":"AtCoder - AtCoder Weekday Contest 0029 Beta","url":"https://atcoder.jp/contests/awc0029/tasks/awc0029_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3 5\n10 20 30\n1 1 2\n2 2\n2 1\n1 3 2\n2 2\n","output":"30\n0\n60\n"},{"input":"4 6\n5 15 25 35\n2 4\n1 1 3\n1 2 3\n2 3\n1 1 4\n2 1\n","output":"35\n45\n0\n"},{"input":"8 12\n100 200 300 400 500 600 700 800\n1 1 2\n1 3 2\n2 2\n1 4 5\n1 6 5\n2 5\n1 2 5\n2 5\n1 7 8\n1 8 5\n2 5\n2 7\n","output":"600\n1500\n2100\n3600\n0\n"},{"input":"10 15\n1000000000 999999999 888888888 777777777 666666666 555555555 444444444 333333333 222222222 111111111\n1 1 2\n1 3 2\n1 4 2\n2 2\n1 5 6\n1 7 6\n1 8 6\n2 6\n1 2 10\n1 6 10\n2 10\n1 9 10\n2 10\n2 1\n2 5\n","output":"3666666664\n1999999998\n5777777773\n5999999995\n0\n0\n"},{"input":"1 1\n1000000000\n2 1\n","output":"1000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut v = input.read_long_vec(n);

    for _ in 0..q {
        let t = input.read_int();
        match t {
            1 => {
                let i = input.read_size() - 1;
                let j = input.read_size() - 1;
                v[j] += v[i];
                v[i] = 0;
            }
            2 => {
                let i = input.read_size() - 1;
                out.print_line(v[i]);
            }
            _ => unreachable!(),
        }
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
