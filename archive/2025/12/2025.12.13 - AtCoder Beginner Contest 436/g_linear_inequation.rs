//{"name":"G - Linear Inequation","group":"AtCoder - AtCoder Beginner Contest 436","url":"https://atcoder.jp/contests/abc436/tasks/abc436_g","interactive":false,"timeLimit":2000,"tests":[{"input":"4 6\n5 4 3 2\n","output":"10\n"},{"input":"6 89\n4 7 5 10 7 6\n","output":"38469\n"},{"input":"1 1000000007\n1\n","output":"1755655\n"},{"input":"20 738894495848985641\n40 58 13 24 65 11 63 29 98 75 40 77 15 50 83 85 35 46 38 37\n","output":"31156940\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n);

    type Mod = ModIntF;
    let mut qty = DefaultHashMap::new(Mod::zero());
    qty[m] = Mod::one();
    while qty.len() > 1 || qty[0] == Mod::zero() {
        for i in 0..n {
            let mut next = DefaultHashMap::new(Mod::zero());
            for (k, v) in qty {
                next[k] += v;
                if k >= a[i] {
                    next[k - a[i]] += v;
                }
            }
            qty = next;
        }
        let mut next = DefaultHashMap::new(Mod::zero());
        for (k, v) in qty {
            next[k / 2] += v;
        }
        qty = next;
    }
    out.print_line(qty[0]);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
