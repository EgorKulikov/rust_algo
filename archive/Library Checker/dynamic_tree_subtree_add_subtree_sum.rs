//{"name":"Dynamic Tree Subtree Add Subtree Sum","group":"Library Checker","url":"https://judge.yosupo.jp/problem/dynamic_tree_subtree_add_subtree_sum","interactive":false,"timeLimit":5000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::euler_tour_tree::EulerTourForest;
use algo_lib::collections::payload::Payload;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(n - 1);

    #[derive(Default)]
    struct Node {
        val: i64,
        sum: i64,
        delta: i64,
        size: i64,
        self_size: i64,
    }
    impl Payload for Node {
        const NEED_UPDATE: bool = true;
        const NEED_ACCUMULATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            assert!(self.delta == 0);
            self.size = self.self_size + left.map_or(0, |l| l.size) + right.map_or(0, |r| r.size);
            self.sum = self.val + left.map_or(0, |l| l.sum) + right.map_or(0, |r| r.sum);
        }

        fn accumulate(&mut self, delta: &Self) {
            self.val += delta.delta * self.self_size;
            self.sum += delta.delta * self.size;
            self.delta += delta.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }
    let mut etf = EulerTourForest::new();
    for i in 0..n {
        etf.add_node(Node {
            val: a[i],
            sum: a[i],
            size: 1,
            self_size: 1,
            ..Node::default()
        });
    }
    for (u, v) in edges {
        etf.add_edge(u, v, Node::default(), Node::default());
    }

    for _ in 0..q {
        let t = input.read_int();
        match t {
            0 => {
                let u = input.read_size();
                let v = input.read_size();
                let w = input.read_size();
                let x = input.read_size();
                etf.remove_edge(u, v);
                etf.add_edge(w, x, Node::default(), Node::default());
            }
            1 => {
                let v = input.read_size();
                let p = input.read_size();
                let x = input.read_long();
                etf.with_subtree_mut(v, p, |node| {
                    node.accumulate(&Node {
                        delta: x,
                        ..Node::default()
                    });
                });
            }
            2 => {
                let v = input.read_size();
                let p = input.read_size();
                out.print_line(etf.with_subtree(v, p, |node| node.sum));
            }
            _ => unreachable!(),
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
