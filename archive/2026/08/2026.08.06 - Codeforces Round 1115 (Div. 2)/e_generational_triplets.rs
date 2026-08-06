use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::number_iterator::iterate_with_base;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();

    type Mod = ModInt7;
    let mut mem = Memoization1d::new(61, |mem, h: usize| -> Mod {
        if h <= 1 {
            Mod::new(1)
        } else {
            mem.call(h - 2) * 2 + mem.call(h - 1)
        }
    });
    let mut ans = Mod::zero();
    for (mask, len, _) in iterate_with_base(1, n, 2) {
        let m = mask.highest_bit();
        let mut qq = 0;
        for i in (0..=m).rev() {
            if mask.is_set(i) {
                qq += 1;
            } else {
                break;
            }
        }
        if qq != m + 1 && qq % 2 != 0 {
            continue;
        }
        if m == 0 {
            if len > 0 {
                ans += mem.call(len - 1);
            }
        } else if mask.is_set(m - 1) {
            let mut q = 0;
            while mask.is_set(q) {
                q += 1;
            }
            if m + 1 == q {
                if len > 0 {
                    ans += mem.call(len - 1);
                }
            } else {
                ans += mem.call(len);
                if len > 0 {
                    ans += mem.call(len - 1);
                }
            }
        }
    }
    out.print_line(ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();

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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
