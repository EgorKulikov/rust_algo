//{"name":"B. Kevin and Geometry","group":"Codeforces - IAEPC Preliminary Contest (Codeforces Round 999, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2061/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n4\n5 5 5 10\n4\n10 5 10 5\n4\n1 2 3 4\n4\n1 1 1 3\n6\n4 2 1 5 7 1\n6\n10 200 30 300 30 100\n4\n100000000 100000000 1 2\n","output":"5 5 5 10\n5 5 10 10\n-1\n-1\n1 1 4 5\n-1\n100000000 100000000 1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n).sorted();

    for i in (0..n - 1).rev() {
        if a[i] == a[i + 1] {
            if i >= 2 {
                out.print_line((a[0], a[1], a[i], a[i + 1]));
                return;
            }
            if a[0] + a[1] + a[2] > a[3] {
                out.print_line(&a[..=3]);
                return;
            }
            for j in 3..n - 1 {
                if a[i] + a[i + 1] + a[j] > a[j + 1] {
                    out.print_line((a[i], a[i + 1], a[j], a[j + 1]));
                    return;
                }
            }
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
