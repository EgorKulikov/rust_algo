//{"name":"Arithmetic","group":"Kattis","url":"https://open.kattis.com/problems/arithmetic","interactive":false,"timeLimit":1000,"tests":[{"input":"4444\n","output":"924\n"},{"input":"20\n","output":"10\n"},{"input":"3211\n","output":"689\n"},{"input":"7654321001234567\n","output":"FAC688053977\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str();

    if s.as_slice() == b"0" {
        out.print_line(0);
        return;
    }
    let mut pos = s.len() % 4;
    if pos == 0 {
        pos += 4;
    }
    let mut process = |from: usize, to: usize, leading: bool| {
        let mut val = 0;
        for i in from..to {
            val = val * 8 + (s[i] - b'0') as usize;
        }
        let d = vec![val / 256, (val / 16) % 16, val % 16];
        let mut print = leading;
        for d in d {
            if d != 0 {
                print = true;
            }
            if print {
                out.print(if d < 10 {
                    b'0' + d as u8
                } else {
                    b'A' + (d as u8 - 10)
                });
            }
        }
    };
    process(0, pos, false);
    for i in (pos + 4..=s.len()).step_by(4) {
        process(i - 4, i, true);
    }
    out.print_line(());
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
