//{"name":"Roy and Tree Permutation","group":"SeriousOJ - Happy New Year 2025","url":"https://judge.eluminatis-of-lu.com/contest/676ffd92569fb90008aac7da/1157","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n8\n1 5 2 1 1 3 5 6\n1 2\n1 3\n2 7\n2 8\n3 4\n4 5\n4 6\n5\n4 3 2 5 1\n","output":"1\n1\n3\n0\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"RoyAndTreePermutation"}}}

use algo_lib::collections::treap::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::Tree;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();
    let edges = input.read_size_pair_vec(n - 1).dec();

    struct Node(usize);
    impl Payload for Node {}
    impl OrdPayload for Node {
        type Key = usize;

        fn key(&self) -> &Self::Key {
            &self.0
        }

        fn union(a: Self, _b: Self) -> Self {
            a
        }
    }
    let mut ans = vec![0; n];
    let graph = Graph::from_biedges(n, &edges);
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (Tree<Node>, usize) {
        let mut treap = Tree::new();
        treap.insert(Node(a[vert]));
        let mut size = 1;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let (call, call_sz) = f.call(e.to(), vert);
            treap = Tree::union(treap, call);
            size += call_sz;
        }
        // eprint!("{}: ", vert);
        // for v in treap.iter() {
        //     eprint!("{} ", v.0);
        // }
        // eprintln!();
        ans[vert] = size - treap.range(..&size).size();
        (treap, size)
    });
    dfs.call(0, n);

    let q = input.read_size();
    for _ in 0..q {
        let x = input.read_size() - 1;
        out.print_line(ans[x]);
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
