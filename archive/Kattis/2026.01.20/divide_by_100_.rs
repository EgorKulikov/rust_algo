//{"name":"Divide by 100...","group":"Kattis","url":"https://open.kattis.com/problems/divideby100","interactive":false,"timeLimit":1000,"tests":[{"input":"92746237\n100000\n","output":"927.46237\n"},{"input":"100000\n100\n","output":"1000\n"},{"input":"1234500\n10000\n","output":"123.45\n"},{"input":"1\n10\n","output":"0.1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut n = input.read_str();
    let mut m = input.read_str();

    while n[Back(0)] == b'0' && m[Back(0)] == b'0' {
        n.pop();
        m.pop();
    }
    if m.len() == 1 {
        out.print_line(n);
        return;
    }
    if n.len() >= m.len() {
        out.print(Str::from(&n[..=n.len() - m.len()]));
        out.print('.');
        out.print_line(Str::from(&n[n.len() - m.len() + 1..]));
    } else {
        out.print("0.");
        for _ in 0..m.len() - n.len() - 1 {
            out.print('0');
        }
        out.print_line(n);
    }
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
