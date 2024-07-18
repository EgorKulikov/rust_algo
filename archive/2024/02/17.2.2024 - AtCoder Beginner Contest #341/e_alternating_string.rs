//{"name":"E - Alternating String","group":"AtCoder - Toyota Programming Contest 2024#2（AtCoder Beginner Contest 341）","url":"https://atcoder.jp/contests/abc341/tasks/abc341_e","interactive":false,"timeLimit":3000,"tests":[{"input":"5 6\n10100\n2 1 3\n2 1 5\n1 1 4\n2 1 5\n1 3 3\n2 2 4\n","output":"Yes\nNo\nYes\nNo\n"},{"input":"1 2\n1\n1 1 1\n2 1 1\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EAlternatingString"}}}

use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_str();

    #[derive(Clone, Default)]
    struct Node {
        is_good: bool,
        left: bool,
        right: bool,
        flipped: bool,
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
            self.is_good =
                left_val.is_good && right_val.is_good && left_val.right != right_val.left;
            self.left = left_val.left;
            self.right = right_val.right;
        }

        fn accumulate(&mut self, value: &Self, _left: usize, _right: usize) {
            if value.flipped {
                self.left ^= true;
                self.right ^= true;
                self.flipped ^= true;
            }
        }

        fn reset_delta(&mut self, _left: usize, _right: usize) {
            self.flipped = false;
        }
    }

    impl Pushable<&()> for Node {
        fn push(&mut self, _delta: &(), _left: usize, _right: usize) {
            self.flipped ^= true;
            self.left ^= true;
            self.right ^= true;
        }
    }

    let mut st = SegmentTree::from_generator(n, |i| Node {
        is_good: true,
        left: s[i] == b'1',
        right: s[i] == b'1',
        flipped: false,
    });

    out.set_bool_output(BoolOutput::YesNo);

    for _ in 0..q {
        let tp = input.read_size();
        let l = input.read_size() - 1;
        let r = input.read_size();

        match tp {
            1 => {
                st.update(l..r, &());
            }
            2 => {
                out.print_line(st.query(l..r).is_good);
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
    //    tester::stress_test();
}
//END MAIN
