//{"name":"C2. Renako Amaori and XOR Game (hard version)","group":"Codeforces - Codeforces Round 1065 (Div. 3)","url":"https://codeforces.com/contest/2171/problem/C2","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n4\n1 4 6 1\n3 2 3 7\n6\n20 11 1 7 7 0\n14 8 3 6 17 6\n4\n2 6 3 6\n3 4 7 1\n5\n1 4 5 5 3\n6 7 1 2 13\n6\n9 5 9 17 17 6\n1 13 6 13 1 15\n5\n2 3 8 1 5\n3 1 6 14 7\n","output":"Mai\nAjisai\nTie\nAjisai\nMai\nTie\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
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
    let xor = xa ^ xb;
    let special_bit = xor.highest_bit();
    for i in (0..n).rev() {
        if a[i].is_set(special_bit) != b[i].is_set(special_bit) {
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
