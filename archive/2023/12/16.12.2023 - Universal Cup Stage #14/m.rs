//{"name":"m","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"m"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    #[derive(Copy, Clone, Default)]
    struct Node {
        max: i64,
        min: i64,
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self {
                min: i64::MAX / 2,
                max: 0,
            }
        }

        fn join(
            &mut self,
            left_val: &Self,
            right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
            self.max = left_val.max.max(right_val.max);
            self.min = left_val.min.min(right_val.min);
        }

        fn accumulate(&mut self, _value: &Self, _left: usize, _right: usize) {}

        fn reset_delta(&mut self, _left: usize, _right: usize) {}
    }

    let mut st = SegmentTree::from_generator(n, |i| Node {
        max: a[i],
        min: a[i],
    });
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| a[i]);
    let mut left = n;
    let mut right = 0;
    let mut ans = None;
    for &i in &order {
        left.minim(i);
        right.maxim(i);
        let Node {
            min: in_min,
            max: in_max,
        } = st.query(left..=right);
        let Node {
            min: left_min,
            max: left_max,
        } = st.query(..left);
        let Node {
            min: right_min,
            max: right_max,
        } = st.query(right + 1..);
        let out_min = left_min.min(right_min);
        let out_max = left_max.max(right_max);
        let cur = (in_max - in_min).max(out_max - out_min);
        ans.minim(cur);
    }
    left = n;
    right = 0;
    for &i in order.rev() {
        left.minim(i);
        right.maxim(i);
        let Node {
            min: in_min,
            max: in_max,
        } = st.query(left..=right);
        let Node {
            min: left_min,
            max: left_max,
        } = st.query(..left);
        let Node {
            min: right_min,
            max: right_max,
        } = st.query(right + 1..);
        let out_min = left_min.min(right_min);
        let out_max = left_max.max(right_max);
        let cur = (in_max - in_min).max(out_max - out_min);
        ans.minim(cur);
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test(run, tester::check);
}
//END MAIN
