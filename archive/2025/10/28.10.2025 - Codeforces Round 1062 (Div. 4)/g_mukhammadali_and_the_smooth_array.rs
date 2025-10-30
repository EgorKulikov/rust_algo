//{"name":"G. Mukhammadali and the Smooth Array","group":"Codeforces - Codeforces Round 1062 (Div. 4)","url":"https://codeforces.com/contest/2167/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n1\n10\n5\n4\n1 2 2 3\n5 6 7 8\n4\n4 3 2 1\n1 1 1 1\n3\n3 1 2\n100 1 1\n5\n5 5 5 5 5\n10 1 10 1 10\n5\n1 3 2 2 4\n100 1 1 1 100\n6\n10 9 8 7 6 5\n1 100 1 100 1 100\n5\n100 1 100 100 100\n1 100 1 1 1\n4\n2 1 2 1\n5 4 3 2\n7\n1 5 3 4 2 6 7\n10 1 10 1 10 1 10\n","output":"0\n0\n3\n2\n0\n1\n203\n1\n6\n11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

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
    let a = input.read_int_vec(n);
    let c = input.read_long_vec(n);

    let mut mem = Memoization1d::new(n, |rec, pos| {
        let mut res = i64::MAX;
        let mut cur = 0;
        for i in pos + 1..n {
            if a[i] >= a[pos] {
                res.minim(rec.call(i) + cur);
            }
            cur += c[i];
        }
        res.minim(cur);
        res
    });
    let mut ans = i64::MAX;
    let mut cur = 0;
    for i in 0..n {
        ans.minim(mem.call(i) + cur);
        cur += c[i];
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
