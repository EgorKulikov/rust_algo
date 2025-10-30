//{"name":"E. khba Loves to Sleep!","group":"Codeforces - Codeforces Round 1062 (Div. 4)","url":"https://codeforces.com/contest/2167/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"10\n4 1 4\n1 0 2 4\n5 5 4\n0 1 2 3 4\n2 1 4\n4 0\n3 4 6\n2 4 3\n3 2 12\n6 12 0\n4 3 12\n8 12 0 4\n1 1 1000000000\n0\n1 1 1000000000\n1000000000\n3 4 9\n8 7 9\n3 4 9\n2 0 1\n","output":"3\n0 1 2 3 4\n2\n0 1 5 6\n3 9\n2 6 10\n1000000000\n0\n0 1 2 3\n6 7 8 9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let x = input.read_size();
    let a = input.read_size_vec(n).sorted();

    let mut left = 0;
    let mut right = x;

    while left < right {
        let mid = (left + right + 1) / 2;
        let mut start = 0;
        let mut cur = 0;
        for i in a.copy_iter() {
            if start + mid <= i {
                cur += i - (start + mid) + 1;
            }
            start = i + mid;
        }
        if start <= x {
            cur += x - start + 1;
        }
        if cur >= k {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    if left == 0 {
        out.print_line_iter(0..k);
        return;
    }
    let mut ans = Vec::new();
    let mut start = 0;
    for i in a {
        if start + left <= i {
            for j in start..=i - left {
                if ans.len() < k {
                    ans.push(j);
                }
            }
        }
        start = i + left;
        if ans.len() >= k {
            break;
        }
    }
    if ans.iter().len() < k {
        for j in start..=x {
            if ans.len() < k {
                ans.push(j);
            }
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
