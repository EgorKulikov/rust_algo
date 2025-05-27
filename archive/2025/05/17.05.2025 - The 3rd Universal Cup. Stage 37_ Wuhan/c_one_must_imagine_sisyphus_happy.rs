//{"name":"C. One Must Imagine Sisyphus Happy","group":"Universal Cup - The 3rd Universal Cup. Stage 37: Wuhan","url":"https://contest.ucup.ac/contest/2025/problem/10738","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 3\n0 7 1\n5 4\n6 16 10 5 10\n3 3\n2 1 20\n","output":"4 3 4\n6 3 4 3\n5 3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n);

    let mut direct_delta = DefaultHashMap::new(0);
    let mut with_shift = DefaultHashMap::new(0);
    let mut ans = vec![0; m];
    for i in 0..n {
        let direct = i;
        let reverse = 2 * (n - 1) - i;
        let first = direct + a[i] + 1;
        let rem_first = first % (2 * n - 1);
        if rem_first <= direct {
            let period = first / (2 * n - 1);
            direct_delta[period] += 1;
            continue;
        }
        if rem_first > reverse {
            let period = first / (2 * n - 1) + 1;
            direct_delta[period] += 1;
            continue;
        }
        let first_end = first - rem_first + reverse;
        let second = first_end + a[i] + 1;
        let rem_second = second % (2 * n - 1);
        if rem_second <= direct {
            let period = second / (2 * n - 1);
            direct_delta[period] += 1;
            with_shift[(first / (2 * n - 1), period)] += 1;
            continue;
        }
        if rem_second > reverse {
            let period = second / (2 * n - 1) + 1;
            direct_delta[period] += 1;
            with_shift[(first / (2 * n - 1), period)] += 1;
            continue;
        }
        ans[0] += 1;
        if m < first / (2 * n - 1) {
            continue;
        }
        let period = second / (2 * n - 1) - first / (2 * n - 1);
        with_shift[(first / (2 * n - 1), period)] += 1;
    }
    for (x, d) in direct_delta {
        for i in (0..m).step_by(x) {
            ans[i] += d;
        }
    }
    for ((x, y), d) in with_shift {
        for i in (x..m).step_by(y) {
            ans[i] += d;
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
