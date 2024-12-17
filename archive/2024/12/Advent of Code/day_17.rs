//{"name":"day_17","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day_17"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::scan;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut registers = Vec::new();
    for c in 'A'..='C' {
        scan!(input, format!("Register {}: @", c).as_str(), val: usize);
        registers.push(val);
    }
    input.read_line();
    scan!(input, "Program: @", program: Str);
    let program = program
        .split(',')
        .into_iter()
        .map(|s| s.parse::<usize>())
        .collect::<Vec<_>>();

    // part 1
    {
        let mut ip = 0;
        let mut ans = Vec::new();
        loop {
            if ip + 2 > program.len() {
                break;
            }
            let mut combo = match program[ip + 1] {
                0..=3 => program[ip + 1],
                4..=6 => registers[program[ip + 1] - 4],
                7 => 0,
                _ => unreachable!(),
            };
            match program[ip] {
                0 => {
                    while registers[0] != 0 && combo != 0 {
                        registers[0] /= 2;
                        combo -= 1;
                    }
                }
                1 => {
                    registers[1] ^= program[ip + 1];
                }
                2 => {
                    registers[1] = combo & 7;
                }
                3 => {
                    if registers[0] != 0 {
                        ip = program[ip + 1];
                        continue;
                    }
                }
                4 => {
                    registers[1] ^= registers[2];
                }
                5 => {
                    ans.push(combo & 7);
                }
                6 => {
                    registers[1] = registers[0];
                    while registers[1] != 0 && combo != 0 {
                        registers[1] /= 2;
                        combo -= 1;
                    }
                }
                7 => {
                    registers[2] = registers[0];
                    while registers[2] != 0 && combo != 0 {
                        registers[2] /= 2;
                        combo -= 1;
                    }
                }
                _ => unreachable!(),
            }
            ip += 2;
        }
        let ans = format!("{:?}", ans)
            .replace(" ", "")
            .replace("[", "")
            .replace("]", "");
        out.print_line(ans);
    }

    // part 2
    {
        let mut ans = 0;
        let mut rec =
            RecursiveFunction2::new(|rec, mut ans: usize, step: usize| -> Option<usize> {
                if step == program.len() {
                    return Some(ans);
                }
                ans *= 8;
                let v = program[Back(step)];
                for a_add in 0..8 {
                    let a = ans + a_add;
                    let mut b = a & 7;
                    b ^= 5;
                    let c = a >> b;
                    b ^= c;
                    b ^= 6;
                    if b & 7 == v {
                        let call = rec.call(a, step + 1);
                        if call.is_some() {
                            return call;
                        }
                    }
                }
                None
            });
        out.print_line(rec.call(0, 0));
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}

#[cfg(test)]
mod tester;
//END MAIN
