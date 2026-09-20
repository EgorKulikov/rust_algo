use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::One;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let h = input.read_size_vec(n);

    type Mod = ModInt7;
    let max = h.copy_max();
    let mut first_max = n;
    let mut last_max = n;
    for i in 0..n {
        if h[i] == max {
            last_max = i;
            if first_max == n {
                first_max = i;
            }
        }
    }
    let mut ans = Mod::one();
    for i in first_max + 1..last_max {
        if h[i] != max {
            out.print_line(0);
            return;
        }
        ans *= max;
    }
    let mut cur = max;
    for i in (0..first_max).rev() {
        if h[i] > cur {
            out.print_line(0);
            return;
        }
        if i > 0 && h[i - 1] == h[i] {
            ans *= h[i];
        }
        cur = h[i];
    }
    let mut cur = max;
    for i in last_max + 1..n {
        if h[i] > cur {
            out.print_line(0);
            return;
        }
        if i > 0 && h[i - 1] == h[i] {
            ans *= h[i];
        }
        cur = h[i];
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
