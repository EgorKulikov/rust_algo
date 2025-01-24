//{"name":"Dynamic Tree Vertex Add Subtree Sum","group":"Library Checker","url":"https://judge.yosupo.jp/problem/dynamic_tree_vertex_add_subtree_sum","interactive":false,"timeLimit":5000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

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
    }
    impl Payload for Node {
        const NEED_UPDATE: bool = true;
        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum = self.val + left.map_or(0, |l| l.sum) + right.map_or(0, |r| r.sum);
        }
    }
    let mut etf = EulerTourForest::new();
    for i in 0..n {
        etf.add_node(Node {
            val: a[i],
            sum: a[i],
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
                let p = input.read_size();
                let x = input.read_long();
                etf.with_node_mut(p, |node| {
                    node.val += x;
                    node.sum += x;
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
