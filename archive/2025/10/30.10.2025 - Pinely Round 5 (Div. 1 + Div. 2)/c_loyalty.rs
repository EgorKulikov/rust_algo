//{"name":"C. Loyalty","group":"Codeforces - Pinely Round 5 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2161/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n10 2\n1 2 1 2 1 2 1 2 1 2\n5 10\n2 2 2 2 5\n11 23\n5 5 22 1 21 2 10 3 1 1 2\n1 1\n1\n1 17\n11\n3 100\n44 32 1\n16 100500\n42801 73112 95296 68791 42217 21871 29316 84405 24273 42894 63370 53473 57156 61369 80 27290\n","output":"12\n1 2 2 2 2 2 1 1 1 1\n5\n2 2 2 2 5\n53\n1 1 5 2 1 2 5 3 10 21 22\n1\n1\n0\n11\n0\n44 32 1\n503499\n53473 42894 80 57156 42801 61369 42217 63370 29316 68791 27290 73112 24273 84405 21871 95296\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_long();
    let a = input.read_long_vec(n).sorted();

    let sum = a.copy_sum();
    let q = (sum / x) as usize;
    let mut ans = 0;
    for i in 0..q {
        ans += a[Back(i)];
    }
    let mut small = 0;
    let mut big = n;
    let mut order = Vec::with_capacity(n);
    let mut cur = 0;
    for _ in 0..n {
        if small < n - q && cur / x == (cur + a[small]) / x {
            cur += a[small];
            order.push(a[small]);
            small += 1;
        } else {
            cur += a[big - 1];
            order.push(a[big - 1]);
            big -= 1;
        }
    }
    out.print_line(ans);
    out.print_line(order);
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
