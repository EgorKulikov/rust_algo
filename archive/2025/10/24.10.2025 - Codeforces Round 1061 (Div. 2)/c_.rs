//{"name":"C. Максимальный НОД на доске","group":"Codeforces - Codeforces Round 1061 (Div. 2)","url":"https://codeforces.com/contest/2156/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n9 1\n4 9 6 8 2 6 7 8 2\n10 1\n4 9 6 8 2 6 7 8 2 7\n7 5\n1 1 2 3 4 5 5\n7 4\n1 1 2 3 4 5 5\n14 3\n14 12 7 12 9 9 12 4 3 1 3 6 9 13\n1 0\n1\n","output":"2\n1\n5\n1\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::primes::factorize::all_divisors;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n).sorted();

    let ad = all_divisors(n + 1, false);
    let mut qty = vec![0; n + 1];
    let mut at = 0;
    let mut ans = 0;
    for i in 1..=n {
        while at < n && a[at] < 4 * i {
            for j in ad[a[at]].copy_iter() {
                qty[j] += 1;
            }
            at += 1;
        }
        if at - qty[i] <= k {
            ans = i;
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
