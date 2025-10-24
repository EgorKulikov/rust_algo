//{"name":"E. Predicting Popularity","group":"Codeforces - Educational Codeforces Round 183 (Rated for Div. 2)","url":"https://codeforces.com/contest/2145/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"20 25\n4\n1 22 1 30\n1 22 50 30\n5\n3 1 25\n2 23 22\n4 10 27\n1 21 21\n3 20 26\n","output":"3\n2\n4\n4\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::Tree;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let ac = input.read_int();
    let dr = input.read_int();
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let d = input.read_int_vec(n);

    #[derive(Default)]
    struct Node {
        val: i32,
        delta: i32,
        min: i32,
        size: usize,
        key: (i32, usize),
    }

    impl Node {
        fn new(val: i32, pos: usize, id: usize) -> Self {
            let adjusted = (pos as i32) - val;
            Self {
                val: adjusted,
                delta: 0,
                min: adjusted,
                size: 1,
                key: (val, id),
            }
        }
    }

    impl Payload for Node {
        const NEED_UPDATE: bool = true;
        const NEED_ACCUMULATE: bool = true;

        fn need_push_down(&self) -> bool {
            self.delta != 0
        }

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.min = self.val;
            if let Some(left) = left {
                self.min.minim(left.min);
            }
            if let Some(right) = right {
                self.min.minim(right.min);
            }
            self.size = 1 + left.map_or(0, |l| l.size) + right.map_or(0, |r| r.size);
        }

        fn accumulate(&mut self, delta: &Self) {
            self.val += delta.delta;
            self.delta += delta.delta;
            self.min += delta.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }

    impl OrdPayload for Node {
        type Key = (i32, usize);

        fn key(&self) -> &Self::Key {
            &self.key
        }
    }

    let order = Vec::with_gen(n, |i| ((a[i] - ac).max(0) + (d[i] - dr).max(0), i)).sorted();
    let mut pos = vec![0; n];
    for i in 0..n {
        pos[order[i].1] = i;
    }
    let mut tree = Tree::with_gen(n, |i| Node::new(order[i].0, i, order[i].1));
    let mut refs = tree.refs();

    let q = input.read_size();

    for _ in 0..q {
        let k = input.read_size() - 1;
        let na = input.read_int();
        let nd = input.read_int();

        // let p = tree.index_ref(&refs[pos[k]]);
        tree.raise(&refs[pos[k]]);
        tree.detach();
        tree.push_right(&Node {
            delta: -1,
            ..Default::default()
        });
        let new_val = (na - ac).max(0) + (nd - dr).max(0);
        let p = tree.range(..&(new_val, k)).size();
        refs[pos[k]] = tree.insert_with_id(Node::new(new_val, p, k)).1;
        tree.push_right(&Node {
            delta: 1,
            ..Default::default()
        });
        if tree.payload().unwrap().min >= 0 {
            out.print_line(n);
        } else {
            let mut ans = 0;
            tree.binary_search(|node, left, _| {
                if let Some(left) = left {
                    if left.min < 0 {
                        return Some(Direction::Left);
                    }
                    ans += left.size;
                }
                if node.val < 0 {
                    None
                } else {
                    ans += 1;
                    Some(Direction::Right)
                }
            });
            out.print_line(ans);
        }
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
