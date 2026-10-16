use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_int_table(n, n);

    let mut seen = Arr2d::new(n, n, false);
    let mut ans = vec![0; k + 1];
    for (i, j) in a.indices() {
        if seen[(i, j)] {
            continue;
        }
        let mut state = 0;
        let mut r = i;
        let mut c = j;
        let mut at = 0;
        while !seen[(r, c)] {
            seen[(r, c)] = true;
            if a[(r, c)] == 1 {
                state.set_bit(at);
            }
            at += 1;
            (r, c) = (c, n - r - 1);
        }
        let mut max = vec![0; at + 1];
        for mask in usize::iter_all(at) {
            if !state.is_subset(mask) {
                continue;
            }
            let mut q = 0;
            for i in 0..at {
                if mask.is_set(i) && mask.is_set((i + 1) % at) {
                    q += 1;
                }
            }
            max[mask.count_ones() as usize].maxim(q);
        }
        let base = state.count_ones() as usize;
        for i in (0..=k).rev() {
            for j in (0..=(k - i).min(at - base)).rev() {
                let cand = ans[i] + max[base + j];
                ans[i + j].maxim(cand);
            }
        }
    }
    out.print_line(ans[k]);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
