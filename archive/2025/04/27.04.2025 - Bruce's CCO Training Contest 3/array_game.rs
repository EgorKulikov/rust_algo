//{"name":"Array Game","group":"DMOJ","url":"https://dmoj.ca/problem/oly25practice9","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n10 2 9 2\n10 9 10 9\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);

    if b.copy_count(b[0]) == n {
        let mut ans = 0;
        for c in a.split(|&x| x > b[0]) {
            if c.contains(&b[0]) {
                ans += c.len();
            }
        }
        out.print_line(ans);
        return;
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        let mut start = i;
        while start > 0 && a[start - 1] <= a[i] {
            start -= 1;
        }
        let mut can_upgrade = true;
        for j in start..n {
            let mut cur = if j == 0 { 0 } else { ans[j - 1] };
            if a[j] > a[i] {
                can_upgrade = false;
            }
            if can_upgrade && b[j] == a[i] {
                cur += 1;
            }
            ans[j].maxim(cur);
        }
    }
    out.print_line(ans[n - 1]);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
