//{"name":"D. Kevin and Numbers","group":"Codeforces - IAEPC Preliminary Contest (Codeforces Round 999, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2061/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n2 1\n4 5\n9\n2 1\n3 6\n9\n4 2\n1 2 2 2\n3 4\n4 2\n1 1 3 3\n3 5\n4 2\n1 2 3 4\n3 5\n5 5\n1 2 3 4 5\n5 4 3 2 1\n4 2\n1 1 1 1\n1 1\n4 4\n1 1 1 1\n1 1 1 2\n1 1\n1\n1000000000\n","output":"Yes\nNo\nYes\nYes\nNo\nYes\nNo\nNo\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use std::collections::BinaryHeap;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(m);

    if a.copy_sum() != b.copy_sum() {
        out.print_line(false);
        return;
    }
    let mut a_heap = BinaryHeap::from(a);
    let mut b_heap = BinaryHeap::from(b);
    while !a_heap.is_empty() {
        let a = a_heap.pop().unwrap();
        let b = b_heap.pop().unwrap();
        if a > b {
            out.print_line(false);
            return;
        }
        if a < b {
            a_heap.push(a);
            b_heap.push(b / 2);
            b_heap.push(b - b / 2);
        }
    }
    out.print_line(true);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
