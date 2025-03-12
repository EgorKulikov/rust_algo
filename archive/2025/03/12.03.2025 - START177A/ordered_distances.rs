//{"name":"Ordered Distances","group":"CodeChef - START177A","url":"https://www.codechef.com/START177A/problems/ORDDIST","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4\n2 6 5 4\n5 4 6 2\n4\n2 6 5 4\n5 6 4 2\n3\n2 3 1\n3 2 1\n3\n1 2 3\n3 1 2\n","output":"3\n-1\n2\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_int_vec(n);
    let y = input.read_int_vec(n);

    for i in 0..n {
        if x[i] == y[0] {
            let l = Vec::with_gen(n, |j| ((x[i] - x[j]).abs(), x[j])).sorted();
            let mut good = true;
            for j in 0..n {
                if l[j].1 != y[j] {
                    good = false;
                    break;
                }
            }
            if good {
                out.print_line(i + 1);
            } else {
                out.print_line(-1);
            }
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
