//{"name":"L - Lazy Segment Tree","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_l","interactive":false,"timeLimit":5000,"tests":[{"input":"5 5\n0 1 0 0 1\n2 1 5\n1 3 4\n2 2 5\n1 1 3\n2 1 2\n","output":"2\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LLazySegmentTree"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::mem::swap;

fn solve(input: &mut Input) {
    let n = input.read();
    let q = input.read();
    let a = input.read_vec::<u8>(n);

    #[derive(Copy, Clone, Default)]
    struct Node {
        zeroes: u64,
        ones: u64,
        ans_dir: u64,
        ans_inv: u64,
        inverted: bool,
    }

    impl Node {
        const INVERTED: Self = Self {
            inverted: true,
            zeroes: 0,
            ones: 0,
            ans_dir: 0,
            ans_inv: 0,
        };
    }

    impl SegmentTreeNode for Node {
        fn new(left: usize, right: usize) -> Self {
            Self {
                zeroes: (right - left) as u64,
                ..Default::default()
            }
        }

        fn join(&mut self, left: &Self, right: &Self) {
            self.zeroes = left.zeroes + right.zeroes;
            self.ones = left.ones + right.ones;
            self.ans_dir = left.ans_dir + right.ans_dir + left.ones * right.zeroes;
            self.ans_inv = left.ans_inv + right.ans_inv + left.zeroes * right.ones;
        }

        fn accumulate(&mut self, value: &Self) {
            if value.inverted {
                swap(&mut self.zeroes, &mut self.ones);
                swap(&mut self.ans_dir, &mut self.ans_inv);
                self.inverted ^= true;
            }
        }

        fn reset_delta(&mut self) {
            self.inverted = false;
        }
    }

    let mut st = SegmentTree::new(n);

    for i in 0..n {
        if a[i] == 1 {
            st.point_update(i, &Node::INVERTED);
        }
    }

    for _ in 0usize..q {
        let t: u8 = input.read();
        let l = input.read::<usize>() - 1;
        let r = input.read();
        if t == 1 {
            st.update(l, r, &Node::INVERTED);
        } else {
            out_line!(st.query(l, r).ans_dir);
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
