//{"name":"A Simple Task","group":"Eolymp - Basecamp - Educational Round #4","url":"https://eolymp.com/en/compete/btktopvnh51kpfku7ua54hi0bk/problem/9","interactive":false,"timeLimit":2000,"tests":[{"input":"10 5\nabacdabcda\n7 10 0\n5 8 1\n1 4 0\n3 6 0\n7 10 1\n","output":"cbcaaaabdd\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_str();

    #[derive(Copy, Clone, Eq, PartialEq)]
    enum Order {
        None,
        Ascending,
        Descending,
    }
    impl Default for Order {
        fn default() -> Self {
            Order::None
        }
    }
    #[derive(Default)]
    struct Node {
        qty: [usize; 26],
        from: usize,
        len: usize,
        order: Order,
    }
    impl SegmentTreeNode for Node {
        fn accumulate(&mut self, value: &Self) {
            if value.order == Order::None {
                return;
            }
            self.order = value.order;
            let mut rem = self.len;
            if (self.from == value.from) ^ (value.order == Order::Descending) {
                for i in 0..26 {
                    let cur = rem.min(value.qty[i]);
                    self.qty[i] = cur;
                    rem -= cur;
                }
            } else {
                for i in (0..26).rev() {
                    let cur = rem.min(value.qty[i]);
                    self.qty[i] = cur;
                    rem -= cur;
                }
            }
        }

        fn reset_delta(&mut self) {
            self.order = Order::None;
        }

        fn join(left: &Self, right: &Self) -> Self {
            let mut res = Self::default();
            res.len = left.len + right.len;
            res.from = left.from;
            res.update(left, right);
            res
        }

        fn update(&mut self, left_val: &Self, right_val: &Self) {
            for i in 0..26 {
                self.qty[i] = left_val.qty[i] + right_val.qty[i];
            }
        }
    }
    impl Node {
        fn new(pos: usize, c: u8) -> Self {
            let mut qty = [0; 26];
            qty[(c - b'a') as usize] = 1;
            Self {
                qty,
                from: pos,
                len: 1,
                order: Order::None,
            }
        }
    }
    let mut st = SegmentTree::with_gen(n, |pos| Node::new(pos, s[pos]));

    for _ in 0..q {
        let i = input.read_size() - 1;
        let j = input.read_size();
        let k = input.read_int();
        let mut q = [0; 26];
        st.for_each(i..j, |_, node| {
            for c in 0..26 {
                q[c] += node.qty[c];
            }
        });
        st.for_each_mut(i..j, |_, node| {
            let mut rem = node.len;
            if k == 1 {
                for c in 0..26 {
                    let cur = q[c].min(rem);
                    node.qty[c] = cur;
                    q[c] -= cur;
                    rem -= cur;
                }
                node.order = Order::Ascending;
            } else {
                for c in (0..26).rev() {
                    let cur = q[c].min(rem);
                    node.qty[c] = cur;
                    q[c] -= cur;
                    rem -= cur;
                }
                node.order = Order::Descending;
            }
        });
    }
    let mut ans = Str::new();
    for i in 0..n {
        let node = st.point_query(i);
        for c in 0..26 {
            for _ in 0..node.qty[c] {
                ans.push(b'a' + c as u8);
            }
        }
    }
    out.print_line(ans);
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
        TaskType::Classic | TaskType::RunTwice => input.is_empty(),
        TaskType::Interactive => true,
    }
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
