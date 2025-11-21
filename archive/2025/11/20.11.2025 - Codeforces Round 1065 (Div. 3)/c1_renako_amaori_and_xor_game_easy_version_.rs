//{"name":"C1. Renako Amaori and XOR Game (easy version)","group":"Codeforces - Codeforces Round 1065 (Div. 3)","url":"https://codeforces.com/contest/2171/problem/C1","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n4\n1 0 0 1\n1 0 1 1\n6\n0 1 1 1 1 0\n0 0 1 0 1 1\n4\n0 0 1 0\n1 0 1 1\n5\n1 0 1 1 1\n0 1 1 1 0\n6\n1 1 1 1 1 1\n1 1 1 1 1 1\n5\n0 1 0 0 1\n1 0 0 1 1\n","output":"Ajisai\nMai\nTie\nAjisai\nTie\nMai\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::ops::BitXor;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);

    let xa = a.copy_fold(0, i32::bitxor);
    let xb = b.copy_fold(0, i32::bitxor);
    if xa == xb {
        out.print_line("Tie");
        return;
    }
    for i in (0..n).rev() {
        if a[i] != b[i] {
            if i % 2 == 0 {
                out.print_line("Ajisai");
            } else {
                out.print_line("Mai");
            }
            return;
        }
    }
    unreachable!();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
    eprint!("\x1B[0m");
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive | TaskType::RunTwice => true,
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
