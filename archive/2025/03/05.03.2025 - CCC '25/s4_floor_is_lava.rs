//{"name":"S4 - Floor is Lava","group":"DMOJ - CCC '25","url":"https://dmoj.ca/problem/ccc25s4","interactive":false,"timeLimit":4000,"tests":[{"input":"5 7\n1 2 3\n2 3 2\n1 3 6\n3 4 3\n4 5 7\n2 4 1\n2 5 10\n","output":"9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::id::Id;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();

    let mut id = Id::new();
    id.get((0, 0));
    let mut vertices = vec![Vec::new(); n];
    vertices[0].push(0);
    for (u, v, c) in edges.copy_iter() {
        id.get((u, c));
        id.get((v, c));
        vertices[u].push(c);
        vertices[v].push(c);
    }
    for i in 0..n {
        vertices[i].sort();
        vertices[i].dedup();
    }
    let mut graph = Graph::new(id.len());
    for i in 0..n {
        for (x, y) in vertices[i].consecutive_iter_copy() {
            graph.add_edge(BiWeightedEdge::new(id.get((i, x)), id.get((i, y)), y - x));
        }
    }
    for (u, v, c) in edges {
        graph.add_edge(BiWeightedEdge::new(id.get((u, c)), id.get((v, c)), 0));
    }
    let dist = graph.distances_from(id.get((0, 0)));
    let mut ans = None;
    for c in vertices[n - 1].copy_iter() {
        ans.minim(dist[id.get((n - 1, c))].unwrap().0);
    }
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
