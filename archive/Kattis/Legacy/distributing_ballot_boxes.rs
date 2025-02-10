//{"name":"Distributing Ballot Boxes","group":"Kattis","url":"https://open.kattis.com/problems/ballotboxes","interactive":false,"timeLimit":3000,"tests":[{"input":"2 7\n200000\n500000\n\n4 6\n120\n2680\n3400\n200\n\n-1 -1\n","output":"100000\n1700\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DistributingBallotBoxes"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BinaryHeap;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    input.skip_whitespace();
    if input.peek() == Some(b'-') {
        input.read_int();
        input.read_int();
        return;
    }
    let n = input.read_size();
    let mut b = input.read_int();
    let a = input.read_int_vec(n);

    let mut qty = vec![1; n];
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        b -= 1;
        heap.push((a[i], i));
    }
    for _ in 0..b {
        let (_, idx) = heap.pop().unwrap();
        qty[idx] += 1;
        heap.push((a[idx].upper_div(qty[idx]), idx));
    }
    out.print_line((0..n).map(|i| a[i].upper_div(qty[i])).max());
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
