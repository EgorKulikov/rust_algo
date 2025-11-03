//{"name":"pumpkins_and_tombstones_parts_23","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt9;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_utils::factorial;
use algo_lib::numbers::number_ext::Power;
use std::io::Write;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let e = input.read_size();
    let m = input.read_size();

    type Mod = ModInt9;
    let mut ans = Mod::zero();
    let slots = n - k;
    for segs in k.saturating_sub(1)..=k + 1 {
        if slots < segs {
            continue;
        }
        if slots == 0 {
            ans += Mod::one();
            continue;
        }
        if segs == 0 {
            continue;
        }
        ans += Mod::from(e * m).power(segs)
            * Mod::from(e + m - 1).power(slots - segs)
            * factorial::<Mod>(slots - 1)
            / factorial::<Mod>(segs - 1)
            / factorial::<Mod>(slots - segs)
            * if segs == k { 2 } else { 1 };
    }
    writeln!(out, "Case #{}: {}", test_case, ans).unwrap();
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
