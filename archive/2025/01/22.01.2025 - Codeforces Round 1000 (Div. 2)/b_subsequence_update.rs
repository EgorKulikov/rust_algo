//{"name":"B. Subsequence Update","group":"Codeforces - Codeforces Round 1000 (Div. 2)","url":"https://codeforces.com/contest/2063/problem/B","interactive":false,"timeLimit":1500,"tests":[{"input":"6\n2 1 1\n2 1\n3 2 3\n1 2 3\n3 1 3\n3 1 2\n4 2 3\n1 2 2 2\n5 2 5\n3 3 2 3 5\n6 1 3\n3 6 6 4 3 2\n","output":"1\n3\n6\n3\n11\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let l = input.read_size() - 1;
    let r = input.read_size();
    let mut a = input.read_long_vec(n);

    a[..l].sort_unstable();
    a[l..r].sort_unstable_by_key(|i| -i);
    a[r..].sort_unstable();
    let mut pref = 0;
    while pref < l && l + pref < r && a[pref] < a[l + pref] {
        pref += 1;
    }
    let mut ans = a[..pref].iter().sum::<i64>() + a[l + pref..r].iter().sum::<i64>();
    let mut suf = 0;
    while r + suf < n && l + suf < r && a[r + suf] < a[l + suf] {
        suf += 1;
    }
    ans.minim(a[r..r + suf].iter().sum::<i64>() + a[l + suf..r].iter().sum::<i64>());
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

//START MAIN
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
//END MAIN
