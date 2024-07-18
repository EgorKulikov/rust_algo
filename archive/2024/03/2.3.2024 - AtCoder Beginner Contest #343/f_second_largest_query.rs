//{"name":"F - Second Largest Query","group":"AtCoder - AtCoder Beginner Contest 343","url":"https://atcoder.jp/contests/abc343/tasks/abc343_f","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n3 3 1 4 5\n2 1 3\n2 5 5\n1 3 3\n2 2 4\n","output":"1\n0\n2\n"},{"input":"1 1\n1000000000\n2 1 1\n","output":"0\n"},{"input":"8 9\n2 4 4 3 9 1 1 2\n1 5 4\n2 7 7\n2 2 6\n1 4 4\n2 2 5\n2 2 7\n1 1 1\n1 8 1\n2 1 8\n","output":"0\n1\n0\n2\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSecondLargestQuery"}}}

use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_int_vec(n);

    #[derive(Clone, Default)]
    struct Node {
        max: i32,
        max_qty: i32,
        second_max: i32,
        second_max_qty: i32,
    }

    impl Node {
        fn single(val: i32) -> Self {
            Self {
                max: val,
                max_qty: 1,
                second_max: 0,
                second_max_qty: 0,
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Default::default()
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
            self.max_qty = if left_val.max > right_val.max {
                self.second_max = right_val.max.max(left_val.second_max);
                self.second_max_qty = if right_val.max > left_val.second_max {
                    right_val.max_qty
                } else if right_val.max < left_val.second_max {
                    left_val.second_max_qty
                } else {
                    left_val.second_max_qty + right_val.max_qty
                };
                left_val.max_qty
            } else if right_val.max > left_val.max {
                self.second_max = left_val.max.max(right_val.second_max);
                self.second_max_qty = if left_val.max > right_val.second_max {
                    left_val.max_qty
                } else if left_val.max < right_val.second_max {
                    right_val.second_max_qty
                } else {
                    left_val.max_qty + right_val.second_max_qty
                };
                right_val.max_qty
            } else {
                self.second_max = left_val.second_max.max(right_val.second_max);
                self.second_max_qty = if left_val.second_max > right_val.second_max {
                    left_val.second_max_qty
                } else if left_val.second_max < right_val.second_max {
                    right_val.second_max_qty
                } else {
                    left_val.second_max_qty + right_val.second_max_qty
                };
                left_val.max_qty + right_val.max_qty
            };
        }

        fn accumulate(&mut self, _value: &Self, _left: usize, _right: usize) {}

        fn reset_delta(&mut self, _left: usize, _right: usize) {}
    }

    impl Pushable<&i32> for Node {
        fn push(&mut self, value: &i32, _left: usize, _right: usize) {
            *self = Self::single(*value);
        }
    }

    let mut st = SegmentTree::from_generator(n, |i| Node::single(a[i]));
    for _ in 0..q {
        let t = input.read_size();

        match t {
            1 => {
                let p = input.read_size() - 1;
                let x = input.read_int();
                st.update(p..=p, &x);
            }
            2 => {
                let l = input.read_size() - 1;
                let r = input.read_size() - 1;
                let node = st.query(l..=r);
                out.print_line(node.second_max_qty);
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
