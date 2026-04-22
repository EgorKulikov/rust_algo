//{"name":"D - Repainting the Fence","group":"AtCoder - AtCoder Weekday Contest 0053 Beta","url":"https://atcoder.jp/contests/awc0053/tasks/awc0053_d","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n1 3 2\n2 5 4\n4 4 7\n","output":"2 4 4 7 4\n"},{"input":"6 4\n3 6 1\n1 2 9\n2 5 3\n6 6 8\n","output":"9 3 3 3 3 8\n"},{"input":"12 7\n1 12 5\n3 8 2\n6 10 9\n2 2 4\n11 12 7\n5 7 1\n9 9 6\n","output":"5 4 2 2 1 1 1 9 6 9 7 7\n"},{"input":"20 11\n1 5 3\n6 10 4\n11 15 5\n16 20 6\n4 17 8\n2 2 1\n19 20 9\n7 13 2\n1 20 10\n5 5 11\n10 18 12\n","output":"10 10 10 10 11 10 10 10 10 12 12 12 12 12 12 12 12 12 10 10\n"},{"input":"1 0\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    #[derive(Default)]
    struct Node {
        col: usize,
    }
    impl SegmentTreeNode for Node {
        fn accumulate(&mut self, value: &Self) {
            if value.col != 0 {
                self.col = value.col;
            }
        }
        fn reset_delta(&mut self) {
            self.col = 0;
        }
    }
    let mut st = SegmentTree::<Node>::new(n);
    for _ in 0..m {
        let l = input.read_size() - 1;
        let r = input.read_size();
        let c = input.read_size();
        st.update(l..r, &Node { col: c });
    }
    out.print_line_iter((0..n).map(|i| st.point_query(i).col));
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
