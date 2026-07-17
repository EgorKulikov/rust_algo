use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::Str;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_size();
    let h = input.read_int();
    let mut a = input.read_size_vec(s);
    drop(input);

    let opts = b"01*";

    if s == 1 {
        out.print_line(a[0]);
        out.print_line(0);
        return;
    }

    if h == 1 {
        let mut order = (0..s).collect::<Vec<_>>().sorted_by_key(|&i| a[i]);
        let mut ans = vec![Str::new(); s];
        let mut set = Vec::with_gen(s, |i| vec![i]);
        let mut total = 0;
        while order.len() > 1 {
            let take = if order.len() % 2 == 0 { 2 } else { 3 };
            let mut sum = 0;
            for i in 0..take {
                sum += a[order[i]];
                for j in set[order[i]].clone() {
                    ans[j].push(opts[i]);
                    if i != 0 {
                        set[order[0]].push(j);
                    }
                }
            }
            for _ in 1..take {
                order.remove(1);
            }
            a[order[0]] = sum;
            order.sort_by_key(|&i| a[i]);
            total += sum;
        }
        out.print_line(total);
        for s in ans.iter_mut() {
            s.reverse();
        }
        out.print_line(ans);
        return;
    }
}

pub static TASK_TYPE: TaskType = TaskType::Classic;
pub static TEST_TYPE: TestType = TestType::MultiNumber;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, true, pre_calc, solve);
    eprint!("\x1B[0m");
    output.flush();
    is_exhausted
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
