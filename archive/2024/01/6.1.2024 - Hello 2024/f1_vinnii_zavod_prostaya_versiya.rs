//{"name":"F1. Винный завод (простая версия)","group":"Codeforces - Hello 2024","url":"https://codeforces.com/contest/1919/problem/F1","interactive":false,"timeLimit":5000,"tests":[{"input":"4 3\n3 3 3 3\n1 4 2 8\n1000000000000000000 1000000000000000000 1000000000000000000\n4 3 8 1000000000000000000\n2 5 1 1000000000000000000\n3 0 0 1000000000000000000\n","output":"12\n12\n10\n"},{"input":"5 5\n10 3 8 9 2\n3 4 10 8 1\n1000000000000000000 1000000000000000000 1000000000000000000 1000000000000000000\n5 4 9 1000000000000000000\n1 1 1 1000000000000000000\n2 7 4 1000000000000000000\n4 1 1 1000000000000000000\n1 8 3 1000000000000000000\n","output":"34\n25\n29\n21\n27\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F1VinniiZavodProstayaVersiya"}}}

use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);
    let mut c = input.read_long_vec(n - 1);
    c.push(0);

    #[derive(Copy, Clone)]
    struct Node {
        ans: i64,
        will_pass: i64,
        plus: i64,
        add_pass: i64,
    }

    impl Node {
        fn new(a: i64, b: i64, c: i64) -> Self {
            Self {
                ans: a.min(b),
                will_pass: (a - b).max(0).min(c),
                plus: (b - a).max(0),
                add_pass: c - (a - b).max(0).min(c),
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self {
                ans: 0,
                will_pass: 0,
                plus: 0,
                add_pass: 0,
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
            let add_ans = left_val.will_pass.min(right_val.plus);
            self.ans = left_val.ans + right_val.ans + add_ans;
            let add_will_pass = (left_val.will_pass - add_ans).min(right_val.add_pass);
            self.will_pass = add_will_pass + right_val.will_pass;
            let redirect_to_plus = (right_val.plus - add_ans).min(left_val.add_pass);
            self.plus = left_val.plus + redirect_to_plus;
            self.add_pass =
                (left_val.add_pass - redirect_to_plus).min(right_val.add_pass - add_will_pass);
        }

        fn accumulate(&mut self, _value: &Self, _left: usize, _right: usize) {}

        fn reset_delta(&mut self, _left: usize, _right: usize) {}
    }

    impl Pushable<Node> for Node {
        fn push(&mut self, delta: Node, _left: usize, _right: usize) {
            *self = delta;
        }
    }

    let mut st = SegmentTree::from_generator(n, |i| Node::new(a[i], b[i], c[i]));
    for _ in 0..q {
        let p = input.read_size() - 1;
        let x = input.read_long();
        let y = input.read_long();
        let z = input.read_long();
        st.point_update(p, Node::new(x, y, z));
        out.print_line(st.query(..).ans);
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
