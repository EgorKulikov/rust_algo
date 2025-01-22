//{"name":"E. Nested Segments","group":"Codeforces - Codeforces Round 997 (Div. 2)","url":"https://codeforces.com/contest/2056/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1 0\n2 3\n1 1\n2 2\n1 2\n5 2\n1 3\n2 3\n4 1\n1 1\n6 2\n1 3\n4 6\n2300 0\n","output":"1\n1\n2\n5\n4\n187997613\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::One;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut s = input.read_size_pair_vec(m).dec();

    s.sort_by_key(|&(a, b)| (a, Reverse(b)));
    type Mod = ModIntF;
    let mut pos = 0;
    let mut ans = Mod::one();
    let c = Combinations::<Mod>::new(n * 2 + 1);
    let mut rec = RecursiveFunction2::new(|rec, mut start, end| {
        let mut qty = 0;
        while pos < m && s[pos].1 <= end {
            qty += s[pos].0 - start + 1;
            start = s[pos].1 + 1;
            pos += 1;
            rec.call(s[pos - 1].0, s[pos - 1].1);
        }
        qty += end + 1 - start;
        qty -= 1;
        ans *= c.c(qty * 2, qty) / Mod::from_index(qty + 1);
    });
    rec.call(0, n - 1);
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
