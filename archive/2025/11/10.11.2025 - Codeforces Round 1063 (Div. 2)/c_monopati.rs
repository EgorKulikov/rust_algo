//{"name":"C. Monopati","group":"Codeforces - Codeforces Round 1063 (Div. 2)","url":"https://codeforces.com/contest/2163/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n1 3\n3 1\n3\n1 2 3\n3 2 1\n4\n1 5 5 5\n5 3 1 2\n4\n8 8 8 8\n8 8 8 8\n6\n6 6 5 7 9 12\n1 4 2 8 5 6\n","output":"2\n5\n4\n8\n25\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_table(2, n);

    let mut ltr_min = vec![a[(0, 0)]];
    let mut ltr_max = vec![a[(0, 0)]];
    for i in 1..n {
        ltr_min.push(ltr_min[Back(0)].min(a[(0, i)]));
        ltr_max.push(ltr_max[Back(0)].max(a[(0, i)]));
    }
    let mut rtl_min = vec![a[(1, n - 1)]];
    let mut rtl_max = vec![a[(1, n - 1)]];
    for i in (0..n - 1).rev() {
        rtl_min.push(rtl_min[Back(0)].min(a[(1, i)]));
        rtl_max.push(rtl_max[Back(0)].max(a[(1, i)]));
    }
    rtl_min.reverse();
    rtl_max.reverse();
    let mut m = vec![2 * n; 2 * n];
    for i in 0..n {
        let min = ltr_min[i].min(rtl_min[i]);
        let max = ltr_max[i].max(rtl_max[i]);
        m[min - 1].minim(max - 1);
    }
    let mut ans = 0;
    let mut cur = 2 * n;
    for i in (0..2 * n).rev() {
        cur.minim(m[i]);
        ans += 2 * n - cur;
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
        TaskType::Classic | TaskType::RunTwice => input.is_empty(),
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
