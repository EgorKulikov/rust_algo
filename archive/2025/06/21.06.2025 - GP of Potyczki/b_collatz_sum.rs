//{"name":"B. Collatz Sum","group":"Universal Cup - GP of Potyczki","url":"https://contest.ucup.ac/contest/2135/problem/12151","interactive":false,"timeLimit":1000,"tests":[{"input":"10 2\n","output":"73\n"},{"input":"999888777666 1\n","output":"990122835\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization4;
use algo_lib::misc::recursive_function::Callable4;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::Zero;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let k = input.read_int();

    type Mod = ModInt7;
    let mut mem = Memoization4::new(|mem, start: i64, step: i64, len: i64, k: i32| -> Mod {
        if len == 0 {
            return Mod::zero();
        }
        if k == 0 {
            (Mod::new_wide(start) * 2 + Mod::new_wide(len - 1) * Mod::new_wide(step))
                * Mod::new_wide(len)
                / 2
        } else {
            if start % 2 == 0 && step % 2 == 0 {
                mem.call(start / 2, step / 2, len, k - 1)
            } else if step % 2 == 0 {
                mem.call(start * 3 + 1, step * 3, len, k - 1)
            } else if start % 2 == 0 {
                mem.call(start / 2, step, (len + 1) / 2, k - 1)
                    + mem.call((start + step) * 3 + 1, step * 6, len / 2, k - 1)
            } else {
                mem.call(start * 3 + 1, step * 6, (len + 1) / 2, k - 1)
                    + mem.call((start + step) / 2, step, len / 2, k - 1)
            }
        }
    });
    out.print_line(mem.call(1, 1, n, k));
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

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
