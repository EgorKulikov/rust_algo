//{"name":"G. Most Expensive Item","group":"Codeforces - Coding Challenge Alpha V - by AlgoRave","url":"https://codeforces.com/gym/105005/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n2 7 1 8 2 8\n15\n3 1\n3 3\n3 4\n1 1 5 4\n3 1\n3 3\n3 4\n1 3 6 9\n3 1\n3 3\n3 4\n2 4\n3 1\n3 3\n3 4\n","output":"2\n1\n8\n4\n4\n8\n4\n9\n9\n2\n9\n9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GMostExpensiveItem"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::default_map::default_tree_map::DefaultTreeMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{
    PointOperationClosure, Pushable, SegmentTree, SegmentTreeNode,
};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use std::cell::Cell;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    #[derive(Default)]
    struct Node {
        set: DefaultTreeMap<i32, i32>,
    }

    impl Node {
        fn single(x: i32) -> Self {
            let mut set = DefaultTreeMap::new();
            set[x] += 1;
            Node { set }
        }
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Node::default()
        }

        fn join(
            &mut self,
            _left_val: &Self,
            _right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
        }

        fn accumulate(&mut self, _value: &Self, _left: usize, _right: usize) {}

        fn reset_delta(&mut self, _left: usize, _right: usize) {}
    }

    impl Pushable<&i32> for Node {
        fn push(&mut self, delta: &i32, _left: usize, _right: usize) {
            if *delta > 0 {
                self.set[*delta] += 1;
            } else {
                let delta = -delta;
                self.set[delta] -= 1;
                if self.set[delta] == 0 {
                    self.set.remove(&delta);
                }
            }
        }
    }

    let mut st = SegmentTree::from_generator(n, |i| Node::single(a[i]));
    let mut op = DefaultHashMap::new();

    let q = input.read_size();
    for i in 1..=q {
        let tp = input.read_size();
        match tp {
            1 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let x = input.read_int();
                op[i] = (l, r, x);
                st.update(l..r, &x);
            }
            2 => {
                let id = input.read_size();
                let (l, r, x) = op[id];
                st.update(l..r, &-x);
            }
            3 => {
                let at = input.read_size() - 1;
                let ans: Cell<Option<i32>> = Cell::new(None);
                let mut point_op = PointOperationClosure::new(
                    |node: &mut Node, _, ans: &Cell<Option<i32>>| {
                        let mut answ = ans.get();
                        answ.maxim(node.set.iter().next_back().map(|(x, _)| *x));
                        ans.set(answ);
                    },
                    |node, _, _, ans, _, mid, _| {
                        let mut answ = ans.get();
                        answ.maxim(node.set.iter().next_back().map(|(x, _)| *x));
                        ans.set(answ);
                        if at < mid {
                            Direction::Left
                        } else {
                            Direction::Right
                        }
                    },
                );
                st.point_operation(&mut point_op, &ans);
                out.print_line(ans.get());
            }
            _ => unreachable!(),
        }
    }
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
                solve(&mut input, &mut output, i + 1, &pre_calc);
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
    //    tester::stress_test();
}
//END MAIN
