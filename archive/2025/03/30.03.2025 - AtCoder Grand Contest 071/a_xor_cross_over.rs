//{"name":"A - XOR Cross Over","group":"AtCoder - AtCoder Grand Contest 071","url":"https://atcoder.jp/contests/agc071/tasks/agc071_a","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 3 4 5\n","output":"6\n"},{"input":"7\n1 5 14 23 20 18 20\n","output":"39\n"},{"input":"20\n246260893965 450834729933 1091503137264 661761201979 238822689279 375606126051 183045127603 1004516515418 976478741401 957665143474 451659136716 828528157302 204109014940 184065081345 122138832666 130646707415 144391522538 87966805947 381909891703 343575641318\n","output":"3597854777632\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::ops::BitXor;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut mem = Memoization2d::new(n + 1, n + 1, |mem, l, r| -> (i64, i64) {
        if l == r {
            (0, 0)
        } else if l + 1 == r {
            (a[l], 0)
        } else if l + 2 == r {
            (2 * (a[l] ^ a[l + 1]), 0)
        } else if (r - l) % 2 == 1 {
            let all = a[l..r].iter().copied().fold(0, i64::bitxor);
            let mut res = i64::MAX;
            for i in (l..r).step_by(2) {
                res.minim(mem.call(l, i).0 + mem.call(i + 1, r).0);
            }
            (res + all, res)
        } else {
            let all = a[l..r].iter().copied().fold(0, i64::bitxor);
            let mut res = i64::MAX;
            for i in (l + 2..r).step_by(2) {
                res.minim(mem.call(l, i).0 + mem.call(i, r).0);
            }
            for i in (l..r).step_by(2) {
                res.minim(2 * all + mem.call(l, i).0 + mem.call(i + 1, r).1);
            }
            (res, 0)
        }
    });
    out.print_line(mem.call(0, n).0);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
