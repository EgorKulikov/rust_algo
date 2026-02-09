//{"name":"D - Merchant on the Highway","group":"AtCoder - AtCoder Weekday Contest 0001 Beta","url":"https://atcoder.jp/contests/awc0001/tasks/awc0001_d","interactive":false,"timeLimit":2000,"tests":[{"input":"5 10 2\n8 3\n5 4\n10 5\n3 2\n7 3\n","output":"21\n"},{"input":"4 5 1\n100 2\n200 3\n150 2\n50 1\n","output":"350\n"},{"input":"10 50 3\n1000000000 10\n500000000 8\n800000000 12\n300000000 5\n600000000 15\n900000000 20\n400000000 7\n700000000 11\n200000000 6\n550000000 9\n","output":"3450000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let ab = input.read_vec::<(i64, usize)>(n);

    let mut mem = Memoization3d::new(n + 1, n, m + 1, |mem, cur, last, rem| -> i64 {
        if cur == n || cur > last + k {
            0
        } else {
            let mut ans = mem.call(cur + 1, last, rem);
            if rem >= ab[cur].1 {
                ans.maxim(ab[cur].0 + mem.call(cur + 1, cur, rem - ab[cur].1));
            }
            ans
        }
    });
    let mut ans = 0;
    for i in 0..n {
        ans.maxim(ab[i].0 + mem.call(i + 1, i, m - ab[i].1));
    }
    out.print_line(ans);
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
