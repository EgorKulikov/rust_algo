//{"name":"Min-Max Product Partition","group":"CodeChef - START173A","url":"https://www.codechef.com/START173A/problems/MNMXPRPAR","interactive":false,"timeLimit":7000,"tests":[{"input":"2\n3 3\n1 2 1\n2 1\n3 2\n1 3\n4 5\n3 2 4 1\n3 5\n1 9\n2 7\n1 8\n2 11\n","output":"10\n6\n10\n22\n78\n92\n196\n346\n318\n436\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::Tree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut a = input.read_unsigned_vec(n);

    type Mod = ModIntF;
    struct Node {
        val: Mod,
        to_right: Mod,
        to_left: Mod,
        len_p2: Mod,
        ans: Mod,
        min: Mod,
        max: Mod,
        key: (u32, usize),
    }
    impl Node {
        fn new(val: u32, idx: usize) -> Self {
            Self {
                val: Mod::new(val),
                to_right: Mod::new(val),
                to_left: Mod::new(val),
                len_p2: Mod::new(2),
                min: Mod::new(val),
                max: Mod::new(val),
                ans: Mod::zero(),
                key: (val, idx),
            }
        }
    }
    impl Payload for Node {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.to_right = self.val;
            self.to_left = self.val;
            self.ans = Mod::zero();
            self.len_p2 = Mod::new(2);
            self.min = self.val;
            self.max = self.val;
            if let Some(left) = left {
                self.ans += left.ans;
                self.ans += left.to_right * self.to_left;
                self.ans += self.val * left.max;
                self.min = left.min;
                self.to_left *= left.len_p2;
                self.to_left += left.to_left;
                self.to_right += left.to_right * self.len_p2;
                self.len_p2 *= left.len_p2;
            }
            if let Some(right) = right {
                self.ans += right.ans;
                self.ans += right.to_left * self.to_right;
                self.ans += self.val * right.min;
                self.max = right.max;
                self.to_right *= right.len_p2;
                self.to_right += right.to_right;
                self.to_left += right.to_left * self.len_p2;
                self.len_p2 *= right.len_p2;
            }
        }
    }
    impl OrdPayload for Node {
        type Key = (u32, usize);

        fn key(&self) -> &Self::Key {
            &self.key
        }
    }
    let mut treap = Tree::new();
    for i in 0..n {
        treap.insert(Node::new(a[i], i));
    }
    out.print_line(treap.payload().unwrap().ans);
    for _ in 0..q {
        let id = input.read_size() - 1;
        let v = input.read_unsigned();
        treap.remove(&(a[id], id));
        a[id] = v;
        treap.insert(Node::new(v, id));
        treap.rebuild();
        out.print_line(treap.payload().unwrap().ans);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
