//{"name":"Divide into 2 groups","group":"Eolymp - Basecamp - Educational Round 1","url":"https://basecamp.eolymp.com/en/compete/1v399r4bst3f1apjnuj8as5pbc/problem/4","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n","output":"2 2\n1 4\n2 3\n"},{"input":"5\n","output":"3 2\n1 2 4\n3 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut a = Vec::new();
    let mut a_sum = 0;
    let mut b = Vec::new();
    let mut b_sum = 0;
    for i in (1..=n).rev() {
        if a_sum <= b_sum {
            a_sum += i;
            a.push(i);
        } else {
            b_sum += i;
            b.push(i);
        }
    }
    out.print_line((a.len(), b.len()));
    out.print_line(a);
    out.print_line(b);
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
