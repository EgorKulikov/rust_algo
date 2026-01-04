//{"name":"coderun8","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let t = input.read_size();
    let klr = input.read_vec::<(usize, usize, usize)>(t);

    let mut ans = vec![0; t];
    let mut by_k = vec![Vec::new(); 101];
    for (i, (k, l, r)) in klr.iter_enumerate() {
        by_k[k].push((l, r, i));
    }

    let mut cur = vec![None; 700_001];
    cur[1] = Some(1);
    let mut next = vec![None; 700_001];
    let mut qty = vec![0; 700_001];
    for k in 1..=100 {
        next.fill(None);
        for i in 1..=700_000 {
            if next[i].is_some() {
                qty[i] = qty[i - 1] + 1;
            } else {
                qty[i] = qty[i - 1];
            }
            if let Some(v) = cur[i] {
                for p in v + 1..=700_000 / i {
                    next[i * p].minim(p);
                }
            }
        }
        for (l, r, id) in by_k[k].copy_iter() {
            ans[id] = qty[r] - qty[l - 1];
        }
        swap(&mut cur, &mut next);
    }
    out.print_per_line(&ans);
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
