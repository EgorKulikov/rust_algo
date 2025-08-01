//{"name":"C. Divine Tree","group":"Codeforces - Codeforces Round 1033 (Div. 2) and CodeNite 2025","url":"https://codeforces.com/contest/2120/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n1 2\n4 6\n","output":"-1\n3\n3 1\n1 2\n2 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut m = input.read_size();

    if m < n || m > n * (n + 1) / 2 {
        out.print_line(-1);
        return;
    }
    if m < 2 * n - 1 {
        out.print_line(m - n + 1);
        if m - n + 1 != 1 {
            out.print_line((m - n + 1, 1));
        }
        for i in 2..=n {
            if i != m - n + 1 {
                out.print_line((1, i));
            }
        }
        return;
    }
    out.print_line(n);
    m -= n;
    if n == 1 {
        return;
    }
    for i in (1..n).rev() {
        if 2 * i <= m {
            out.print_line((n, i));
            m -= i;
            continue;
        }
        out.print_line((n, m - i + 1));
        if m - i + 1 != 1 {
            out.print_line((n, 1));
        }
        for j in 2..=i {
            if j != m - i + 1 {
                out.print_line((1, j));
            }
        }
        return;
    }
    unreachable!();
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
