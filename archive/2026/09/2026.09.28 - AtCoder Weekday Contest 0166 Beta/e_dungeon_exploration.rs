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
    let mut ab = input.read_long_pair_vec(n);

    #[derive(Default, Clone)]
    struct Node {
        max: i64,
        sum_b: i64,
        delta: i64,
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.max = left_val.max.max(right_val.max);
            self.sum_b = left_val.sum_b + right_val.sum_b;
        }

        fn accumulate(&mut self, value: &Self) {
            self.max += value.delta;
            self.delta += value.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }
    let mut sum = 0;
    let mut st = SegmentTree::with_gen(n, |i| {
        let res = Node {
            max: ab[i].0 - sum,
            sum_b: ab[i].1,
            delta: 0,
        };
        sum += ab[i].1;
        res
    });
    for _ in 0..q {
        let t = input.read_int();
        if t == 1 {
            let at = input.read_size() - 1;
            let a = input.read_long();
            let b = input.read_long();
            st.update(
                at + 1..,
                &Node {
                    delta: ab[at].1 - b,
                    ..Default::default()
                },
            );
            let to_left = st.query(0..at).sum_b;
            st.point_update(
                at,
                Node {
                    max: a - to_left,
                    sum_b: b,
                    delta: 0,
                },
            );
            ab[at] = (a, b);
        } else {
            let s = input.read_long();
            out.print_line(
                st.binary_search_in(.., |node| node.max > s, |_, at| at)
                    .unwrap_or(n),
            );
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
