//{"name":"P2 - Square Sum","group":"DMOJ - Dilhan's Computing Contest 1","url":"https://dmoj.ca/problem/dcc1p2","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 5\n9982 44353\n123456789 123456789\n","output":"7\n1551747\n923038039\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::mem::swap;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::integer_sqrt::IntegerSqrt;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::One;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut a = input.read_u64();
    let mut b = input.read_u64();

    type Mod = ModIntF;

    if a > b {
        swap(&mut a, &mut b);
    }
    let x = a.lower_sqrt();
    let y = b.lower_sqrt();
    let z = (a + b).lower_sqrt();

    fn sum_sq(n: u64) -> Mod {
        let n = Mod::new(n as u32);
        n * (n + Mod::one()) * (Mod::new(2) * n + Mod::one()) / Mod::new(6)
    }

    let p1 = sum_sq(x) + Mod::new(x as u32 + 1);
    let p3 = Mod::new_wide((a + b + 1) as i64) * Mod::new((z - y) as u32) - (sum_sq(z) - sum_sq(y));
    let p2 = Mod::new((y - x) as u32) * Mod::new_wide(a as i64 + 1);
    let ans = p1 + p3 + p2;
    out.print_line(ans);
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
