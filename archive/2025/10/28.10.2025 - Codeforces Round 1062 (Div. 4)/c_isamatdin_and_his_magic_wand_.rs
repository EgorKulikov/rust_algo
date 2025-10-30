//{"name":"C. Isamatdin and His Magic Wand!","group":"Codeforces - Codeforces Round 1062 (Div. 4)","url":"https://codeforces.com/contest/2167/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n4\n2 3 1 4\n5\n3 2 1 3 4\n4\n3 7 5 1\n2\n1000000000 2\n3\n1 3 5\n5\n2 5 3 1 7\n4\n2 4 8 6\n","output":"1 2 3 4\n1 2 3 3 4\n3 7 5 1\n1000000000 2\n1 3 5\n1 2 3 5 7\n2 4 8 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let num_odd = a.copy_filter(|&x| x % 2 == 1).count();
    if num_odd == 0 || num_odd == n {
        out.print_line(a);
    } else {
        out.print_line(a.sorted());
    }
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
