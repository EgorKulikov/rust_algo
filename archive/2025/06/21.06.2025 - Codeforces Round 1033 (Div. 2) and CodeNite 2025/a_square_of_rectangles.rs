//{"name":"A. Square of Rectangles","group":"Codeforces - Codeforces Round 1033 (Div. 2) and CodeNite 2025","url":"https://codeforces.com/contest/2120/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n100 100 10 10 1 1\n5 3 5 1 5 1\n2 3 1 2 1 1\n8 5 3 5 3 3\n3 3 3 3 2 1\n","output":"NO\nYES\nYES\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let lb = input.read_int_pair_vec(3);

    out.print_line(
        lb[0].0 == lb[2].0 && (lb[0].1 + lb[1].1 + lb[2].1) == lb[0].0
            || lb[0].1 == lb[2].1 && (lb[0].0 + lb[1].0 + lb[2].0) == lb[0].1
            || lb[2].0 == lb[1].0 && lb[2].1 + lb[1].1 == lb[0].1 && lb[0].0 + lb[2].0 == lb[0].1
            || lb[2].1 == lb[1].1 && lb[2].0 + lb[1].0 == lb[0].0 && lb[0].1 + lb[2].1 == lb[0].0,
    );
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
