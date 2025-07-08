//{"name":"E. Make it Zero","group":"Codeforces - EPIC Institute of Technology Round Summer 2025 (Codeforces Round 1036, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2124/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n1 2 3\n2\n2 5\n4\n5 3 1 5\n","output":"1\n1 2 3\n-1\n2\n3 1 1 1\n2 2 0 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_long_vec(n);

    let p = a.partial_sums();
    for i in 1..n {
        if p[i] * 2 == p[n] {
            out.print_line(1);
            out.print_line(a);
            return;
        }
        let u = p[i - 1];
        let v = p[n] - p[i];
        if (u - v).abs() < a[i - 1] {
            if (u - v).abs() % 2 != a[i - 1] % 2 {
                out.print_line(-1);
                return;
            }
            if a[i - 1] > u + v {
                out.print_line(-1);
                return;
            }
            let mut ans = vec![0; n];
            let y = (a[i - 1] - (u - v).abs()) / 2;
            if u < v {
                for j in 0..i - 1 {
                    ans[j] = a[j];
                    a[j] = 0;
                }
                ans[i - 1] = y;
                a[i - 1] -= y;
                let mut rem = u - y;
                for j in i..n {
                    let cur = a[j].min(rem);
                    ans[j] = cur;
                    a[j] -= cur;
                    rem -= cur;
                }
            } else {
                for j in i..n {
                    ans[j] = a[j];
                    a[j] = 0;
                }
                ans[i - 1] = y;
                a[i - 1] -= y;
                let mut rem = v - y;
                for j in 0..i - 1 {
                    let cur = a[j].min(rem);
                    ans[j] = cur;
                    a[j] -= cur;
                    rem -= cur;
                }
            }
            out.print_line(2);
            out.print_line(ans);
            out.print_line(a);
            return;
        }
    }
    out.print_line(-1);
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
