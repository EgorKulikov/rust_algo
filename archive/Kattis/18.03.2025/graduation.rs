//{"name":"Graduation","group":"Kattis","url":"https://open.kattis.com/problems/skolavslutningen","interactive":false,"timeLimit":2000,"tests":[{"input":"2 3 2\nAAB\nABB\n","output":"1\n"},{"input":"2 2 3\nAC\nBC\n","output":"2\n"},{"input":"2 3 3\nABC\nABC\n","output":"3\n"},{"input":"3 5 5\nABECE\nBCDAE\nCADBD\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let st = input.read_char_table(n, m);

    let mut cols = vec![Vec::new(); k];
    for i in 0..n {
        for j in 0..m {
            cols[(st[(i, j)] - b'A') as usize].push(j);
        }
    }
    let mut dsu = DSU::new(m);
    for mut class in cols {
        class.sort();
        class.dedup();
        for (a, b) in class.consecutive_iter_copy() {
            dsu.union(a, b);
        }
    }
    out.print_line(dsu.set_count());
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
