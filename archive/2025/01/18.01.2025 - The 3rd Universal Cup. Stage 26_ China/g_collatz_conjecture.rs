//{"name":"G. Collatz Conjecture","group":"Universal Cup - The 3rd Universal Cup. Stage 26: China","url":"https://contest.ucup.ac/contest/1894/problem/9981","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n2 1 1\n2 1 2\n2 1 3\n2 1 100\n314 159 265\n314 159 2653\n314 159 26535\n","output":"Yes\nYes\nNo\nNo\nYes\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::dynamic_value;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt;
use algo_lib::numbers::num_traits::invertible::Invertible;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_long();
    let b = input.read_long();
    let n = input.read_long();

    dynamic_value!(Modulo: u32 = a as u32);
    type Mod = ModInt<Modulo>;
    let b_inv = Mod::new_from_wide(b).inv().unwrap();
    let x = (n - 1) % b + 1;
    let steps = (Mod::new_from_wide(-x) * b_inv).val() as i64;
    out.print_line(n <= x + b * steps);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
