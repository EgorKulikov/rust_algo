use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::treap::multi_treap_set::MultiTreapSet;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::{PartialSums, UpperDiv};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_int_vec(n);

    let mut ans = 0;
    let mut m = a.clone().sorted()[n.upper_div(2) - 1];
    for x in 0..2 {
        let mut target = (n + x).upper_div(2) as i32;
        for i in a.copy_iter() {
            if i < m {
                target -= 1;
            }
        }
        for s in 0..2 {
            let arr = Vec::with_gen(n, |i| {
                if i % 2 == (s ^ x) {
                    if a[i] == m - 1 { -1 } else { 0 }
                } else {
                    if a[i] == m { 1 } else { 0 }
                }
            });
            let ps = arr.partial_sums();
            let mut set = MultiTreapSet::new();
            for i in (0..=n).rev() {
                if i % 2 == s {
                    ans += set.more_or_eq(&(ps[i] + target));
                }
                set.insert(ps[i]);
            }
        }

        for x in a.iter_mut() {
            *x *= -1;
        }
        m *= -1;
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
