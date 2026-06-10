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

    #[derive(Default, Clone)]
    struct Node {
        qty: usize,
        sum: usize,
        delta: usize,
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.qty = left_val.qty + right_val.qty;
            self.sum = left_val.sum + right_val.sum;
        }

        fn accumulate(&mut self, value: &Self) {
            self.sum -= self.qty * value.delta;
            self.delta += value.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }

    let mut st = SegmentTree::<Node>::new(q);
    let mut last_r = vec![None; n];
    let mut last_c = vec![0; n];
    for i in 0..q {
        let t = input.read_int();
        let x = input.read_size() - 1;

        match t {
            1 => {
                if let Some(r) = last_r[x] {
                    st.point_update(r, Node::default());
                }
                st.point_update(
                    i,
                    Node {
                        qty: 1,
                        sum: n,
                        ..Default::default()
                    },
                );
                last_r[x] = Some(i);
            }
            2 => {
                st.update(
                    last_c[x]..i,
                    &Node {
                        delta: 1,
                        ..Default::default()
                    },
                );
                last_c[x] = i;
            }
            _ => unreachable!(),
        }
        out.print_line(st.query(..).sum);
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
