//{"name":"witch_s_cauldron_part_1","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::eol::EolStr;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use algo_lib::{scan, str_scan};
use std::io::Write;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let has = (0..n)
        .map(|_| {
            scan!(input, "@(@)\n", name: EolStr, qty: i32);
            (name.unwrap(), qty)
        })
        .collect::<std::collections::HashMap<_, _>>();
    let ans = (0..m)
        .map(|_| {
            input.read_char();
            let s = input.read_line();
            let mut good = true;
            for token in s.trim_ascii().split(|x| *x == b',') {
                let token = token.trim_ascii();
                str_scan!(token, "@(@)", name: EolStr, qty: i32);
                let have_qty = has.get(&name.unwrap()).copied().unwrap_or(0);
                if have_qty < qty {
                    good = false;
                    break;
                }
            }
            good
        })
        .filter(|&x| x)
        .count();
    writeln!(out, "Case #{}: {}", test_case, ans).unwrap();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
