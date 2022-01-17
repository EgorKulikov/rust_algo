//{"name":"#5 - Index","group":"DMOJ - COCI '20 Contest 6","url":"https://dmoj.ca/problem/coci20c6p5","interactive":false,"timeLimit":2500,"tests":[{"input":"7 6\n3 2 3 1 1 4 7\n3 4\n1 7\n1 6\n4 5\n1 2\n5 7\n","output":"1\n3\n3\n1\n2\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Index"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{Operation, SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let q = input.read_usize();
    let p = input.read_usize_vec(n);

    struct Node {
        init: bool,
        vals: Vec<usize>,
    }

    impl SegmentTreeNode for Node {
        fn new(left: usize, right: usize) -> Self {
            Self {
                init: false,
                vals: Vec::with_capacity(right - left),
            }
        }

        fn join(&mut self, left: &Self, right: &Self) {
            if self.init {
                return;
            }
            self.init = true;
            self.vals.extend_from_slice(&left.vals);
            self.vals.extend_from_slice(&right.vals);
            self.vals.sort_unstable();
        }

        fn accumulate(&mut self, _: &Self) {}

        fn reset_delta(&mut self) {}
    }

    let mut st = SegmentTree::from_generator(n, |i| Node {
        init: true,
        vals: vec![p[i]],
    });
    struct Op;
    impl Operation<Node, usize, usize> for Op {
        fn process_result(&mut self, node: &mut Node, args: &usize) -> usize {
            node.vals.len() - node.vals.as_slice().lower_bound(args)
        }

        fn join_results(&mut self, left_res: usize, right_res: usize, _: &usize) -> usize {
            left_res + right_res
        }

        fn empty_result(&mut self, _: usize, _: usize, _: &usize) -> usize {
            0
        }
    }

    let mut op = Op;

    for _ in 0..q {
        let l = input.read_usize() - 1;
        let r = input.read_usize();

        let mut a = 1;
        let mut b = r - l;
        while a < b {
            let mid = (a + b + 1) >> 1;
            let val = st.operation(l, r, &mut op, &mid);
            if val >= mid {
                a = mid;
                b.minim(val);
            } else {
                a.maxim(val);
                b = mid - 1;
            }
        }
        out_line!(a);
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
