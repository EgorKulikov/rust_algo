//{"name":"E - Pairing Wizards","group":"AtCoder - AtCoder Regular Contest 142","url":"https://atcoder.jp/contests/arc142/tasks/arc142_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 5\n2 4\n3 3\n4 2\n5 1\n3\n1 4\n2 5\n3 5\n","output":"2\n"},{"input":"4\n1 1\n1 1\n1 1\n1 1\n3\n1 2\n2 3\n3 4\n","output":"0\n"},{"input":"9\n1 1\n2 4\n5 5\n7 10\n9 3\n9 13\n10 9\n3 9\n2 9\n7\n1 5\n2 5\n1 6\n2 4\n3 4\n4 9\n8 9\n","output":"22\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPairingWizards"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let mut wizards = input.read_int_pair_vec(n);
    let m = input.read_usize();
    let edges = input.read_usize_pair_vec(m).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let mut ans = 0;
    let mut special = BitSet::new(n);
    for i in 0..n {
        if graph[i].is_empty() {
            continue;
        }
        let (a, b) = wizards[i];
        if a >= b {
            continue;
        }
        let mut is_special = true;
        let mut min_power = 0;
        for e in &graph[i] {
            if wizards[e.to()].1 >= b {
                is_special = false;
                break;
            }
            min_power.maxim(wizards[e.to()].1);
        }
        if is_special {
            if a < min_power {
                ans += min_power - a;
                wizards[i].0 = min_power;
            }
            special.set(i, true);
        } else {
            ans += b - a;
            wizards[i].0 = b;
        }
    }
    let mut vert_count = 2;
    let mut add_edges = vec![Vec::new(); n];
    let mut edges = Vec::new();
    for i in 0..n {
        let (a, b) = wizards[i];
        if special[i] {
            edges.push((vert_count, 1, b - a));
            let cur = vert_count;
            vert_count += 1;
            for e in &graph[i] {
                edges.push((vert_count, cur, std::i32::MAX));
                add_edges[e.to()].push(((b - wizards[e.to()].0).max(0), vert_count));
                vert_count += 1;
            }
        }
    }
    let mut flow_graph = Graph::new(vert_count);
    for (f, t, cost) in edges {
        flow_graph.add_edge(f, FlowEdge::new(t, cost.into_i64()));
    }
    for i in 0..n {
        add_edges[i].sort_unstable();
        add_edges[i].reverse();
        let mut last = 0;
        for &(cost, to) in &add_edges[i] {
            flow_graph.add_edge(last, FlowEdge::new(to, cost.into_i64()));
            last = to;
        }
    }
    out_line!(ans + flow_graph.max_flow(0, 1).into_i32());
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
