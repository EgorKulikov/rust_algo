//{"name":"D - Forming an Adventurer Party","group":"AtCoder - AtCoder Weekday Contest 0041 Beta","url":"https://atcoder.jp/contests/awc0041/tasks/awc0041_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2\n3 5\n8 2\n6 4\n5 3\n","output":"36\n"},{"input":"5 1\n4 7\n2 10\n9 3\n6 6\n5 8\n","output":"40\n"},{"input":"8 3\n5 9\n12 4\n7 8\n10 6\n3 11\n9 5\n6 10\n8 7\n","output":"150\n"},{"input":"15 7\n13 9\n21 4\n8 15\n17 11\n5 20\n19 7\n14 10\n6 18\n25 3\n11 14\n9 16\n16 8\n7 19\n18 12\n10 13\n","output":"880\n"},{"input":"1 1\n1000000 1000000\n","output":"1000000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let ab = input.read_long_pair_vec(n).sorted_by_key(|&(_, b)| -b);

    let mut heap = BinaryHeap::new();
    let mut sum = 0;
    for i in 0..k {
        sum += ab[i].0;
        heap.push(Reverse(ab[i].0));
    }
    let mut ans = sum * ab[k - 1].1;
    for i in k..n {
        let (a, b) = ab[i];
        sum += a;
        heap.push(Reverse(a));
        let Reverse(smallest) = heap.pop().unwrap();
        sum -= smallest;
        ans.maxim(sum * b);
    }
    out.print_line(ans);
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
