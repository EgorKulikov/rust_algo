//{"name":"Make K Most Frequent","group":"CodeChef - START169A","url":"https://www.codechef.com/START169A/problems/P3169","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 1\n1\n2 2\n1 2\n2 1\n1 2\n4 1\n2 2 1 2\n5 3\n1 1 3 2 2\n","output":"0\n0\n0\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size() - 1;
    let a = input.read_size_vec(n).dec();

    let mut qty = a.qty();
    if qty[k] == qty.copy_max() {
        out.print_line(0);
        return;
    }
    for i in 0..n - 1 {
        qty[a[i]] -= 1;
        if qty[k] == qty.copy_max() {
            out.print_line(1);
            return;
        }
    }
    qty = a.qty();
    for i in (1..n).rev() {
        qty[a[i]] -= 1;
        if qty[k] == qty.copy_max() {
            out.print_line(1);
            return;
        }
    }
    out.print_line(2);
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
