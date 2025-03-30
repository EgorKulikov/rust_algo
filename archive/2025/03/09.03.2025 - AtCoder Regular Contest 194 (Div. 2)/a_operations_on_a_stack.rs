//{"name":"A - Operations on a Stack","group":"AtCoder - AtCoder Regular Contest 194 (Div. 2)","url":"https://atcoder.jp/contests/arc194/tasks/arc194_a","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3 -1 -4 5 -9 2\n","output":"8\n"},{"input":"1\n-1\n","output":"-1\n"},{"input":"20\n-14 74 -48 38 -51 43 5 37 -39 -29 80 -44 -55 59 17 89 -37 -68 38 -16\n","output":"369\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut mem = Memoization1d::new(n + 1, |mem, pos| -> i64 {
        if pos == n {
            0
        } else {
            let mut res = mem.call(pos + 1) + a[pos];
            if pos + 2 <= n {
                res.maxim(mem.call(pos + 2));
            }
            res
        }
    });
    out.print_line(mem.call(0));
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
