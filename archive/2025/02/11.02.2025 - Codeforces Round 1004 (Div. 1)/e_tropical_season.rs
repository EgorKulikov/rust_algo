//{"name":"E. Tropical Season","group":"Codeforces - Codeforces Round 1004 (Div. 1)","url":"https://codeforces.com/contest/2066/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"4 7\n2 2 4 11\n- 2\n+ 4\n+ 30\n+ 40\n- 4\n+ 2\n+ 2\n","output":"Yes\nNo\nYes\nNo\nYes\nNo\nNo\nYes\n"},{"input":"6 7\n5000 1000 400 400 100 99\n+ 1\n- 5000\n- 1\n- 400\n- 400\n- 100\n- 99\n","output":"No\nYes\nYes\nYes\nNo\nNo\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::by_index;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::Tree;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n).sorted();

    struct Node {
        val: i64,
        sum: i64,
        min: i64,
        max: i64,
        min_val: i64,
        min_delta: i64,
        min_delta_val: i64,
        key: (i64, usize),
    }

    impl Node {
        fn new(val: i64, id: usize) -> Self {
            Self {
                val,
                sum: val,
                min: val,
                max: val,
                min_val: val,
                min_delta: val,
                min_delta_val: val,
                key: (val, id),
            }
        }
    }

    impl Payload for Node {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum = self.val + left.map_or(0, |x| x.sum) + right.map_or(0, |x| x.sum);
            self.min = left.map_or(self.val, |x| x.min);
            self.min_val = left.map_or(self.val, |x| x.min_val);
            self.max = right.map_or(self.val, |x| x.max);
            self.min_delta = self.val;
            self.min_delta_val = self.val;
            if let Some(left) = left {
                if self.min_delta.minim(left.min_delta) {
                    self.min_delta_val = left.min_delta_val;
                }
                let cand = self.val - left.max;
                if self.min_delta.minim(cand) {
                    self.min_delta_val = self.val;
                }
            }
            if let Some(right) = right {
                if self.min_delta.minim(right.min_delta) {
                    self.min_delta_val = right.min_delta_val;
                }
                let cand = right.min - self.val;
                if self.min_delta.minim(cand) {
                    self.min_delta_val = right.min_val;
                }
            }
        }
    }
    impl OrdPayload for Node {
        type Key = (i64, usize);

        fn key(&self) -> &Self::Key {
            &self.key
        }
    }

    let mut tree = Tree::with_gen(n, |i| Node::new(a[i], i));
    fn get_ans(tree: &mut Tree<Node>) -> bool {
        let mut left = Tree::<Node>::new();
        while tree.size() > 1 {
            let sum = left.payload().map_or(0, |x| x.sum);
            if tree.payload().unwrap().min <= sum {
                let head = tree.range(..=&(sum, usize::MAX)).detach();
                assert!(!head.is_empty());
                left.push_back(head);
                continue;
            }
            if sum < tree.payload().unwrap().min_delta {
                tree.push_front(left);
                return false;
            }
            let val = tree.payload().unwrap().min_delta_val;
            let head = tree.range(..=&(val, usize::MAX)).detach();
            assert!(!head.is_empty());
            left.push_back(head);
        }
        tree.push_front(left);
        true
    }

    let mut by_id = by_index(&a);

    out.set_bool_output(BoolOutput::YesNo);
    out.print_line(get_ans(&mut tree));

    for i in 0..q {
        let command = input.read_char();
        let val = input.read_long();
        match command {
            b'+' => {
                by_id[val].push(i + n);
                tree.insert(Node::new(val, i + n));
            }
            b'-' => {
                let id = by_id[val].pop().unwrap();
                tree.remove(&(val, id));
            }
            _ => unreachable!(),
        }
        out.print_line(get_ans(&mut tree));
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
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
//END MAIN
