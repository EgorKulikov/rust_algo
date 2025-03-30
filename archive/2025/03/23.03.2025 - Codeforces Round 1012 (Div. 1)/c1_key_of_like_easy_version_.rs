//{"name":"C1. Key of Like (Easy Version)","group":"Codeforces - Codeforces Round 1012 (Div. 1)","url":"https://codeforces.com/contest/2089/problem/C1","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n3 1 0\n3 2 0\n2 5 0\n9 104 0\n","output":"1 0 0\n500000004 1 500000004\n200000004 800000008\n869203933 991076635 39374313 496894434 9358446 51822059 979588764 523836809 38844739\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::mem::swap;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::invertible::Invertible;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let l = input.read_size();
    let _k = input.read_size();

    type Mod = ModInt7;
    let mut ans = vec![Mod::zero(); n];
    let mut cur = vec![Mod::zero(); n];
    cur[0] = Mod::one();
    let mut next = vec![Mod::zero(); n];
    for i in 0..l {
        let vars = l - i;
        next.fill(Mod::zero());
        let add = Mod::from_index(vars).inv().unwrap();
        let small = vars / n;
        let big = small + 1;
        let big_qty = vars % n;
        for j in 0..n {
            for k in 0..big_qty {
                ans[(j + k) % n] += cur[j] * add * Mod::from_index(big);
                next[(j + k + 1) % n] += cur[j] * add * Mod::from_index(big);
            }
            for k in big_qty..n {
                ans[(j + k) % n] += cur[j] * add * Mod::from_index(small);
                next[(j + k + 1) % n] += cur[j] * add * Mod::from_index(small);
            }
        }
        swap(&mut cur, &mut next);
    }
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
