//{"name":"E. Make Cycle in Tree","group":"SeriousOJ - Educational Round 1","url":"https://judge.eluminatis-of-lu.com/contest/686fe616d425270007014c27/1205","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5\n1 10 3 2 5\n1 2 3\n1 3 2\n3 4 4\n3 5 1\n7\n79 15 99 0 1 69 71\n7 1 1\n5 2 28\n4 5 13\n5 1 39\n3 7 16\n6 4 6\n","output":"7\n215\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let w = input.read_long_vec(n);
    let edges = input.read_vec::<(usize, usize, i64)>(n - 1).dec();

    let mut graph = Graph::new(n);
    for (u, v, c) in edges {
        graph.add_edge(BiWeightedEdge::new(u, v, c));
    }
    let mut ans = None;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (i64, i64) {
        let mut best = None;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let (direct, non_direct) = f.call(e.to(), vert);
            ans.maxim(non_direct - e.weight() * 2 + w[vert]);
            if let Some(best) = best {
                ans.maxim(best - e.weight() * 2 + direct.max(non_direct));
            }
            best.maxim(direct.max(non_direct) - e.weight() * 2 + w[vert]);
        }
        (w[vert], best.unwrap_or(i64::MIN / 2))
    });
    dfs.call(0, n);
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
