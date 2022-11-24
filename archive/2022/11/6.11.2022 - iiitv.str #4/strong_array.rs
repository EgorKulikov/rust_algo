//{"name":"Strong Array","group":"CodeChef - STRV2022","url":"https://www.codechef.com/STRV2022/problems/STRENGTH","interactive":false,"timeLimit":1000,"tests":[{"input":"1 2\n4\n1 1 8\n2 1 1\n","output":"8\n"},{"input":"6 5\n5 3 7 12 2 4\n2 3 5\n1 4 6\n2 3 6\n1 1 1\n2 1 1\n","output":"51\n39\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"StrongArray"}}}

use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let q = input.read_usize();
    let a = input.read_int_vec(n);

    #[derive(Copy, Clone, Default)]
    struct Node {
        sum: i32,
        max: i32,
        min: i32,
    }
    impl Node {
        fn new(x: i32) -> Self {
            Self {
                sum: x,
                max: x,
                min: x,
            }
        }
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
            self.sum = left_val.sum + right_val.sum;
            self.max = left_val.max.max(right_val.max);
            self.min = left_val.min.min(right_val.min);
        }

        fn accumulate(&mut self, _value: &Self, _left: usize, _right: usize) {}

        fn reset_delta(&mut self, _left: usize, _right: usize) {}
    }
    impl Pushable<i32> for Node {
        fn push(&mut self, delta: i32, _left: usize, _right: usize) {
            self.sum = delta;
            self.max = delta;
            self.min = delta;
        }
    }
    let mut tree = SegmentTree::from_generator(n, |i| Node::new(a[i]));

    for _ in 0..q {
        let t = input.read_usize();
        match t {
            1 => {
                let x = input.read_usize() - 1;
                let y = input.read_int();
                tree.point_update(x, y);
            }
            2 => {
                let l = input.read_usize() - 1;
                let r = input.read_usize();
                let ans = tree.query(l..r);
                out_line!(ans.sum + (ans.max - ans.min) * (r - l).into_i32());
            }
            _ => unreachable!(),
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
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
