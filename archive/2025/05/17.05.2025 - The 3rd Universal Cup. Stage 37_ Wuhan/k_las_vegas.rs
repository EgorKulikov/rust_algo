//{"name":"K. Las Vegas","group":"Universal Cup - The 3rd Universal Cup. Stage 37: Wuhan","url":"https://contest.ucup.ac/contest/2025/problem/10746","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 3\n3 3 2\n2 7 5\n3 5 2\n1 6 4\n3 4\n100 100 100 1\n0 0 0 1\n100 100 100 1\n1 4\n20 0 20 0\n","output":"4 0 6 0\n1 1 1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::edges::weighted_flow_edge::WeightedFlowEdge;
use algo_lib::graph::min_cost_flow::{CostAndFlow, MinCostFlow};
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_table(n, m);

    let mut winner = vec![None; n];
    let mut second_winner = vec![None; n];
    let mut win_cost = vec![0; n];
    let mut tie_cost = vec![0; n];
    let mut num_wins = vec![0; m];
    for i in 0..n {
        for j in 0..m {
            if a[(i, j)] == 0 {
                continue;
            }
            let times = a.row(i).filter(|&&x| x == a[(i, j)]).count();
            if times == 1 {
                if winner[i].is_none() || a[(i, j)] > a[(i, winner[i].unwrap())] {
                    second_winner[i] = winner[i];
                    winner[i] = Some(j);
                    tie_cost[i] = a[(i, j)];
                } else if second_winner[i].is_none()
                    || a[(i, j)] > a[(i, second_winner[i].unwrap())]
                {
                    second_winner[i] = Some(j);
                }
            }
        }
        if let Some(w) = winner[i] {
            num_wins[w] += 1;
        }
        for x in tie_cost[i] + 1.. {
            if a.row(i).filter(|&&y| y == x).count() == 0 {
                win_cost[i] = x;
                break;
            }
        }
    }
    let mut best = i64::MAX;
    let mut ans = vec![0; n];
    const X: i64 = 10_000_000_000_000;
    for w in 0..=n {
        let mut graph = Graph::new(m + n + 3);
        let source = m + 1;
        let sink = m + 2;
        for i in 0..m {
            graph.add_edge(WeightedFlowEdge::new(source, i, 0, num_wins[i] as i64));
            graph.add_edge(WeightedFlowEdge::new(i, sink, 0, w as i64));
            graph.add_edge(WeightedFlowEdge::new(i, sink, X, n as i64));
        }
        graph.add_edge(WeightedFlowEdge::new(m, sink, -X, w as i64));
        graph.add_edge(WeightedFlowEdge::new(m, sink, 0, n as i64));
        for i in 0..n {
            let from = winner[i].unwrap_or(source);
            let to = second_winner[i].unwrap_or(sink);
            graph.add_edge(WeightedFlowEdge::new(from, m + 3 + i, 0, 1));
            graph.add_edge(WeightedFlowEdge::new(m + 3 + i, to, tie_cost[i], 1));
            graph.add_edge(WeightedFlowEdge::new(m + 3 + i, m, win_cost[i], 1));
        }
        let CostAndFlow { cost, .. } = graph.min_cost_max_flow(source, sink);
        let cand = cost + (w as i64) * X;
        if best.minim(cand) {
            ans.fill(0);
            for i in 0..n {
                let to = second_winner[i].unwrap_or(sink);
                for e in &graph[m + 3 + i] {
                    if e.flow(&graph) == 1 {
                        if e.to() == to {
                            ans[i] = tie_cost[i];
                        } else if e.to() == m {
                            ans[i] = win_cost[i];
                        }
                    }
                }
            }
        }
    }
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
