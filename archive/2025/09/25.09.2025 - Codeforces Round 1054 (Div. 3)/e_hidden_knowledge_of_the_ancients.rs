//{"name":"E. Hidden Knowledge of the Ancients","group":"Codeforces - Codeforces Round 1054 (Div. 3)","url":"https://codeforces.com/contest/2149/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 1 1 1\n5\n5 2 2 3\n1 2 1 3 2\n6 3 1 6\n1 2 3 1 2 3\n4 1 1 2\n7 7 7 7\n7 3 2 4\n1 2 1 2 3 2 1\n","output":"1\n5\n10\n7\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let l = input.read_size();
    let r = input.read_size();
    let a = input.read_size_vec(n);

    let mut first_set = DefaultHashMap::new(0);
    let mut first = 0;
    let mut last = 0;
    let mut last_set = DefaultHashMap::new(0);
    let mut ans = 0;
    for i in 0..n {
        while first < n && first_set.len() < k {
            first_set[a[first]] += 1;
            first += 1;
        }
        if first_set.len() < k {
            break;
        }
        while last < n && last_set.len() <= k {
            last_set[a[last]] += 1;
            last += 1;
        }
        if last_set.len() == k {
            last = n + 1;
        }
        ans += (i + r + 1).min(last).saturating_sub((i + l).max(first));
        first_set[a[i]] -= 1;
        if first_set[a[i]] == 0 {
            first_set.remove(&a[i]);
        }
        last_set[a[i]] -= 1;
        if last_set[a[i]] == 0 {
            last_set.remove(&a[i]);
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
