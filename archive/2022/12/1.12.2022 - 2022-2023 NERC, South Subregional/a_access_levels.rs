//{"name":"A. Access Levels","group":"Codeforces - 2022-2023 ICPC, NERC, Южный четвертьфинал (онлайн-трансляция, правила ICPC, командное соревнование)","url":"https://codeforces.com/contest/1765/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n01\n11\n10\n","output":"2\n1 2\n2 2\n1 2\n2 2\n2 1\n"},{"input":"2 3\n101\n100\n","output":"1\n1 1 1\n1 10 5\n8\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAccessLevels"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let access = input.read_char_table(n, m);

    let mut requirements: Vec<BitSet> = Vec::with_capacity(m);
    for i in 0..m {
        requirements.push(access.column(i).map(|&c| c == '1').collect_vec().into());
    }

    let mut graph = Graph::new(2 * m + 2);
    for i in 0..m {
        graph.add_edge(2 * m, FlowEdge::new(i, 1));
        graph.add_edge(m + i, FlowEdge::new(2 * m + 1, 1));
        for j in 0..m {
            if i != j
                && requirements[i].is_subset(&requirements[j])
                && (i < j || requirements[i] != requirements[j])
            {
                graph.add_edge(i, FlowEdge::new(m + j, 1));
            }
        }
    }

    let flow = graph.max_flow(2 * m, 2 * m + 1);
    let mut dsu = DSU::new(m);
    for i in 0..m {
        for e in &graph[i] {
            if e.flow(&graph) == 1 && e.to() < 2 * m {
                dsu.join(i, e.to() - m);
            }
        }
    }

    out_line!(dsu.set_count());
    assert!((flow as usize) + dsu.set_count() == m);
    let parts = dsu.parts();
    let mut colors = vec![0; m];
    for (i, part) in parts.iter().enumerate() {
        for &j in part {
            colors[j] = i + 1;
        }
    }
    out_line!(colors);
    let mut level = Arr2d::new(n, parts.len(), 1);
    for k in 0..n {
        for (i, part) in parts.iter().enumerate() {
            for &j in part {
                if requirements[j][k] {
                    level[(k, i)] += 1;
                }
            }
        }
    }
    let mut r = vec![m + 2; m];
    for i in 0..m {
        for j in 0..n {
            if requirements[i][j] {
                r[i].minim(level[(j, colors[i] - 1)]);
            }
        }
    }
    out_line!(r);
    out_line!(level);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
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
