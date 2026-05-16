//{"name":"E - Number of Blocks in an Interval","group":"AtCoder - AtCoder Weekday Contest 0063 Beta","url":"https://atcoder.jp/contests/awc0063/tasks/awc0063_e","interactive":false,"timeLimit":2000,"tests":[{"input":"6 6\n1 1 2 2 2 1\n2 1 6\n2 2 5\n1 3 5 1\n2 1 6\n1 2 4 3\n2 1 6\n","output":"3\n2\n1\n3\n"},{"input":"5 7\n4 5 5 4 4\n2 1 5\n1 2 4 4\n2 1 5\n2 3 3\n1 1 1 7\n2 1 2\n2 4 5\n","output":"3\n1\n1\n2\n1\n"},{"input":"12 12\n1 2 2 3 3 3 2 2 4 4 5 1\n2 1 12\n2 3 10\n1 2 5 2\n2 1 8\n1 9 12 2\n2 1 12\n1 6 6 7\n2 5 7\n1 1 12 3\n2 1 12\n1 4 9 1\n2 1 12\n","output":"7\n4\n4\n4\n3\n1\n3\n"},{"input":"20 18\n1 1 2 3 3 4 4 4 5 6 6 7 8 8 9 9 9 10 1 1\n2 1 20\n2 4 17\n1 3 8 3\n2 1 10\n1 10 15 8\n2 8 18\n1 1 2 2\n2 1 5\n1 16 20 8\n2 10 20\n1 5 5 2\n2 1 20\n1 6 14 1\n2 1 20\n2 6 14\n1 1 20 7\n2 1 20\n2 11 11\n","output":"11\n7\n4\n5\n2\n1\n6\n5\n1\n1\n1\n"},{"input":"1 5\n1000000000\n2 1 1\n1 1 1 1\n2 1 1\n1 1 1 1000000000\n2 1 1\n","output":"1\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let c = input.read_size_vec(n);

    #[derive(Default, Clone)]
    struct Node {
        first_color: usize,
        last_color: usize,
        num_blocks: usize,
        delta: Option<usize>,
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.first_color = left_val.first_color;
            self.last_color = right_val.last_color;
            self.num_blocks = left_val.num_blocks + right_val.num_blocks;
            if left_val.last_color == right_val.first_color {
                self.num_blocks -= 1;
            }
        }

        fn accumulate(&mut self, value: &Self) {
            if let Some(delta) = value.delta {
                self.first_color = delta;
                self.last_color = delta;
                self.num_blocks = 1;
                self.delta = Some(delta);
            }
        }

        fn reset_delta(&mut self) {
            self.delta = None;
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node {
        first_color: c[i],
        last_color: c[i],
        num_blocks: 1,
        delta: None,
    });

    for _ in 0..q {
        let m = input.read_int();
        match m {
            1 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let x = input.read_size();
                st.update(
                    l..r,
                    &Node {
                        delta: Some(x),
                        ..Default::default()
                    },
                );
            }
            2 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                out.print_line(st.query(l..r).num_blocks);
            }
            _ => unreachable!(),
        }
    }
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
