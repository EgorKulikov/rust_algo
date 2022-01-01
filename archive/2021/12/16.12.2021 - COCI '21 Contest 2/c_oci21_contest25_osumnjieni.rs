//{"name":"COCI '21 Contest 2 #5 Osumnjičeni","group":"DMOJ","url":"https://dmoj.ca/problem/coci21c2p5","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1 1\n1 1\n3\n1 1\n2 2\n1 2\n","output":"1\n1\n2\n"},{"input":"3\n1 1\n2 2\n3 3\n3\n1 1\n2 3\n1 3\n","output":"1\n1\n1\n"},{"input":"5\n1 3\n3 3\n4 6\n2 3\n1 1\n3\n1 4\n3 5\n1 5\n","output":"3\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"COCI21Contest25Osumnjieni"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{compress, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let range: Vec<(u32, u32)> = input.read_vec(n);

    let l = range.iter().map(|(i, _)| *i).collect_vec();
    let r = range.into_iter().map(|(_, j)| j).collect_vec();
    let (v, (l, r)) = compress!(l, r);
    let x = v.len();

    #[derive(Copy, Clone)]
    struct Node {
        val: usize,
        delta: usize,
    }

    impl SegmentTreeNode for Node {
        fn new(_: usize, _: usize) -> Self {
            Node {
                val: usize::MAX,
                delta: usize::MAX,
            }
        }

        fn join(&mut self, left: &Self, right: &Self) {
            self.val = left.val.min(right.val);
        }

        fn accumulate(&mut self, value: &Self) {
            self.val.minim(value.delta);
            self.delta.minim(value.delta);
        }

        fn reset_delta(&mut self) {
            self.delta = usize::MAX;
        }
    }

    let mut tree = SegmentTree::from_generator(x, |_| Node {
        val: n,
        delta: usize::MAX,
    });
    let mut next = vec![0; n];
    for i in (0..n).rev() {
        next[i] = tree.query(l[i], r[i] + 1).val;
        if i != n - 1 {
            let val = next[i + 1];
            next[i].minim(val);
        }
        tree.update(l[i], r[i] + 1, &Node { val: n, delta: i });
    }
    let mut levels = vec![next];
    while levels.last().unwrap()[0] != n {
        let last = levels.last().unwrap();
        let mut next = Vec::with_capacity(n);
        for i in 0..n {
            if last[i] == n {
                break;
            }
            next.push(last[last[i]]);
        }
        next.resize(n, n);
        levels.push(next);
    }

    let q = input.read();
    for _ in 0..q {
        let mut p = input.read::<usize>() - 1;
        let q = input.read();
        let mut ans = 1;
        for i in (0..levels.len()).rev() {
            if levels[i][p] < q {
                ans += 1 << i;
                p = levels[i][p];
            }
        }
        out_line!(ans);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
