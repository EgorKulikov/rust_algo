//{"name":"E. Bad Day for Bablyonovic","group":"Codeforces - GoForGold Long Challenge - August 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/629150/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n5 3 2\n1 1 2 1 2\n1 5\n1 3\n3 5\n","output":"3\n2\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BTreeSet;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(n).dec();

    let mut pos = vec![n; k];
    let mut set = BTreeSet::new();
    for i in 0..k {
        set.insert((n, i));
    }
    let mut once = vec![n, n];
    for i in (0..n).rev() {
        set.remove(&(pos[a[i]], a[i]));
        pos[a[i]] = i;
        set.insert((pos[a[i]], a[i]));
        once.push(set.last().unwrap().0);
    }
    once.reverse();
    let mut dp = vec![once];
    for i in 0..20 {
        let mut next = vec![n; n + 2];
        for j in 0..n {
            next[j] = dp[i][dp[i][j] + 1];
        }
        dp.push(next);
    }

    for _ in 0..q {
        let mut l = input.read_size() - 1;
        let r = input.read_size() - 1;

        let mut ans = 1;
        for i in (0..=20).rev() {
            if dp[i][l] <= r {
                l = dp[i][l] + 1;
                ans += 1 << i;
            }
        }
        out.print_line(ans);
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
