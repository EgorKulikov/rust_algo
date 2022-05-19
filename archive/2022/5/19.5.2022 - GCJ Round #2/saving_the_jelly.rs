//{"name":"Saving the Jelly","group":"Google Coding Competitions - Round 2 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/00000000008778ec/0000000000b158f8","interactive":false,"timeLimit":10000,"tests":[{"input":"4\n2\n-3 0\n-1 0\n3 0\n-2 -1\n-2 1\n1\n0 0\n1 1\n2 2\n3\n10 0\n-10 0\n0 0\n0 5\n-1 0\n5 0\n0 -5\n2\n3 4\n3 4\n5 7\n3 4\n5 7\n","output":"Case #1: POSSIBLE\n2 2\n1 3\nCase #2: IMPOSSIBLE\nCase #3: POSSIBLE\n3 2\n2 4\n1 3\nCase #4: POSSIBLE\n1 2\n2 3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SavingTheJelly"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::geometry::point::Point;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use std::collections::VecDeque;

use algo_lib::out_line;

fn solve(input: &mut Input, test_case: usize) {
    let n = input.read_usize();
    let pos: Vec<Point<i64>> = input.read_vec(n);
    let candies: Vec<Point<i64>> = input.read_vec(n + 1);

    let mut graph = Graph::new(2 * n + 2);
    for i in 0..n {
        graph.add_edge(2 * n, FlowEdge::new(i, 1));
        graph.add_edge(n + i, FlowEdge::new(2 * n + 1, 1));
    }
    for i in 0..n {
        for j in 0..n {
            if pos[i].square_dist(candies[j + 1]) <= pos[i].square_dist(candies[0]) {
                graph.add_edge(i, FlowEdge::new(n + j, 1));
            }
        }
    }
    if graph.max_flow(2 * n, 2 * n + 1) != n {
        out_line!(format!("Case #{}: IMPOSSIBLE", test_case));
        return;
    }
    let mut order = Vec::with_capacity(n);
    for i in 0..n {
        let mut ord = Vec::new();
        for j in 0..n {
            if pos[i].square_dist(candies[j + 1]) <= pos[i].square_dist(candies[0]) {
                ord.push(j);
            }
        }
        ord.sort_by_cached_key(|&j| pos[i].square_dist(candies[j + 1]));
        order.push(VecDeque::from(ord));
    }
    let mut used_pos = BitSet::new(n);
    let mut used_candies = BitSet::new(n);
    for _ in 0..n {
        for i in 0..n {
            if used_pos[i] {
                continue;
            }
            let to = {
                let mut res = None;
                for e in &mut graph[i] {
                    if e.flow() > 0 {
                        res = Some(e.to() - n);
                        e.p
                        break;
                    }
                }
                res.unwrap()
            };
            while let Some(&v) = order[i].front() {
                if used_candies[v] {
                    order[i].pop_front();
                } else {
                    break;
                }
            }
            if to == *order[i].front().unwrap() {
                used_pos.set(i, true);
                used_candies.set(to, true);
                break;
            }
            *graph[i][0].capacity_mut() += 1;
            *graph[2 * ()]
        }
    }
    out_line!(format!("Case #{}:", test_case));
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
    // input.skip_whitespace();
    // !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
