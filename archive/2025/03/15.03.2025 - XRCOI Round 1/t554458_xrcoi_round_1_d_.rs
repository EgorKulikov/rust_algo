//{"name":"T554458 [XRCOI Round 1] D. 似此星辰非昨夜","group":"Luogu","url":"https://www.luogu.com.cn/problem/T554458?contestId=221170","interactive":false,"timeLimit":200,"tests":[{"input":"1\n5\n","output":"18\n"},{"input":"1\n6\n","output":"200\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::number_ext::Power;
use algo_lib::numbers::unsigned_big_int::UBigInt;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    input.read_size();
    let n: UBigInt = input.read();

    if n < UBigInt::from(5) {
        out.print_line(0);
        return;
    }
    type Mod = ModInt7;
    let r1 = &n % 1_000_000_007;
    let r2 = &n % 1_000_000_006 + 1_000_000_006;
    let n = Mod::new_signed(r1);

    let mut ans = Mod::zero();
    for mask in 1i32..32 {
        if (mask & 1) == 0 && (mask & 2) != 0 {
            continue;
        }
        if (mask & 8) == 0 && (mask & 16) != 0 {
            continue;
        }
        let sign = if mask.count_ones() % 2 == 1 {
            Mod::one()
        } else {
            -Mod::one()
        };
        let mut types = 0;
        if (mask & 3) != 0 {
            types += 1;
        }
        if (mask & 4) != 0 {
            types += 1;
        }
        if (mask & 24) != 0 {
            types += 1;
        }
        let first_types = if (mask & 1) != 0 { types - 1 } else { types };
        if (mask & 3) == 3 {
            if (mask & 24) == 24 {
                ans += sign
                    * (Mod::new(types).power(r2 - 3)
                        * Mod::new(first_types)
                        * (n - Mod::one())
                        * (n - Mod::new(2))
                        + Mod::new(types).power(r2 - 2) * (n - Mod::one()));
            } else {
                ans +=
                    sign * Mod::new(first_types) * Mod::new(types).power(r2 - 2) * (n - Mod::one());
            }
        } else {
            if (mask & 24) == 24 {
                ans += sign
                    * (Mod::new(first_types) * Mod::new(types).power(r2 - 2) * (n - Mod::one())
                        + Mod::new(types).power(r2 - 1));
            } else {
                ans += sign * Mod::new(first_types) * Mod::new(types).power(r2 - 1);
            }
        }
    }
    out.print_line(ans);
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
