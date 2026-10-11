use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::hungarian_algorithm::hungarian_algorithm;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

/// Minimum cost of assigning every row to a distinct column (`rows <= cols`).
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n).dec();
    let b = input.read_long_vec(n);

    let mut left = vec![i64::MAX; k];
    let mut right = vec![i64::MIN; k];
    for i in 0..n {
        left[a[i]].minim(b[i]);
        right[a[i]].maxim(b[i]);
    }
    let arr = Arr2d::with_gen(k, k, |i, j| {
        if j * 2 < k {
            -left[i] * (2 * j as i64 - (k - 1) as i64)
        } else {
            -right[i] * (2 * j as i64 - (k - 1) as i64)
        }
    });
    out.print_line(-hungarian_algorithm(&arr));
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
