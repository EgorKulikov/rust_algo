//{"name":"Similar Pair","group":"HackerRank - Algorithms - Search","url":"https://www.hackerrank.com/challenges/similarpair/problem?utm_campaign=challenge-recommendation&utm_medium=email&utm_source=24-hour-campaign","interactive":false,"timeLimit":4000,"tests":[{"input":"5 2\n3 2\n3 1\n1 4\n1 5\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SimilarPair"}}}

use algo_lib::collections::iter_ext::iters::Iters;
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
    let k = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    let root = n * (n - 1) / 2 - edges.iter_map(|(_, c)| c).sum::<usize>();
    struct Node(usize);
    impl Payload for Node {}
    impl OrdPayload for Node {
        type Key = usize;

        fn key(&self) -> &Self::Key {
            &self.0
        }
    }
    let mut ans = 0;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        let mut treap = Tree::new();
        treap.insert(Node(vert));
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            treap = Tree::union(f.call(e.to(), vert), treap);
        }
        ans += treap.range(&vert.saturating_sub(k)..=&(vert + k)).size() - 1;
        treap
    });
    dfs.call(root, n);
    out.print_line(ans);
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
