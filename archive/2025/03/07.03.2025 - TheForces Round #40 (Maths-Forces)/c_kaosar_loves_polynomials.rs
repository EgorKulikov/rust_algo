//{"name":"C. Kaosar loves Polynomials","group":"Codeforces - TheForces Round #40 (Maths-Forces)","url":"https://codeforces.com/gym/105767/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 1\n1 1\n2 1\n5 3\n2 1\n-2 -2\n2003 3\n-1 4 -2 1\n","output":"2\n64\n16\n993748093\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::number_ext::Power;
use std::ops::Add;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    type Mod = ModInt7;

    let k = input.read_size();
    let n = input.read_size();
    let a = input
        .read_int_vec(n + 1)
        .iter_map(Mod::new_signed)
        .collect::<Vec<_>>();

    let sum = a.copy_fold(Mod::zero(), Mod::add);
    out.print_line(sum.power(k));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
