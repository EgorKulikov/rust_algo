//{"name":"Rock Band","group":"Kattis","url":"https://open.kattis.com/problems/rockband","interactive":false,"timeLimit":4000,"tests":[{"input":"3 8\n4 5 2 1 6 8 3 7\n5 2 4 8 6 1 3 7\n2 5 4 8 1 6 3 7\n","output":"3\n2 4 5\n"},{"input":"2 8\n6 2 8 7 1 3 4 5\n2 8 7 1 3 4 5 6\n","output":"8\n1 2 3 4 5 6 7 8\n"},{"input":"6 3\n1 2 3\n1 3 2\n2 1 3\n2 3 1\n3 1 2\n3 2 1\n","output":"3\n1 2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_size();
    let s = input.read_size();
    let order = input.read_size_table(m, s);

    let mut set = BTreeSet::new();
    let mut added = 0;
    loop {
        let was = set.len();
        let take = was.max(1);

        for i in 0..m {
            for j in added..take {
                set.insert(order[(i, j)]);
            }
        }
        if set.len() == was {
            break;
        }
        added = take;
    }
    out.print_line(set.len());
    out.print_line_iter(set.into_iter());
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
