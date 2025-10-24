//{"name":"E. Maximum OR Popcount","group":"Codeforces - Codeforces Global Round 29 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2147/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 3\n0\n0\n2\n4\n2 2\n1 3\n0\n3\n2 1\n1000000000 1000000000\n1000000000\n","output":"0\n1\n2\n2\n3\n31\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut a = input.read_long_vec(n);

    let mut op = 0;
    let mut ans = Vec::new();
    let mut last = None;
    while op <= 1_000_000_000 {
        let or = a.iter().fold(0, |acc, &x| acc | x);
        if last.maxim(or.count_ones()) {
            ans.push((op, or.count_ones()));
        }
        let mut first_non_set = 0;
        while or.is_set(first_non_set) {
            first_non_set += 1;
        }
        for j in (0..=first_non_set).rev() {
            let or = a.iter().fold(0, |acc, &x| acc | x);
            if or.is_set(j) {
                continue;
            }
            let mut best_delta = i64::MAX;
            let mut at = n;
            for i in 0..n {
                let cur = (1 << j) - (a[i] & i64::all_bits(j));
                if best_delta.minim(cur) {
                    at = i;
                }
            }
            a[at] += best_delta;
            op += best_delta;
        }
    }

    for _ in 0..q {
        let b = input.read_long();
        let pos = ans.lower_bound(&(b, u32::MAX)) - 1;
        out.print_line(ans[pos].1);
    }
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
