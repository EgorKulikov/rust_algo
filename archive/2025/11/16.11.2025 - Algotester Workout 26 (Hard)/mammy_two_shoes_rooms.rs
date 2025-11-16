//{"name":"Mammy Two Shoes’ Rooms","group":"Algotester","url":"https://algotester.com/en/ContestProblem/DisplayWithFile/147293","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n4 7\n7 47\n3\n0 1\n1 6\n2 3\n4 5\n4 5\n5 6\n3\n0 2\n0 2\n1 2\n2 5\n3 4\n5 6\n3\n0 1\n0 2\n1 2\n1 2\n1 4\n2 3\n","output":"possible\npossible\npossible\nimpossible\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::vec_ext::detuple::Detuple;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let lr = input.read_size_pair_vec(n * 2).sorted();

    let (l, r) = lr.detuple();
    let mut first = BTreeSet::new();
    let mut second = BTreeSet::new();

    for i in 0..n * 2 {
        if let Some(&(end, id)) = first.floor(&(l[i], 2 * n)) {
            first.remove(&(end, id));
            second.insert((r[i], i));
        } else if let Some(&(end, id)) = second.first() {
            if end < r[i] {
                second.remove(&(end, id));
                second.insert((r[i], i));
                first.insert((r[id], id));
            } else {
                first.insert((r[i], i));
            }
        } else {
            first.insert((r[i], i));
        }
    }
    if first.is_empty() {
        out.print_line("possible");
    } else {
        out.print_line("impossible");
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
