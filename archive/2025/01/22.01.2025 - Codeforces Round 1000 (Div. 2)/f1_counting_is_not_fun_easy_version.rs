//{"name":"F1. Counting Is Not Fun (Easy Version)","group":"Codeforces - Codeforces Round 1000 (Div. 2)","url":"https://codeforces.com/contest/2063/problem/F1","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n3\n2 5\n1 6\n3 4\n4\n1 6\n7 8\n2 3\n4 5\n6\n2 3\n1 6\n7 8\n9 12\n10 11\n4 5\n","output":"5 1 1 1\n14 2 2 1 1\n132 42 5 2 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::treap::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::Tree;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::as_index::AsIndex;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let lr = input.read_size_pair_vec(n);

    struct Node {
        pos: usize,
        self_len: usize,
        sum_len: usize,
    }

    impl Payload for Node {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum_len =
                self.self_len + left.map_or(0, |x| x.sum_len) + right.map_or(0, |x| x.sum_len);
        }

        fn reset_delta(&mut self) {}
    }

    impl OrdPayload for Node {
        type Key = usize;

        fn key(&self) -> &Self::Key {
            &self.pos
        }
    }

    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(2 * n + 1);

    let catalan = Vec::with_gen(2 * n + 1, |i| {
        if i % 2 == 0 {
            c.c(i, i / 2) / Mod::from_index(i / 2 + 1)
        } else {
            Mod::zero()
        }
    });

    #[derive(Default, Clone)]
    struct STNode {
        parent: Option<(usize, usize)>,
    }

    impl SegmentTreeNode for STNode {
        fn accumulate(&mut self, value: &Self) {
            if let Some(delta) = value.parent {
                self.parent.minim(delta);
            }
        }
    }

    let mut st = SegmentTree::<STNode>::new(2 * n);
    let mut root: Tree<Node> = Tree::new();
    let mut inside: Vec<Tree<Node>> = Vec::with_capacity(n);
    let mut outer = 2 * n;
    let mut inner = Vec::new();
    let mut ans = catalan[2 * n];
    let mut res = Vec::new();
    res.push(catalan[2 * n]);
    for i in 0..n {
        let parent = st.point_query(lr[i].0).parent.map(|x| x.1);
        let tree = if let Some(parent) = parent {
            &mut inside[parent]
        } else {
            &mut root
        };
        let mut take = tree.range(&lr[i].0..&lr[i].1).detach();
        let len = take.payload().map_or(0, |x| x.sum_len);
        if let Some(parent) = parent {
            ans /= catalan[inner[parent]];
            inner[parent] += len;
            inner[parent] -= lr[i].1 - lr[i].0 + 1;
            ans *= catalan[inner[parent]];
            inside[parent].insert_with_id(Node {
                pos: lr[i].0,
                self_len: lr[i].1 - lr[i].0 + 1,
                sum_len: lr[i].1 - lr[i].0 + 1,
            });
        } else {
            ans /= catalan[outer];
            outer += len;
            outer -= lr[i].1 - lr[i].0 + 1;
            ans *= catalan[outer];
            root.insert_with_id(Node {
                pos: lr[i].0,
                self_len: lr[i].1 - lr[i].0 + 1,
                sum_len: lr[i].1 - lr[i].0 + 1,
            });
        }
        inside.push(take);
        inner.push(lr[i].1 - lr[i].0 - 1 - len);
        ans *= catalan[lr[i].1 - lr[i].0 - 1 - len];
        res.push(ans);
        st.update(
            lr[i].0..=lr[i].1,
            &STNode {
                parent: Some((lr[i].1 - lr[i].0, i)),
            },
        );
    }
    out.print_line(res);
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
