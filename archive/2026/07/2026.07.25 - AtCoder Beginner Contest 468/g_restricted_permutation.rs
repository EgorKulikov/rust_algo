use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    if s[0] == b'x' || s[n - 1] == b'x' {
        out.print_line(0);
        return;
    }
    type Mod = ModIntF;
    let mut mem = Memoization2d::new(n + 1, n + 1, |mem, pos, qty| -> Mod {
        if (s[pos - 1] == b'o') != (qty == 1) {
            Mod::zero()
        } else if pos == n {
            Mod::one()
        } else {
            let mut res = mem.call(pos + 1, qty + 1);
            res += mem.call(pos + 1, qty) * 2 * qty;
            if qty >= 2 {
                res += mem.call(pos + 1, qty - 1) * qty * (qty - 1);
            }
            res
        }
    });
    out.print_line(mem.call(1, 1));
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
