//{"name":"G - Add and Multiply Queries","group":"AtCoder - Hitachi Vantara Programming Contest 2024（AtCoder Beginner Contest 368）","url":"https://atcoder.jp/contests/abc368/tasks/abc368_g","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 2 4\n1 2 2\n3\n3 1 3\n1 1 1\n3 1 3\n","output":"12\n7\n"},{"input":"6\n65 32 12 5 8 312\n4 1 3 15 16 2\n6\n3 2 6\n3 1 5\n1 5 6\n2 4 9\n3 2 6\n3 3 5\n","output":"46080\n69840\n27648\n1728\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GAddAndMultiplyQueries"}}}

use algo_lib::collections::segment_tree::{Pushable, QueryResult, SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::misc::value::Value;
use algo_lib::numbers::inf_int::InfInt;
use algo_lib::numbers::num_utils::UpperDiv;
use algo_lib::value;
use std::cell::Cell;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_long_vec(n);
    let mut b = input.read_long_vec(n);

    value!(Inf: i64 = 1_000_000_000_000_000_000);
    type Int = InfInt<i64, Inf>;
    #[derive(Default)]
    struct Node {
        changes: Vec<(Int, Int, Int)>,
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self::default()
        }

        fn join(
            &mut self,
            left_val: &Self,
            right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
            self.changes.clear();
            let mut at = 0;
            for (i, &(from, a, b)) in left_val.changes.iter().enumerate() {
                while at + 1 < right_val.changes.len()
                    && a * from + b >= right_val.changes[at + 1].0
                {
                    at += 1;
                }
                let end = if i + 1 < left_val.changes.len() {
                    left_val.changes[i + 1].0
                } else {
                    Int::new(Inf::val())
                };
                let mut from = from;
                while from < end
                    && at + 1 < right_val.changes.len()
                    && a * end + b > right_val.changes[at + 1].0
                {
                    let mut left = from;
                    let mut right = end;

                    while left < right {
                        let mid = Int::new((left.n + right.n) / 2);

                        if a * mid + b < right_val.changes[at + 1].0 {
                            left = mid + Int::new(1);
                        } else {
                            right = mid;
                        }
                    }
                    let (_, a1, b1) = right_val.changes[at];
                    self.changes.push((from, a * a1, b * a1 + b1));
                    from = left;
                    at += 1;
                }
                if from < end {
                    let (_, a1, b1) = right_val.changes[at];
                    self.changes.push((from, a * a1, b * a1 + b1));
                }
            }
        }

        fn accumulate(&mut self, _value: &Self, _left: usize, _right: usize) {}

        fn reset_delta(&mut self, _left: usize, _right: usize) {}
    }

    impl Pushable<(Int, Int)> for Node {
        fn push(&mut self, delta: (Int, Int), _left: usize, _right: usize) {
            self.changes.clear();
            let (b, a) = delta;
            if b != Int::new(0) {
                self.changes.push((Int::new(0), Int::new(1), b));
            }
            if a != Int::new(0) && a != Int::new(1) {
                self.changes
                    .push((Int::new(b.n.upper_div(a.n - 1)), a, Int::new(0)));
            }
            if self.changes.is_empty() {
                self.changes.push((Int::new(0), Int::new(1), Int::new(0)));
            }
        }
    }

    impl QueryResult<(), Cell<Int>> for Node {
        fn empty_result(_args: &Cell<Int>) -> () {}

        fn result(&self, args: &Cell<Int>) -> () {
            let val = args.get();
            let pos = self
                .changes
                .upper_bound(&(val, Int::new(Inf::val()), Int::new(Inf::val())))
                - 1;
            let (_, a, b) = self.changes[pos];
            args.set(a * val + b);
        }

        fn join_results(
            _left_res: (),
            _right_res: (),
            _args: &Cell<Int>,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) -> () {
        }
    }

    let mut st = SegmentTree::from_generator(n, |i| {
        let mut node = Node::default();
        node.push((Int::new(a[i]), Int::new(b[i])), 0, 0);
        node
    });

    let q = input.read_size();
    for _ in 0..q {
        let tp = input.read_int();
        match tp {
            1 => {
                let id = input.read_size() - 1;
                let x = input.read_long();
                a[id] = x;
                st.point_update(id, (Int::new(a[id]), Int::new(b[id])));
            }
            2 => {
                let id = input.read_size() - 1;
                let x = input.read_long();
                b[id] = x;
                st.point_update(id, (Int::new(a[id]), Int::new(b[id])));
            }
            3 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let cell = Cell::new(Int::new(0));
                st.query_with_args(l..r, &cell);
                out.print_line(cell.get());
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
