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
    let v = input.read_long_vec(n);

    #[derive(Default)]
    struct Node {
        delta: i64,
        num_open: usize,
        sum_closed: i64,
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.sum_closed = left_val.sum_closed + right_val.sum_closed;
            self.num_open = left_val.num_open + right_val.num_open;
        }

        fn accumulate(&mut self, value: &Self) {
            self.delta += value.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }

    let mut st = SegmentTree::with_gen(n, |_| Node {
        num_open: 1,
        ..Node::default()
    });

    for _ in 0..q {
        let t = input.read_int();
        match t {
            1 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let x = input.read_long();
                st.update(
                    l..r,
                    &Node {
                        delta: x,
                        ..Node::default()
                    },
                );
            }
            2 => {
                let p = input.read_size() - 1;
                let add = st.point_query(p).delta;
                st.point_update(
                    p,
                    Node {
                        sum_closed: add + v[p],
                        ..Node::default()
                    },
                );
            }
            3 => {
                let mut ans = 0;
                let mut k = input.read_size();
                st.binary_search_in(
                    ..,
                    |node| {
                        if node.num_open >= k {
                            true
                        } else {
                            ans += node.sum_closed;
                            k -= node.num_open;
                            false
                        }
                    },
                    |_, _| (),
                );
                out.print_line(ans);
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
