//{"name":"Annoyed Coworkers","group":"Kattis","url":"https://open.kattis.com/problems/annoyedcoworkers","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4\n1 2\n2 3\n3 4\n4 5\n","output":"7\n\n"},{"input":"3 2\n1 1000\n1000 1\n","output":"1002\n\n"},{"input":"5 2\n1 1\n2 2\n","output":"5\n\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::bin_search::search_first_true;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_long();
    let c = input.read_size();
    let ad = input.read_long_pair_vec(c);

    let left = ad.copy_map(|(a, _)| a).max().unwrap();
    let right = left + h * 1_000_000_000;

    out.print_line(search_first_true(left, right, |x| {
        let mut so_far = 0;
        for (a, d) in ad.copy_iter() {
            so_far += (x - a) / d;
            if so_far >= h {
                return true;
            }
        }
        false
    }));
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
