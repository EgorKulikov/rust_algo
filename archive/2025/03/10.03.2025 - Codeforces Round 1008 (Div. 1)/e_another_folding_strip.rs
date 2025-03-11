//{"name":"E. Another Folding Strip","group":"Codeforces - Codeforces Round 1008 (Div. 1)","url":"https://mirror.codeforces.com/contest/2077/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n0 1 0\n6\n1 0 0 1 2 1\n5\n2 1 2 4 3\n12\n76 55 12 32 11 45 9 63 88 83 32 6\n","output":"4\n28\n47\n7001\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::Tree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::replace_with::ReplaceWith;
use algo_lib::misc::test_type::TaskType;
use std::mem::swap;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    type Mod = ModIntF;
    struct Node {
        qty: Mod,
        self_qty: Mod,
        val: i64,
        sum: Mod,
    }

    impl Payload for Node {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.qty = self.self_qty;
            self.sum = Mod::new_wide(self.val) * self.qty;
            if let Some(left) = left {
                self.qty += left.qty;
                self.sum += left.sum;
            }
            if let Some(right) = right {
                self.qty += right.qty;
                self.sum += right.sum;
            }
        }
    }

    impl OrdPayload for Node {
        type Key = i64;

        fn key(&self) -> &Self::Key {
            &self.val
        }

        fn union(a: Self, b: Self) -> Self {
            Self {
                qty: a.qty + b.qty,
                self_qty: a.self_qty + b.self_qty,
                val: a.val,
                sum: a.sum + b.sum,
            }
        }
    }

    struct Half {
        tree: Tree<Node>,
        shift: i64,
    }

    impl Half {
        fn new() -> Self {
            Self {
                tree: Tree::new(),
                shift: 0,
            }
        }

        fn add_zero(&mut self) {
            let zero = Tree::single(Node {
                qty: Mod::one(),
                self_qty: Mod::one(),
                val: self.shift,
                sum: Mod::new_wide(self.shift),
            });
            self.tree.replace_with(|tree| Tree::union(tree, zero));
        }

        fn add_k(&mut self, k: i64) {
            self.shift -= k;
        }

        fn remove_k(&mut self, k: i64) {
            self.shift += k;
            let val = self.shift;
            let qty = self
                .tree
                .range(..=&val)
                .detach()
                .payload()
                .map_or(Mod::zero(), |node| node.qty);
            self.tree.replace_with(|tree| {
                Tree::union(
                    tree,
                    Tree::single(Node {
                        qty,
                        self_qty: qty,
                        val,
                        sum: qty * Mod::new_wide(val),
                    }),
                )
            });
        }

        fn sum(&mut self) -> Mod {
            if let Some(payload) = self.tree.payload() {
                payload.sum - Mod::new_wide(self.shift) * payload.qty
            } else {
                Mod::zero()
            }
        }
    }

    let mut cur = Half::new();
    let mut other = Half::new();
    let mut ans = Mod::zero();
    for a in a {
        cur.add_zero();
        other.add_zero();
        cur.add_k(a);
        other.remove_k(a);
        ans += cur.sum() + other.sum();
        swap(&mut cur, &mut other);
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
