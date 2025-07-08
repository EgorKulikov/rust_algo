//{"name":"B. Shrinking Array","group":"Codeforces - Educational Codeforces Round 180 (Rated for Div. 2)","url":"https://codeforces.com/contest/2112/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n1 3 3 7\n2\n6 9\n4\n3 1 3 7\n4\n1 3 5 2\n","output":"0\n-1\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut ans = None;
    for i in 0..n {
        let mut min = a[i];
        let mut max = a[i];
        for j in i + 1..n {
            if a[j] >= min - 1 && a[j] <= max + 1 {
                ans.minim(j - i - 1);
                break;
            }
            min.minim(a[j]);
            max.maxim(a[j]);
        }
        min = a[i];
        max = a[i];
        for j in (0..i).rev() {
            if a[j] >= min - 1 && a[j] <= max + 1 {
                ans.minim(i - j - 1);
                break;
            }
            min.minim(a[j]);
            max.maxim(a[j]);
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
