use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let l = input.read_size_vec(n);

    type Mod = ModIntF;

    let mut good = true;
    for i in 0..n {
        if l[i] != i + 1 {
            good = false;
            break;
        }
    }
    if good {
        out.print_line(Mod::from(k) + Mod::from(k) * (k - 1) * (Mod::from(2).power(n - 1) - 1));
        return;
    }
    if k <= 2 {
        out.print_line(0);
        return;
    }
    let mut ans = Mod::from(1);
    let mut min_delta = 1;
    for i in 1..n {
        if l[i] == 1 {
            out.print_line(0);
            return;
        }
        if l[i] != l[i - 1] + 1 {
            if l[i] > l[i - 1] {
                out.print_line(0);
                return;
            }
            let delta = l[i - 1] + 1 - l[i];
            if delta + 1 < min_delta {
                out.print_line(0);
                return;
            }
            ans *= Mod::from(2).power(delta.saturating_sub(min_delta)) * (k - 2);
            min_delta = l[i];
        }
    }
    ans *= Mod::from(k) * (k - 1) * (Mod::from(2).power(l[n - 1] - min_delta));
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
