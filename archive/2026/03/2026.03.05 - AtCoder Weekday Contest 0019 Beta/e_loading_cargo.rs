//{"name":"E - Loading Cargo","group":"AtCoder - AtCoder Weekday Contest 0019 Beta","url":"https://atcoder.jp/contests/awc0019/tasks/awc0019_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 5\n2 3\n4 2\n","output":"2\n"},{"input":"5\n1 100\n2 50\n3 20\n1 10\n2 5\n","output":"5\n"},{"input":"8\n10 90\n20 70\n30 50\n40 30\n50 10\n100 0\n5 1000000000\n1000000000 0\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BinaryHeap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let wd = input.read_size_pair_vec(n).sorted_by_key(|&(w, d)| w + d);

    let mut heap = BinaryHeap::new();
    let mut sum = 0;
    for (w, d) in wd {
        heap.push(w);
        sum += w;
        if sum > d + w {
            sum -= heap.pop().unwrap();
        }
    }
    out.print_line(heap.len());
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        _ => {
            unreachable!();
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
