//{"name":"A - Refilling Water","group":"AtCoder - AtCoder Weekday Contest 0044 Beta","url":"https://atcoder.jp/contests/awc0044/tasks/awc0044_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 8\n10 3\n8 5\n6 2\n","output":"10\n6\n2\n"},{"input":"5 100\n10 5\n20 20\n15 3\n8 0\n12 7\n","output":"10\n20\n15\n8\n12\n"},{"input":"8 1000000000000\n1000000000 0\n500000000 200000000\n1000000000 1000000000\n300000000 100000000\n700000000 0\n400000000 400000000\n600000000 150000000\n200000000 50000000\n","output":"1000000000\n500000000\n1000000000\n300000000\n700000000\n400000000\n600000000\n200000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut m = input.read_long();
    let cs = input.read_long_pair_vec(n);

    for (c, s) in cs {
        let cur = (c - s).min(m);
        m -= cur;
        out.print_line(s + cur);
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
