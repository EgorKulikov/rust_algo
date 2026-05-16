//{"name":"D. For the Emperor!","group":"Yandex - Yandex Cup 2024 — Algorithm — Final","url":"https://contest.yandex.com/contest/72057/problems/D/","interactive":false,"timeLimit":1000,"tests":[{"input":"7 6\n2 1 0 1 2 3 4\n1 2\n1 3\n2 4\n2 5\n3 6\n3 7\n","output":"2\n"},{"input":"4 4\n1 1 1 1\n1 2\n1 3\n2 4\n3 4\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DForTheEmperor"}}}

use algo_lib::collections::bit_set::BitSet;
// use algo_lib::graph::min_cost_flow::MinCostFlow;
use algo_lib::collections::iter_ext::iters_ref::ItersRef;
// use algo_lib::collections::iter_ext::iters_ref::ItersRef;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::edges::weighted_flow_edge::WeightedFlowEdge;
use algo_lib::graph::flow_graph::FlowGraph;
use algo_lib::graph::strongly_connected_components::{
    StronglyConnectedComponents, StronglyConnectedComponentsTrait,
};
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::VecDeque;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    // let time = Instant::now();
    let n = input.read_size();
    //5
    let m = input.read_size();
    let a = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::from_edges(n, &edges);
    let StronglyConnectedComponents { color, condensed } = graph.strongly_connected_components();
    let mut b = vec![0; condensed.vertex_count()];
    for i in 0..n {
        b[color[i]] += a[i];
    }

    let mut enabled = BitSet::new(condensed.vertex_count());
    enabled.fill(true);
    for i in 0..condensed.vertex_count() {
        for e in &condensed[i] {
            enabled.unset(e.to());
        }
    }
    for i in 0..condensed.vertex_count() {
        if enabled[i] && b[i] == 0 {
            out.print_line(-1);
            return;
        }
    }
    let mut target = 0;
    for i in 0..condensed.vertex_count() {
        if b[i] == 0 {
            target -= 1001;
        } else {
            target -= 1;
        }
    }

    fn min_cost(
        graph: &mut Graph<WeightedFlowEdge<i64, i64, ()>>,
        source: usize,
        sink: usize,
    ) -> i64 {
        let mut dist = vec![i64::MIN; graph.vertex_count()];
        let mut prev = vec![(0, 0); graph.vertex_count()];
        let mut ans = 0;
        loop {
            dist.fill(i64::MAX);
            dist[source] = 0;
            let mut queue = VecDeque::new();
            queue.push_back(source);
            let mut done = BitSet::new(graph.vertex_count());
            while let Some(vert) = queue.pop_front() {
                done.set(vert);
                for (i, e) in graph[vert].iter().enumerate() {
                    if e.capacity() != 0 {
                        let cand = dist[vert] + e.weight();
                        if cand < dist[e.to()] {
                            dist[e.to()] = cand;
                            prev[e.to()] = (vert, i);
                            if done[e.to()] {
                                queue.push_front(e.to());
                            } else {
                                queue.push_back(e.to());
                            }
                        }
                    }
                }
            }
            if dist[sink] >= 0 {
                return ans;
            }
            let mut cur = sink;
            let mut cap = i64::MAX;
            while cur != source {
                let (from, i) = prev[cur];
                cap.minim(graph[from][i].capacity());
                cur = from;
            }
            ans += cap * dist[sink];
            cur = sink;
            while cur != source {
                let (from, i) = prev[cur];
                graph.push_flow(graph[from][i].push_flow(cap));
                cur = from;
            }
        }
    }

    loop {
        // if time.elapsed().as_millis() > 900 {
        //     out.print_line(-1);
        //     return;
        // }
        let mut graph = Graph::new(2 * condensed.vertex_count() + 2);
        let source = 2 * condensed.vertex_count();
        let sink = 2 * condensed.vertex_count() + 1;
        for i in 0..condensed.vertex_count() {
            if b[i] == 0 {
                graph.add_edge(WeightedFlowEdge::new(2 * i, 2 * i + 1, -1001, 1));
            } else {
                graph.add_edge(WeightedFlowEdge::new(2 * i, 2 * i + 1, -1, 1));
            }
            graph.add_edge(WeightedFlowEdge::new(2 * i, 2 * i + 1, 0, n as i64));
            if enabled[i] {
                graph.add_edge(WeightedFlowEdge::new(source, 2 * i, 0, b[i]));
            } else {
                graph.add_edge(WeightedFlowEdge::new(source, 2 * i + 1, 0, b[i]));
            }
            graph.add_edge(WeightedFlowEdge::new(2 * i + 1, sink, 0, (n * n) as i64));
            for e in &condensed[i] {
                graph.add_edge(WeightedFlowEdge::new(2 * i + 1, 2 * e.to(), 0, n as i64));
            }
        }
        // let CostAndFlow { cost, .. } = graph.min_cost_flow(source, sink);
        let cost = min_cost(&mut graph, source, sink);
        if cost == target {
            out.print_line(enabled.count_ones());
            return;
        }
        let mut is_bad = BitSet::new(condensed.vertex_count());
        for i in 0..condensed.vertex_count() {
            for e in &graph[2 * i] {
                if e.to() == 2 * i + 1 && e.weight() == -1 && e.flow(&graph) == 0 {
                    if b[i] == 0 {
                        out.print_line(-1);
                        return;
                    }
                    if enabled[i] {
                        panic!();
                    }
                    is_bad.set(i);
                }
            }
        }
        let mut reachable = BitSet::new(condensed.vertex_count());
        for i in 0..condensed.vertex_count() {
            if reachable[i] || is_bad[i] {
                for e in &condensed[i] {
                    reachable.set(e.to());
                }
            }
        }
        let mut found = false;
        for i in 0..condensed.vertex_count() {
            if is_bad[i] && !reachable[i] {
                enabled.set(i);
                found = true;
            }
        }
        if !found {
            out.print_line(-1);
            return;
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
