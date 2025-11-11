//{"name":"Interpreter","group":"CSES - Datatähti 2026 alku","url":"https://cses.fi/637/task/C","interactive":false,"timeLimit":1000,"tests":[{"input":"PRINT X\nINCREASE X\nPRINT X\nINCREASE X\nPRINT X\nCLEAR X\nPRINT X\n","output":"0 1 2 0\n"},{"input":"INCREASE A\nINCREASE A\nINCREASE A\nINCREASE A\nINCREASE A\n\nREPEAT A TIMES (\n    INCREASE A\n    PRINT A\n)\n","output":"6 7 8 9 10\n"},{"input":"# Create number 3\nINCREASE A INCREASE A INCREASE A\n\n# Create number 4\nINCREASE B INCREASE B INCREASE B INCREASE B\n\n# Calculate 3 * 4\nREPEAT A TIMES ( REPEAT B TIMES ( INCREASE C ) )\n\n# Print result\nPRINT C\n","output":"12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    enum Command {
        Clear(usize),
        Increase(usize),
        Print(usize),
        Repeat(usize),
        End,
    }
    let mut program = Vec::new();
    let mut tokens = Vec::new();
    while !input.is_empty() {
        let token = input.read_str();
        if token.contains(&b'#') {
            let pos = token.iter().position(|&c| c == b'#').unwrap();
            if pos > 0 {
                tokens.push(Str::from(&token[..pos]));
            }
            if !input.is_eol() {
                input.read_line();
            }
            continue;
        }
        tokens.push(token);
    }
    let mut at = 0;
    while at < tokens.len() {
        match tokens[at].as_slice() {
            b"PRINT" => {
                assert_eq!(tokens[at + 1].len(), 1);
                let var = tokens[at + 1][0] - b'A';
                program.push(Command::Print(var as usize));
                at += 2;
            }
            b"INCREASE" => {
                assert_eq!(tokens[at + 1].len(), 1);
                let var = tokens[at + 1][0] - b'A';
                program.push(Command::Increase(var as usize));
                at += 2;
            }
            b"CLEAR" => {
                assert_eq!(tokens[at + 1].len(), 1);
                let var = tokens[at + 1][0] - b'A';
                program.push(Command::Clear(var as usize));
                at += 2;
            }
            b"REPEAT" => {
                assert_eq!(tokens[at + 1].len(), 1);
                let var = tokens[at + 1][0] - b'A';
                assert_eq!(tokens[at + 2].as_slice(), b"TIMES");
                assert_eq!(tokens[at + 3].as_slice(), b"(");
                program.push(Command::Repeat(var as usize));
                at += 4;
            }
            b")" => {
                program.push(Command::End);
                at += 1;
            }
            _ => unreachable!(),
        }
    }
    program.push(Command::End);

    let mut ans = Vec::new();
    let mut variables = [0u32; 26];
    let mut run = RecursiveFunction::new(|rec, mut at: usize| loop {
        match &program[at] {
            Command::Clear(x) => {
                variables[*x] = 0;
            }
            Command::Increase(x) => {
                variables[*x] += 1;
            }
            Command::Print(x) => {
                ans.push(variables[*x]);
            }
            Command::Repeat(x) => {
                for _ in 0..variables[*x] {
                    rec.call(at + 1);
                }
                let mut balance = 0;
                loop {
                    match &program[at] {
                        Command::Repeat(_) => balance += 1,
                        Command::End => {
                            balance -= 1;
                            if balance == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    at += 1;
                }
            }
            Command::End => {
                return;
            }
        }
        at += 1;
    });
    run.call(0);
    out.print_line(ans);
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
