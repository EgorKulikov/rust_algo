use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_size();
    let w = input.read_size();
    let q = input.read_size();

    let mut ans = Arr2d::new(h, w, (0, b'A'));
    for i in 1..=q {
        let r = input.read_size() - 1;
        let c = input.read_size() - 1;
        let x = input.read_char();
        ans[(r, c)] = (i, x);
    }
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if i > 0 {
                let cand = ans[(i, j)];
                ans[(i - 1, j)].maxim(cand);
            }
            if j > 0 {
                let cand = ans[(i, j)];
                ans[(i, j - 1)].maxim(cand);
            }
        }
    }
    let ans = Arr2d::with_gen(h, w, |i, j| ans[(i, j)].1);
    out.print_table(&ans);
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
