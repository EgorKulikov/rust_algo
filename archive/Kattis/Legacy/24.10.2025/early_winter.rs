//{"name":"Early Winter","group":"Kattis","url":"https://open.kattis.com/problems/earlywinter","interactive":false,"timeLimit":1000,"tests":[{"input":"4 2\n3 3 3 2\n","output":"It hadn't snowed this early in 3 years!\n"},{"input":"2 10\n0 100\n","output":"It hadn't snowed this early in 0 years!\n"},{"input":"3 1\n42 43 44\n","output":"It had never snowed this early!\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let d_n = input.read_int();
    let d = input.read_int_vec(n);

    let ans = d.into_iter().take_while(|&x| x > d_n).count();
    if ans == n {
        out.print_line("It had never snowed this early!");
    } else {
        out.print_line(format!("It hadn't snowed this early in {} years!", ans));
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

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
