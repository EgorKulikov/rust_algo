//{"name":"A - Organizing the Bookshelf","group":"AtCoder - AtCoder Weekday Contest 0017 Beta","url":"https://atcoder.jp/contests/awc0017/tasks/awc0017_a","interactive":false,"timeLimit":2000,"tests":[{"input":"5 2\n1 10\n3 20\n0 15\n2 30\n5 25\n","output":"55\n"},{"input":"4 1\n5 100\n3 200\n10 50\n2 300\n","output":"0\n"},{"input":"10 5\n3 120\n7 250\n5 80\n0 300\n12 60\n4 150\n6 90\n1 200\n9 110\n5 170\n","output":"1020\n"},{"input":"20 100\n50 1000000000\n101 500000000\n99 800000000\n200 300000000\n0 999999999\n100 750000000\n150 600000000\n30 450000000\n75 200000000\n110 350000000\n1 900000000\n88 100000000\n100 550000000\n250 400000000\n99 650000000\n1000000000 1000000000\n45 700000000\n102 850000000\n100 500000000\n60 300000000\n","output":"7899999999\n"},{"input":"1 0\n0 1\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_long();
    let cd = input.read_long_pair_vec(n);

    let mut ans = 0;
    for (c, d) in cd {
        if c <= k {
            ans += d;
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
