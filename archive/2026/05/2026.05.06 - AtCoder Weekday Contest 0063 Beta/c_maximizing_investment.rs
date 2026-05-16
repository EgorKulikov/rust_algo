//{"name":"C - Maximizing Investment","group":"AtCoder - AtCoder Weekday Contest 0063 Beta","url":"https://atcoder.jp/contests/awc0063/tasks/awc0063_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n1 3 2\n","output":"15\n"},{"input":"4 3\n5 5 1 2\n","output":"48\n"},{"input":"10 20\n12 7 25 3 18 30 1 9 14 22\n","output":"31457391\n"},{"input":"40 123456789012345678\n100000000 250000000 333333333 123456789 987654321 456789123 789123456 111111111 222222222 444444444 555555555 666666666 777777777 888888888 999999937 314159265 271828182 161803398 141421356 173205080 999999999 1 2 3 999999998 500000000 600000000 700000000 800000000 900000000 135791357 246802468 102030405 908070605 112233445 556677889 424242424 123123123 321321321 999000999\n","output":"448506850\n"},{"input":"1 1000000000000000000\n1000000000\n","output":"963666222\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let s = input.read_long_vec(n);

    let sum = s.copy_sum();
    let max = s.copy_max();
    type Mod = ModInt7;
    out.print_line(Mod::from(sum) + Mod::from(max) * (Mod::new(2).power(k) - 1));
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
