//{"name":"L. Letters on T-shirts","group":"Universal Cup - CERC 2025","url":"https://contest.ucup.ac/contest/2814/problem/16011","interactive":false,"timeLimit":1000,"tests":[{"input":"abc\nrc\nce\n","output":"YES\n2\n3 2\n"},{"input":"cer\ncr\nicpc\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::next_permutation::NextPermutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_str_vec(3);

    for i in 0..3 {
        if s[i].as_slice() == b"cerc" {
            out.print_line(true);
            out.print_line(1);
            out.print_line(i + 1);
            return;
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            if i != j {
                let mut cand = s[i].clone();
                cand += &s[j];
                if cand.as_slice() == b"cerc" {
                    out.print_line(true);
                    out.print_line(2);
                    out.print_line((i + 1, j + 1));
                    return;
                }
            }
        }
    }
    let mut order = (0..3).collect::<Vec<_>>();
    loop {
        let mut cand = Str::new();
        for i in order.copy_iter() {
            cand += &s[i];
        }
        if cand.as_slice() == b"cerc" {
            out.print_line(true);
            out.print_line(3);
            out.print_line(order.inc());
            return;
        }
        if !order.next_permutation() {
            break;
        }
    }
    out.print_line(false);
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
