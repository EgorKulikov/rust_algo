//{"name":"D. Another Segment Problem","group":"Codeforces - GoForGold Long Challenge - August 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/629150/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3\n0 2\n1 5\n2 3\n4\n1 1\n2 2\n3 3\n4 4\n","output":"2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::Tree;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let segs = input.read_int_pair_vec(n).sorted();

    struct Node {
        val: i32,
        key: (i32, usize),
        max: i32,
        min: i32,
    }

    impl Node {
        fn new(left: i32, right: i32, id: usize) -> Self {
            Self {
                val: right - left,
                key: (right, id),
                max: right - left,
                min: right - left,
            }
        }
    }

    impl Payload for Node {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.max = self.val;
            self.min = self.val;
            if let Some(left) = left {
                self.max.maxim(left.max);
                self.min.minim(left.min);
            }
            if let Some(right) = right {
                self.max.maxim(right.max);
                self.min.minim(right.min);
            }
        }
    }

    impl OrdPayload for Node {
        type Key = (i32, usize);

        fn key(&self) -> &Self::Key {
            &self.key
        }
    }

    let mut tree: Tree<Node> = Tree::new();
    let mut ans = 0;
    for (i, (l, r)) in segs.iter_enumerate() {
        let good = tree.range(..=&(r, usize::MAX));
        if let Some(p) = good.payload() {
            ans.maxim(p.max - (r - l));
            ans.maxim((r - l) - p.min);
        }
        tree.insert(Node::new(l, r, i));
    }
    out.print_line(ans);
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
