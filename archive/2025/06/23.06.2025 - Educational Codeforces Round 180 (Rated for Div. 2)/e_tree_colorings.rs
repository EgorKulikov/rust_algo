//{"name":"E. Tree Colorings","group":"Codeforces - Educational Codeforces Round 180 (Rated for Div. 2)","url":"https://codeforces.com/contest/2112/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n1\n3\n5\n7\n9\n","output":"1\n2\n3\n4\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::factorize::all_divisors;

type PreCalc = Vec<i32>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let m = input.read_size();

    out.print_line(data[m]);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = Vec::new();

    let d = all_divisors(500_000, false);
    let mut mem = Memoization1d::new(500_001, |mem, i| -> i32 {
        if i % 2 == 0 {
            -2
        } else if i == 1 {
            0
        } else {
            let mut res = 1 + mem.call(i - 2);
            for &dd in d[i].iter() {
                if dd != 1 && dd != i {
                    res.minim(mem.call(dd) + mem.call(i / dd));
                }
            }
            res
        }
    });
    for i in 0..=500_000 {
        pre_calc.push(mem.call(i) + 1);
    }

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
