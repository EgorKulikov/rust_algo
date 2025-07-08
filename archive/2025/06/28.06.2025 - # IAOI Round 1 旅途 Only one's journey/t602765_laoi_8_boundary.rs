//{"name":"T602765 「LAOI-8」Boundary","group":"Luogu","url":"https://www.luogu.com.cn/problem/T602765?contestId=187787","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3\n1 3 2\n9\n1 2 3 4 5 6 7 8 9\n","output":"4\n13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    let mut ans = a[0].abs_diff(a[n - 1]) + n;
    let mut left = usize::MAX / 2;
    for i in 1..n - 1 {
        ans.minim(left + a[i].abs_diff(a[n - 1]) + n + 2);
        left.minim(a[i].abs_diff(a[0]));
    }
    for i in 2..n - 1 {
        ans.minim(a[i - 1].abs_diff(a[0]) + a[i].abs_diff(a[n - 1]) + n);
    }
    let mut good = vec![10 * n; n];
    for i in 1..n - 1 {
        ans.minim(n + a[i].abs_diff(a[n - 1]) + good[a[i - 1]] + 1);
        let cur = a[i + 1];
        if cur > 0 {
            good[cur - 1].minim(a[i].abs_diff(a[0]));
        }
        if cur + 1 < n {
            good[cur + 1].minim(a[i].abs_diff(a[0]));
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
