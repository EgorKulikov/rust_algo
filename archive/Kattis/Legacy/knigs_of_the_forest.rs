//{"name":"Knigs of the Forest","group":"Kattis","url":"https://open.kattis.com/problems/knigsoftheforest","interactive":false,"timeLimit":1000,"tests":[{"input":"2 4\n2013 2\n2011 1\n2011 3\n2014 4\n2012 6\n","output":"2013\n"},{"input":"2 4\n2011 1\n2013 2\n2012 4\n2011 5\n2014 3\n","output":"unknown\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BinaryHeap;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let k = input.read_size();
    let n = input.read_size();
    let mut moose = input.read_size_pair_vec(n + k - 1);

    let target = moose[0].1;
    moose.sort();
    let mut heap = BinaryHeap::new();
    for i in 0..k - 1 {
        heap.push(moose[i].1);
    }
    for i in k - 1..n + k - 1 {
        heap.push(moose[i].1);
        if heap.pop().unwrap() == target {
            out.print_line(moose[i].0);
            return;
        }
    }
    out.print_line("unknown");
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
