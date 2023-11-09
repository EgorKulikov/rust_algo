//{"name":"D. Robot dance","group":"Yandex - Yandex Cup 2023 — Algorithm — Semifinal","url":"https://contest.yandex.com/contest/54740/problems/D/","interactive":false,"timeLimit":2500,"tests":[{"input":"5\n1 2 0 4 3\n8\n0 0\n1 1\n2 2\n0 2\n3 3\n3 4\n4 4\n2 3\n","output":"3\n3\n3\n3\n0\n0\n0\n5\n"},{"input":"2\n0 1\n3\n0 0\n0 1\n1 1\n","output":"1\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRobotDance"}}}

use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n);
    let q = input.read_size();
    let queries = input.read_size_pair_vec(q);

    let mut id = vec![n; n];
    let mut pos_id = Vec::new();
    for i in 0..n {
        if id[i] != n {
            continue;
        }
        pos_id.push(i);
        let mut j = i;
        loop {
            id[j] = i;
            j = p[j];
            if j == i {
                break;
            }
        }
    }

    #[derive(Clone)]
    struct Node(Option<usize>);

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self(None)
        }

        fn join(
            &mut self,
            left_val: &Self,
            right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
            if let Some(&left) = left_val.0.as_ref() {
                if let Some(&right) = &right_val.0.as_ref() {
                    self.0 = Some(left.min(right));
                } else {
                    self.0 = Some(left);
                }
            } else {
                *self = right_val.clone();
            }
        }

        fn accumulate(&mut self, _value: &Self, _left: usize, _right: usize) {}

        fn reset_delta(&mut self, _left: usize, _right: usize) {}
    }

    impl Pushable<Node> for Node {
        fn push(&mut self, delta: Node, _left: usize, _right: usize) {
            *self = delta;
        }
    }

    let mut last = vec![None; n];
    let mut tree = SegmentTree::<Node>::new(n + pos_id.len());
    for (i, &pos) in pos_id.iter().enumerate() {
        tree.point_update(n + i, Node(Some(pos)));
        last[pos] = Some(n + i);
    }
    let mut q_at = vec![Vec::new(); n];
    for (i, &(l, _)) in queries.iter().enumerate() {
        q_at[l].push(i);
    }
    let mut ans = vec![0; q];
    for i in (0..n).rev() {
        if let Some(pos) = last[id[i]] {
            tree.point_update(pos, Node(None));
        }
        tree.point_update(i, Node(Some(id[i])));
        last[id[i]] = Some(i);
        for &j in &q_at[i] {
            let node = tree.query(queries[j].1 + 1..);
            ans[j] = node.0.unwrap_or(n);
        }
    }
    out.print_per_line(&ans);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
