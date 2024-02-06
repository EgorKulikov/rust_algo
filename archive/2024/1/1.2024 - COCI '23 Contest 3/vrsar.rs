//{"name":"#2 - Vrsar","group":"DMOJ - COCI '23 Contest 3","url":"https://dmoj.ca/problem/coci23c3p2","interactive":false,"timeLimit":1000,"tests":[{"input":"3 1\n3 7 0\n6 11 3\n10 13 5\n1\n","output":"6\n"},{"input":"3 2\n5 10 3\n3 6 1\n1 5 0\n0 3\n","output":"5 8\n"},{"input":"1 3\n3 3 3\n0 1 2\n","output":"0 1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Vrsar"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::compress::compress;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let rinks = input.read_vec::<(i32, i32, i32)>(n);

    #[derive(Clone, Copy)]
    struct Node {
        left_pos: i32,
        right_pos: i32,
        left_skate: i32,
        right_skate: i32,
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self {
                left_pos: i32::MAX / 2,
                right_pos: 0,
                left_skate: 0,
                right_skate: 0,
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
            self.left_pos = left_val.left_pos;
            self.right_pos = right_val.right_pos;
            self.left_skate = left_val
                .left_skate
                .max(right_val.left_skate - (right_val.left_pos - left_val.left_pos));
            self.right_skate = right_val
                .right_skate
                .max(left_val.right_skate - (right_val.right_pos - left_val.right_pos));
        }

        fn accumulate(&mut self, _value: &Self, _left: usize, _right: usize) {}

        fn reset_delta(&mut self, _left: usize, _right: usize) {}
    }

    let pos = rinks.iter().map(|&(x, _, _)| x).collect::<Vec<_>>();
    let (at, [pos]) = compress([&pos]);
    let mut t = vec![0; at.len()];
    for ((_, y, _), i) in rinks.into_iter().zip(pos) {
        t[i].maxim(y);
    }
    let mut st = SegmentTree::from_generator(at.len(), |i| Node {
        left_pos: at[i],
        right_pos: at[i],
        left_skate: t[i],
        right_skate: t[i],
    });

    let a = input.read_int_vec(m);
    let mut ans = Vec::with_capacity(m);
    for a in a {
        let p = at.upper_bound(&a);
        let left_res = st.query(..p);
        assert!(left_res.right_pos <= a);
        let left_res = left_res.right_skate - (a - left_res.right_pos);
        let p = at.lower_bound(&a);
        let right_res = st.query(p..);
        assert!(right_res.left_pos >= a);
        let right_res = right_res.left_skate - (right_res.left_pos - a);
        ans.push(0.max(left_res).max(right_res));
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
    let test_type = TestType::Single;
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
    tester::stress_test();
}
//END MAIN
