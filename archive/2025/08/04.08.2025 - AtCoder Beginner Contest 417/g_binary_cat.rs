//{"name":"G - Binary Cat","group":"AtCoder - AtCoder Beginner Contest 417","url":"https://atcoder.jp/contests/abc417/tasks/abc417_g","interactive":false,"timeLimit":6000,"tests":[{"input":"7\n0 1 1\n0 0 2\n1 1 1\n2 3 2\n2 4 3\n5 4 2\n6 7 6\n","output":"0\n0\n1\n1\n1\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::Tree;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
// use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let q = input.read_size();
    let lrx: Vec<(usize, usize, usize)> = input.read_vec(q);

    let mut len = vec![1usize, 1];
    for (l, r, _) in lrx.copy_iter() {
        len.push(len[l].saturating_add(len[r]));
    }
    struct Node {
        delta: usize,
        id: usize,
        key: (usize, usize),
    }
    impl Payload for Node {
        fn accumulate(&mut self, delta: &Self) {
            self.delta += delta.delta;
            self.key.0 -= delta.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }

        const NEED_ACCUMULATE: bool = true;
    }
    impl OrdPayload for Node {
        type Key = (usize, usize);

        fn key(&self) -> &Self::Key {
            &self.key
        }
    }
    let mut treaps = Vec::with_gen(q + 2, |_| Tree::new());
    for i in 0..q {
        treaps[i + 2].insert(Node {
            delta: 0,
            id: i,
            key: (lrx[i].2 - 1, i),
        });
    }
    for i in (0..q).rev() {
        let (l, r, _) = lrx[i];
        let (left, mut right) = treaps[i + 2].detach().split(&(len[l] - 1, usize::MAX));
        right.push(&Node {
            delta: len[l],
            id: 0,
            key: (0, 0),
        });
        let ll = Tree::union(treaps[l].detach(), left);
        treaps[l] = ll;
        let rr = Tree::union(treaps[r].detach(), right);
        treaps[r] = rr;
    }
    let mut ans = vec![0; q];
    for node in treaps[1].iter() {
        ans[node.id] = 1;
    }
    out.print_per_line(&ans);
    // let mut mem = Memoization2::new(|mem, i: usize, x: usize| {
    //     assert!(x < len[i]);
    //     if i == 0 {
    //         0
    //     } else if i == 1 {
    //         1
    //     } else if x >= len[lrx[i - 2].0] {
    //         mem.call(lrx[i - 2].1, x - len[lrx[i - 2].0])
    //     } else {
    //         mem.call(lrx[i - 2].0, x)
    //     }
    // });
    // for (i, (_, _, x)) in lrx.copy_enumerate() {
    //     out.print_line(mem.call(i + 2, x - 1));
    // }
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
