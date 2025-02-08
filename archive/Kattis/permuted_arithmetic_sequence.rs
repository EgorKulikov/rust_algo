//{"name":"Permuted Arithmetic Sequence","group":"Kattis","url":"https://open.kattis.com/problems/permutedarithmeticsequence","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5 1 2 3 4 5\n3 20 6 13\n4 5 9 15 19\n","output":"arithmetic\npermuted arithmetic\nnon-arithmetic\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_size();
    let mut a = input.read_int_vec(m);

    fn is_arithmetic(a: &[i32]) -> bool {
        let d = a[1] - a[0];
        for i in 2..a.len() {
            if a[i] - a[i - 1] != d {
                return false;
            }
        }
        true
    }
    if is_arithmetic(&a) {
        out.print_line("arithmetic");
        return;
    }
    a.sort();
    if is_arithmetic(&a) {
        out.print_line("permuted arithmetic");
        return;
    }
    out.print_line("non-arithmetic");
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
