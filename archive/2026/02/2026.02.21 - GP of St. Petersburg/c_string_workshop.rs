//{"name":"C. String Workshop","group":"Universal Cup - GP of St. Petersburg","url":"https://contest.ucup.ac/contest/3384/problem/17163","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n5 20\nDBCAA\n2 1 5\n2 2 4\n1 3 Z\n2 1 3\n3 1 5\n2 1 5\n1 5 A\n3 2 4\n2 1 5\n1 1 C\n2 1 2\n3 4 5\n2 3 5\n1 2 B\n2 1 5\n3 1 1\n2 1 1\n1 4 D\n2 1 5\n3 1 5\n","output":"18\n3\n1\n16\n0\n0\n7\n1\n1\n1\n9\n0\n0\n3\n3\n"},{"input":"3\n3 5\nCBA\n2 1 3\n3 1 3\n2 1 3\n1 2 C\n2 1 2\n5 5\nABCDE\n2 2 5\n1 5 A\n2 1 5\n3 1 5\n2 1 5\n6 5\nYYYYYY\n2 1 6\n1 3 A\n2 1 6\n3 2 5\n2 1 6\n","output":"4\n4\n0\n0\n0\n9\n9\n0\n0\n3\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_str();

    #[derive(Default, Copy, Clone)]
    struct Letter {
        qty: i64,
        sum: i64,
        sum2: i64,
    }

    impl Letter {
        fn add(self, qty: i64) -> Self {
            Self {
                qty: self.qty,
                sum: self.sum + self.qty * qty,
                sum2: self.sum2 + self.sum * qty + self.qty * (qty * qty + qty) / 2,
            }
        }

        fn join(left: Self, right: Self) -> Self {
            Self {
                qty: left.qty + right.qty,
                sum: left.sum + right.sum,
                sum2: left.sum2 + right.sum2,
            }
        }
    }

    #[derive(Default, Copy, Clone)]
    struct Node {
        letters: [Letter; 26],
        len: i64,
        sorted: bool,
        left: usize,
    }

    fn sum(n: i64) -> i64 {
        n * (n - 1) / 2
    }

    fn sum2(n: i64) -> i64 {
        n * (n - 1) * (n + 1) / 6
    }

    impl Node {
        fn new(letter: u8, pos: usize) -> Self {
            let mut node = Node::default();
            node.len = 1;
            node.left = pos;
            let id = (letter - b'A') as usize;
            node.letters[id].qty = 1;
            // node.letters[id].sum = 1;
            // node.letters[id].sum2 = 1;
            node
        }

        fn sort(&mut self) {
            let mut total = 0;
            for i in 0..26 {
                self.letters[i].sum = sum(self.letters[i].qty);
                self.letters[i].sum2 = sum2(self.letters[i].qty);
                self.letters[i] = self.letters[i].add(total);
                total += self.letters[i].qty;
            }
            self.sorted = true;
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            let mut left = 0;
            self.len = left_val.len + right_val.len;
            self.left = left_val.left;
            for i in 0..26 {
                left += left_val.letters[i].qty;
                self.letters[i] = Letter::join(left_val.letters[i], right_val.letters[i].add(left));
            }
        }

        fn accumulate(&mut self, value: &Self) {
            if value.sorted {
                let mut rem = self.len;
                if self.left == value.left {
                    for i in 0..26 {
                        let cur = value.letters[i].qty.min(rem);
                        self.letters[i].qty = cur;
                        rem -= cur;
                    }
                } else {
                    for i in (0..26).rev() {
                        let cur = value.letters[i].qty.min(rem);
                        self.letters[i].qty = cur;
                        rem -= cur;
                    }
                }
                self.sort();
            }
        }

        fn reset_delta(&mut self) {
            self.sorted = false;
        }
    }

    let mut tree = SegmentTree::with_gen(n, |i| Node::new(s[i], i));

    for _ in 0..q {
        let t = input.read_int();
        match t {
            1 => {
                let pos = input.read_size() - 1;
                let c = input.read_char();
                tree.point_update(pos, Node::new(c, pos));
            }
            _ => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let mut node = Node::default();
                tree.for_each(l..r, |_, cur| {
                    node = Node::join(&node, cur);
                });
                let mut sorted = node;
                sorted.sort();
                let mut res = 0;
                for i in 0..26 {
                    res += sorted.letters[i].sum2 - node.letters[i].sum2;
                }
                out.print_line(res);
                if t == 3 {
                    tree.for_each_mut(l..r, |_, cur| {
                        let mut rem = cur.len;
                        for i in 0..26 {
                            let c = sorted.letters[i].qty.min(rem);
                            sorted.letters[i].qty -= c;
                            cur.letters[i].qty = c;
                            rem -= c;
                        }
                        cur.sort();
                    });
                }
            }
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
