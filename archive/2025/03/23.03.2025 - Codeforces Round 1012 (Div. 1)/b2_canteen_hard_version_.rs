//{"name":"B2. Canteen (Hard Version)","group":"Codeforces - Codeforces Round 1012 (Div. 1)","url":"https://codeforces.com/contest/2089/problem/B2","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n3 0\n1 1 4\n5 1 4\n4 0\n1 2 3 4\n4 3 2 1\n4 0\n2 1 1 2\n1 2 2 1\n8 0\n1 2 3 4 5 6 7 8\n8 7 6 5 4 3 2 1\n3 6\n1 1 4\n5 1 4\n4 1\n1 2 3 4\n4 3 2 1\n4 1\n2 1 1 2\n1 2 2 1\n4 2\n2 1 1 2\n1 2 2 1\n","output":"1\n4\n4\n8\n0\n2\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::bin_search::search_first_true;
use algo_lib::misc::test_type::TaskType;
use std::collections::VecDeque;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_long();
    let mut a = input.read_long_vec(n);
    let mut b = input.read_long_vec(n);

    if a.copy_sum() <= k {
        out.print_line(0);
        return;
    }
    let mut pos = 0;
    let mut delta = 0;
    let mut max_delta = 0;
    for i in 0..n {
        delta += b[i] - a[i];
        if max_delta.maxim(delta) {
            pos = i + 1;
        }
    }
    a.rotate_left(pos);
    b.rotate_left(pos);
    let ans = search_first_true(0, n, |x| {
        let mut need = 0;
        let mut cur = Vec::new();
        let mut add = 0;
        let mut pos = 0;
        let mut best = VecDeque::new();
        let mut delta = 0;
        for i in 0..n {
            delta += a[i] - b[i];
            delta.maxim(0);
            if delta > add {
                cur.push(delta);
                while let Some(&(_, last)) = best.back() {
                    if last >= delta {
                        best.pop_back();
                    } else {
                        break;
                    }
                }
                best.push_back((cur.len() - 1, delta));
                if cur.len() - pos > x {
                    let (idx, _) = best.pop_front().unwrap();
                    add = cur[idx];
                    pos = idx + 1;
                }
            } else {
                need += add;
                cur.clear();
                add = 0;
                pos = 0;
                best.clear();
                delta = 0;
            }
        }
        assert_eq!(delta, 0);
        need <= k
    });
    out.print_line(ans + 1);
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
