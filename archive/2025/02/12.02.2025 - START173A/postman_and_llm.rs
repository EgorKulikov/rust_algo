//{"name":"Postman and LLM","group":"CodeChef - START173A","url":"https://www.codechef.com/START173A/problems/POSTLLM","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 3\n6 2 1\n5 1\n1 2 3 4 5\n3 2\n1 1 1\n","output":"20 1\n0 5\n0 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::{qty, DefaultHashMap};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::panic::catch_unwind;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let d = input.read_long_vec(n).sorted();

    type Mod = ModIntF;
    for i in 0..=n - m {
        if d[i] == d[i + m - 1] {
            let c = Combinations::<Mod>::new(n + 1);
            let qty = qty(&d);
            let mut ans = Mod::zero();
            for v in qty.into_values() {
                if v >= m {
                    ans += c.c(v, m);
                }
            }
            out.print_line((0, ans));
            return;
        }
    }
    let mut sum = 0;
    let mut cur = 0;
    let c = Combinations::<Mod>::new(n + 1);
    let qty = qty(&d);
    let mut best = i64::MAX;
    let mut ways = Mod::zero();
    let mut qty_inside = DefaultHashMap::new(0);
    for i in 0..m - 1 {
        cur += i as i64 * d[i] - sum;
        sum += d[i];
        qty_inside[d[i]] += 1;
    }
    let Ok((best, ways)) = catch_unwind(move || {
        for i in m - 1..n {
            cur += (m - 1) as i64 * d[i] - sum;
            sum += d[i];
            qty_inside[d[i]] += 1;
            if best.minim(cur) {
                ways = Mod::zero();
            }
            if best == cur {
                let cur_ways = c.c(qty[d[i]], qty_inside[d[i]])
                    * c.c(qty[d[i + 1 - m]], qty_inside[d[i + 1 - m]]);
                ways += cur_ways;
            }
            sum -= d[i + 1 - m];
            qty_inside[d[i + 1 - m]] -= 1;
            cur -= sum - (m - 1) as i64 * d[i + 1 - m];
        }
        (best, ways)
    }) else {
        return;
    };
    out.print_line((best * 2, ways));
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
