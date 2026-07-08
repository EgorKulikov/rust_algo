use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let dm = input.read_size_pair_vec(n).sorted();

    let mut left = 0;
    let mut right = 1_000_000_000;
    while left < right {
        let mid = (left + right) / 2;
        let mut found = false;
        let mut closest = Arr2d::new(10, 32, usize::MAX);
        'outer: for (d, m) in dm.copy_rev() {
            for i in 0..32 {
                if (m | i) == 31 && closest[(9, i)] - d <= mid {
                    found = true;
                    break 'outer;
                }
            }
            for i in (1..9).rev() {
                for j in 0..32 {
                    if closest[(i, j)] - d <= mid {
                        closest[(i + 1, j | m)] = d;
                    }
                }
            }
            closest[(1, m)] = d;
        }
        if found {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    if left == 1_000_000_000 {
        out.print_line(-1);
    } else {
        out.print_line(left);
    }
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
