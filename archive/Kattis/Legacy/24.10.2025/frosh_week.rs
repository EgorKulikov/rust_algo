//{"name":"Frosh Week","group":"Kattis","url":"https://open.kattis.com/problems/froshweek2","interactive":false,"timeLimit":3000,"tests":[{"input":"5 4\n150000 100000 160000 100000 180000\n190000 170000 140000 160000\n","output":"4\n"},{"input":"4 4\n180000 185000 199999 100000\n199999 180000 170000 120000\n","output":"3\n"},{"input":"3 3\n199999 180000 170001\n199999 170000 180000\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut t = input
        .iter_int()
        .take(n)
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<BTreeSet<_>>();
    let l = input.read_int_vec(m);

    for i in l {
        if let Some(&(x, idx)) = t.floor(&(i, n)) {
            t.remove(&(x, idx));
        }
    }
    out.print_line(n - t.len());
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
