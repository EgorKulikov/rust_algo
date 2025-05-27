//{"name":"F. Knapsack","group":"Universal Cup - The 3rd Universal Cup. Stage 37: Wuhan","url":"https://contest.ucup.ac/contest/2025/problem/10741","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5 4\n3 0\n2 3\n3 1\n1 3\n2 1\n2 20250427\n1000000000 1000000000\n114514 1919810\n","output":"10\n628956724\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultTreeMap;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_utils::UpperDiv;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_long();
    let elements = input.read_long_pair_vec(n);

    let mut qty = DefaultTreeMap::new(0);
    for (a, b) in elements {
        qty[b] += a;
    }
    let ba = qty.into_iter().collect::<Vec<_>>().reversed();
    let mut last = 1_000_000_000;
    let mut qty = 0;
    type Mod = ModIntF;
    let mut ans = Mod::zero();
    for (b, a) in ba {
        if qty != 0 {
            if last > b + 60 {
                qty = i64::MAX;
            } else {
                qty = qty.saturating_mul(1 << (last - b));
            }
        }
        last = b;
        if qty >= a {
            qty -= a;
            continue;
        } else {
            let rem = a - qty;
            let times = rem.upper_div(m);
            ans += Mod::new_wide(times) * Mod::new(2).power(b);
            qty = times * m - rem;
        }
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
