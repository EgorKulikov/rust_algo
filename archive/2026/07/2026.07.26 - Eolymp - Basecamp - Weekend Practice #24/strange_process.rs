use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization4;
use algo_lib::misc::recursive_function::Callable4;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::mod_int::mod_utils::combinations;
use algo_lib::numbers::num_traits::algebra::Zero;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut k = input.read_size();
    let mut a = input.read_int_vec(n);

    if 2 * k > n {
        k = n - k;
        for i in a.iter_mut() {
            *i ^= 1;
        }
    }

    type Mod = ModInt7;
    let mut mem = Memoization4::new(|mem, pos: usize, even: u16, odd: u16, change: usize| -> Mod {
        if pos == n {
            if change == k {
                Mod::from((even + odd) as u32)
            } else {
                Mod::zero()
            }
        } else {
            let is_even = a[pos] % 2 == 0;
            let mut res = Mod::zero();
            if is_even {
                res += mem.call(pos + 1, even.saturating_sub(1), odd + 1, change);
            } else {
                res += mem.call(pos + 1, even + 1, odd.saturating_sub(1), change);
            }
            if change < k {
                if !is_even {
                    res += mem.call(pos + 1, even.saturating_sub(1), odd + 1, change + 1);
                } else {
                    res += mem.call(pos + 1, even + 1, odd.saturating_sub(1), change + 1);
                }
            }
            res
        }
    });
    out.print_line(mem.call(0, 0, 0, 0) / combinations::<u32, Mod>(n, k));
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
