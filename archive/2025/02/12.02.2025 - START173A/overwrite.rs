//{"name":"Overwrite","group":"CodeChef - START173A","url":"https://www.codechef.com/START173A/problems/MINOVER","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 1\n4 1 6\n2\n4 2\n1 2 3 4\n5 3\n4 3\n2 3 7 1\n4 3 6\n","output":"2 1 2\n1 2 3 4\n2 3 6 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::min_max::IterMinMaxPos;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut a = input.read_int_vec(n);
    let mut b = input.read_int_vec(m);

    if m == 1 {
        for i in 0..n {
            a[i].minim(b[0]);
        }
        out.print_line(a);
        return;
    }
    let mp = b.min_position();
    b.rotate_left(mp);
    for i in 0..=n - m {
        if a[i] > b[0]
            || a[i] == b[0] && (i < n - m && a[i + 1] > b[0] || i == n - m && a[i..] > b[..])
        {
            a[i..n - m].fill(b[0]);
            a[n - m..].copy_from_slice(&b);
            break;
        }
    }
    out.print_line(a);
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
