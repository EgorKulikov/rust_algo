//{"name":"B. Grid Counting","group":"Codeforces - Codeforces Round 1053 (Div. 1)","url":"https://codeforces.com/contest/2150/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5\n2 2 1 0 0\n2\n2 0\n2\n1 1\n4\n3 1 0 0\n4\n0 0 0 0\n","output":"1\n1\n0\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::One;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    type Mod = ModIntF;
    if a.copy_sum() != n {
        out.print_line(0);
        return;
    }
    if a.copy_skip((n + 1) / 2).any(|x| x > 0) {
        out.print_line(0);
        return;
    }
    let mut qty = 2 - n % 2;
    let c = Combinations::<Mod>::new(n + 1);
    let mut ans = Mod::one();
    for i in (0..((n + 1) / 2)).rev() {
        if qty < a[i] {
            out.print_line(0);
            return;
        }
        ans *= c.c(qty, a[i]);
        qty -= a[i];
        qty += 2;
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
