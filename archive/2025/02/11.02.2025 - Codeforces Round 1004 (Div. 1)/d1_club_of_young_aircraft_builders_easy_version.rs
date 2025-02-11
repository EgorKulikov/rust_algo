//{"name":"D1. Club of Young Aircraft Builders (easy version)","group":"Codeforces - Codeforces Round 1004 (Div. 1)","url":"https://codeforces.com/contest/2066/problem/D1","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n3 2 4\n0 0 0 0\n5 5 7\n0 0 0 0 0 0 0\n","output":"6\n190\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let c = input.read_size();
    let m = input.read_size();
    let _a = input.read_size_vec(m);

    type Mod = ModInt7;
    let cc = Combinations::<Mod>::new(m + 1);
    let mut mem = Memoization2d::new(n, m + 1, |mem, fl, qty| {
        if fl == 0 {
            if qty == m {
                Mod::one()
            } else {
                Mod::zero()
            }
        } else {
            let mut res = Mod::zero();
            for i in 0..=c.min(m - qty) {
                res += mem.call(fl - 1, qty + i) * cc.c(c, i);
            }
            res
        }
    });
    out.print_line(mem.call(n - 1, c));
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
