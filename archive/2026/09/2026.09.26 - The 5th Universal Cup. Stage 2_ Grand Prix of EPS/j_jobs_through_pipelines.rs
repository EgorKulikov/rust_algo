use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_utils::{PartialSums, UpperDiv};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n);

    let mut b = a.clone();
    b.extend_from_slice(&a);
    let s = b.partial_sums();
    let mut steps = Arr2d::new(k.highest_bit() + 1, s.len(), 0);
    let sum = a.copy_sum();
    let mut left = sum.upper_div(k);
    let mut right = sum;
    while left < right {
        let mid = (left + right) / 2;
        let mut at = 0;
        for i in s.indices() {
            at.maxim(i);
            while at + 1 < s.len() && s[at + 1] - s[i] <= mid {
                at += 1;
            }
            steps[(0, i)] = at;
        }
        for i in 1..steps.d1() {
            for j in s.indices() {
                steps[(i, j)] = steps[(i - 1, steps[(i - 1, j)])];
            }
        }
        let mut good = false;
        for i in 0..n {
            let mut at = i;
            for j in 0..steps.d1() {
                if k.is_set(j) {
                    at = steps[(j, at)];
                }
            }
            if at >= i + n {
                good = true;
                break;
            }
        }
        if good {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    out.print_line(sum + (m - 1) * left);
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
