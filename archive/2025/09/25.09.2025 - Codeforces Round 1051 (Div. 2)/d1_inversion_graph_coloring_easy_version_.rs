//{"name":"D1. Inversion Graph Coloring (Easy Version)","group":"Codeforces - Codeforces Round 1051 (Div. 2)","url":"https://codeforces.com/contest/2143/problem/D1","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n4\n4 2 3 1\n7\n7 6 1 2 3 3 2\n5\n1 1 1 1 1\n11\n7 2 1 9 7 3 4 1 3 5 3\n","output":"13\n73\n32\n619\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::One;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    type Mod = ModInt7;
    let mut mem = Memoization3d::new(n + 1, n, n, |mem, p, x, y| {
        if p == n {
            Mod::one()
        } else {
            let mut res = mem.call(p + 1, x, y);
            if a[p] >= x {
                res += mem.call(p + 1, a[p], y);
            } else if a[p] >= y {
                res += mem.call(p + 1, x, a[p]);
            }
            res
        }
    });
    out.print_line(mem.call(0, 0, 0));
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
