//{"name":"A. Penalty Kick","group":"Codeforces - AlgoChief Sprint Round 2","url":"https://codeforces.com/gym/105687/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5 4 6\n1 6 4 8 7\n2 3 9 5 10\n5 4 6\n1 2 3 4 5\n6 7 8 9 10\n","output":"Bob\nBob\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_int();
    let k = input.read_int();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);

    fn score(a: &[i32], m: i32) -> (usize, i32) {
        (a.iter_filter(|&&x| x >= m / 2).count(), *a.iter_max())
    }
    if score(&a, k) > score(&b, m) {
        out.print_line("Alice");
    } else {
        out.print_line("Bob");
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
