//{"name":"E - Flowerbed Watering Management","group":"AtCoder - AtCoder Weekday Contest 0014 Beta","url":"https://atcoder.jp/contests/awc0014/tasks/awc0014_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n3 1 4 1 5\n2 1 5\n1 2 4 3\n2 2 4\n2 1 5\n","output":"14\n15\n23\n"},{"input":"8 7\n2 5 3 7 1 4 6 2\n2 1 8\n1 3 6 2\n2 3 6\n1 1 4 5\n2 2 7\n1 5 8 1\n2 1 8\n","output":"30\n23\n49\n62\n"},{"input":"10 10\n10 20 30 40 50 60 70 80 90 100\n2 1 10\n1 1 5 10\n2 1 5\n2 6 10\n1 3 8 20\n2 1 10\n1 1 10 5\n2 1 10\n2 5 5\n2 1 1\n","output":"550\n200\n400\n720\n770\n85\n25\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

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
    let c = input.read_long_vec(n);

    #[derive(Default, Clone)]
    struct Node {
        sum: i64,
        delta: i64,
        len: i64,
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.sum = left_val.sum + right_val.sum;
        }

        fn join(left: &Self, right: &Self) -> Self {
            Self {
                sum: left.sum + right.sum,
                delta: 0,
                len: left.len + right.len,
            }
        }

        fn accumulate(&mut self, value: &Self) {
            self.sum += self.len * value.delta;
            self.delta += value.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node {
        sum: c[i],
        delta: 0,
        len: 1,
    });

    for _ in 0..q {
        let t = input.read_size();
        let l = input.read_size() - 1;
        let r = input.read_size();
        match t {
            1 => {
                let v = input.read_long();
                st.update(
                    l..r,
                    &Node {
                        delta: v,
                        ..Default::default()
                    },
                );
            }
            2 => {
                out.print_line(st.query(l..r).sum);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
