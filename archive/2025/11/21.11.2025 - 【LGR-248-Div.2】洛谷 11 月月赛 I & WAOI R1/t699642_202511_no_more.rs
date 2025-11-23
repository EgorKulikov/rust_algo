//{"name":"T699642 [语言月赛 202511] 曼波 No More","group":"Luogu","url":"https://www.luogu.com.cn/problem/T699642?contestId=291890","interactive":false,"timeLimit":1000,"tests":[{"input":"5 5 1 1\n0 0 2 2\n","output":"9\n"},{"input":"4 8 1 2\n1 2 3 7\n","output":"9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let _m = input.read_size();
    let r = input.read_size();
    let c = input.read_size();
    let x1 = input.read_size();
    let y1 = input.read_size();
    let x2 = input.read_size();
    let y2 = input.read_size();

    let x1 = x1 / r;
    let x2 = x2 / r;
    let y1 = y1 / c;
    let y2 = y2 / c;
    out.print_line((x2 + 1 - x1) * (y2 + 1 - y1));
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
