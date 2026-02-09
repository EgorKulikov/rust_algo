//{"name":"task_c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::qty;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;
use std::ops::BitXor;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_unsigned_vec(n);

    let target = a.copy_fold(0, u32::bitxor);
    if target == 0 {
        out.print_line(false);
        return;
    }
    let mut set = BTreeSet::new();
    let mut qty = qty(&a);
    for (&k, &v) in qty.iter() {
        set.insert((v, k));
    }
    if n > 1 && set.len() == 1 {
        out.print_line(false);
        return;
    }
    out.print_line(true);
    for _ in 0..n - 1 {
        let (v1, k1) = set.pop_last().unwrap();
        let (v2, k2) = set.pop_last().unwrap();
        qty[k1] -= 1;
        qty[k2] -= 1;
        set.remove(&(qty[k1 ^ k2], k1 ^ k2));
        qty[k1 ^ k2] += 1;
        if v1 > 1 {
            set.insert((v1 - 1, k1));
        }
        if v2 > 1 {
            set.insert((v2 - 1, k2));
        }
        set.insert((qty[k1 ^ k2], k1 ^ k2));
        out.print_line((k1, k2));
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
