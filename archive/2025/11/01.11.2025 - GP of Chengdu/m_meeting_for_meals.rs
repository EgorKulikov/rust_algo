//{"name":"M. Meeting for Meals","group":"Universal Cup - GP of Chengdu","url":"https://contest.ucup.ac/contest/2567/problem/14718","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 3 2\n3 4\n1 2 3\n3 2 1\n4 2 1\n2 1 2\n1 2\n1 2 3\n3 2 3\n1 2 3\n1 2 3\n1 3 5\n","output":"3.0 3.0\n1.5 1.5\n3.5 3.5 2.5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::real::{IntoReal, Real};
use std::collections::BinaryHeap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(k).dec();
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();

    let mut graph = Graph::new(n);
    for (u, v, w) in edges {
        graph.add_edge(BiWeightedEdge::new(u, v, w));
    }
    let dist = graph
        .distances_from(0)
        .iter_map(|x| x.unwrap().0)
        .collect::<Vec<_>>();
    let end_time = a.copy_map(|x| dist[x]).max().unwrap();

    let mut enter: Vec<Option<usize>> = vec![None; n];
    let mut d = vec![i64::MAX; n];
    let mut heap = BinaryHeap::new();
    for (i, j) in a.copy_enumerate() {
        heap.push((0, j, i));
        d[j] = 0;
    }
    let mut ans = vec![Real::zero(); k];
    while let Some((cur_d, vert, id)) = heap.pop() {
        if -cur_d + dist[vert] > end_time {
            continue;
        }
        if let Some(e_id) = enter[vert] {
            if e_id == id {
                continue;
            }
            let res = end_time.into_real() - (d[vert].into_real() - cur_d) / 2;
            ans[id].maxim(res);
            ans[e_id].maxim(res);
            continue;
        }
        enter[vert] = Some(id);
        d[vert] = -cur_d;
        for e in &graph[vert] {
            let next = e.to();
            let nd = -cur_d + e.weight();
            heap.push((-nd, next, id));
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    output.set_precision(1);

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
