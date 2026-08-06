use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let v = input.read_long_vec(n);
    let a = input.read_long_table(n, m);

    let mut ans = m;

    #[derive(Default, Clone)]
    struct Node {
        v: i64,
        qty: usize,
        delta_v: i64,
        delta_qty: usize,
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.v = left_val.v.min(right_val.v);
        }

        fn accumulate(&mut self, value: &Self) {
            self.v -= value.delta_v;
            self.qty += value.delta_qty;
            self.delta_v += value.delta_v;
            self.delta_qty += value.delta_qty;
        }

        fn reset_delta(&mut self) {
            self.delta_v = 0;
            self.delta_qty = 0;
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node {
        v: v[i],
        ..Default::default()
    });
    let order = (0..n * m).map(|i| (i / m, i % m)).collect::<Vec<_>>().sorted_by_key(|&(i, j)| Reverse(a[(i, j)]));
    for (i, j) in order {
        st.update(0..=i, &Node { delta_v: a[(i, j)], delta_qty: 1, ..Default::default() });
        loop {
            let res = st.binary_search_in(.., |node| node.v <= 0, |_, pos| pos);
            if let Some(pos) = res {
                ans.minim(st.point_query(pos).qty);
                st.point_update(pos, Node { v: i64::MAX, ..Default::default() });
            } else {
                break;
            }
        }
    }
    out.print_line(ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
