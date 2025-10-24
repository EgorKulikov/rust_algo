//{"name":"G. Красивое дерево","group":"Codeforces - Codeforces Round 1059 (Div. 3)","url":"https://codeforces.com/contest/2162/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n3\n4\n","output":"-1\n1 3\n2 3\n1 2\n3 1\n4 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    if n > 6 {
        for i in 1..=n {
            if i != 2 && i != n - 4 {
                out.print_line((2, i));
            }
        }
        out.print_line((1, n - 4));
        return;
    }
    if n == 2 {
        out.print_line(-1);
        return;
    }
    if n == 3 {
        out.print_line((1, 3));
        out.print_line((2, 3));
        return;
    }
    if n == 4 {
        out.print_line((1, 2));
        out.print_line((3, 1));
        out.print_line((4, 1));
        return;
    }
    if n == 6 {
        out.print_line((1, 2));
        out.print_line((1, 3));
        out.print_line((1, 4));
        out.print_line((1, 6));
        out.print_line((2, 5));
        return;
    }
    out.print_line((1, 5));
    out.print_line((1, 2));
    out.print_line((2, 3));
    out.print_line((3, 4));
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
