//{"name":"Working at the Restaurant","group":"Kattis","url":"https://open.kattis.com/problems/restaurant","interactive":false,"timeLimit":1000,"tests":[{"input":"3\nDROP 100\nTAKE 50\nTAKE 20\n3\nDROP 3\nDROP 5\nTAKE 8\n0\n","output":"DROP 2 100\nMOVE 2->1 100\nTAKE 1 50\nTAKE 1 20\n\nDROP 2 3\nDROP 2 5\nMOVE 2->1 8\nTAKE 1 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::io::Write;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    if n == 0 {
        return;
    }

    if test_case != 1 {
        out.print_line(());
    }
    let mut q2 = 0;
    let mut q1 = 0;
    for _ in 0..n {
        let command = input.read_str();
        match command.as_slice() {
            b"DROP" => {
                let x = input.read_size();
                writeln!(out, "DROP 2 {}", x).unwrap();
                q2 += x;
            }
            b"TAKE" => {
                let x = input.read_size();
                if q1 >= x {
                    writeln!(out, "TAKE 1 {}", x).unwrap();
                    q1 -= x;
                } else {
                    if q1 != 0 {
                        writeln!(out, "TAKE 1 {}", q1).unwrap();
                    }
                    let need = x - q1;
                    writeln!(out, "MOVE 2->1 {}", q2).unwrap();
                    q1 = q2;
                    q2 = 0;
                    writeln!(out, "TAKE 1 {}", need).unwrap();
                    q1 -= need;
                }
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
