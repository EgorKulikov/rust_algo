use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::value;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    value!(Modulo: u32 = 993244853);
    type Mod = ModInt<Modulo>;
    let mut ans = Mod::zero();
    let mut mult = Mod::one();
    let t = input.read_size();

    'outer:
    for _ in 0..t {
        mult *= 131;

        let n = input.read_size();
        let d = input.read_int_vec(n);
        let mut cur = 0;
        let mut skipped = None;

        let mut expected = 1;
        for i in d {
            cur += i;
            if Some(cur) == skipped {
                skipped = None;
            } else if cur == expected {
                expected += 1;
            } else if skipped.is_none() && cur == expected + 1 {
                skipped = Some(expected);
                expected += 2;
            } else {
                ans += mult * 13579;
                continue 'outer;
            }
        }
        ans += mult * 65537;
    }

    out.print_line(ans);
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
