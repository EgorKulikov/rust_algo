use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    eprintln!("Case #{}:", _test_case);
    let n = input.read_size();
    let k = input.read_size();
    let p = input.read_size();

    if p == 0 {
        let mut c = Arr2d::new(n + 1, n + 1, 0i64);
        for i in 0..=n {
            c[(i, 0)] = 1;
            for j in 1..=i {
                c[(i, j)] = c[(i - 1, j - 1)] + c[(i - 1, j)];
            }
        }
        if 2 * k > n {
            out.print_line(c[(n, k)]);
        } else {
            out.print_line(c[(n - 1, k - 1)]);
        }
    } else {
        let mut ans = Vec::new();
        let add = if 2 * k > n { None } else { Some(n - 1) };
        let max = if 2 * k > n { n } else { n - 1 };
        let target = if 2 * k > n { k } else { k - 1 };
        let mut rec = RecursiveFunction2::new(|rec, mut v: Vec<usize>, cur: usize| {
            if v.len() == target {
                if let Some(add) = add {
                    v.push(add);
                }
                ans.push(v);
                return;
            }
            if cur >= max || max - cur + v.len() < target {
                return;
            }
            rec.call(v.clone(), cur + 1);
            v.push(cur);
            rec.call(v, cur + 1);
        });
        rec.call(Vec::new(), 0);
        out.print_line(ans.len());
        out.print_per_line(&ans);
    }
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
