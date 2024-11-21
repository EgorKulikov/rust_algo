//{"name":"Quick Exit","group":"CodeChef - START161A","url":"https://www.codechef.com/START161A/problems/QUICKEXIT0","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 3\n1 2\n1 3\n3 1\n1 2\n1 3\n5 3\n1 4\n2 5\n3 4\n5 1\n6 2\n1 2\n1 3\n3 4\n1 6\n6 5\n","output":"2\n3\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"QuickExit"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size() - 1;
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    let mut sz = vec![0; n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        sz[vert] = 1;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            sz[vert] += f.call(e.to(), vert);
        }
        sz[vert]
    });
    dfs.call(0, n);
    let lca = graph.lca();
    let mut ans = 1;
    let mut sizes = Vec::new();
    let mut cur = n - 1;
    while cur != 0 {
        ans += 1;
        let p = lca.parent(cur).unwrap();
        for e in &graph[p] {
            if Some(e.to()) != lca.parent(p) && e.to() != cur {
                sizes.push(sz[e.to()]);
            }
        }
        cur = p;
    }
    sizes.sort();
    ans += sizes.copy_rev().skip(k).sum::<usize>();
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
