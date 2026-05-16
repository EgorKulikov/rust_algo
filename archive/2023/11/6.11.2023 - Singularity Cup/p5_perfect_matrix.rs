//{"name":"P5 - Perfect Matrix","group":"DMOJ - Singularity Cup","url":"https://dmoj.ca/problem/scp5","interactive":false,"timeLimit":2000,"tests":[{"input":"2 2\n3 0\n0 3\n3 9\n9 3\n5 5\n1 6\n6 9\n0 9\n","output":"3 2\n3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P5PerfectMatrix"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::flow_with_demand::FlowWithDemand;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let l = input.read_long_table(n, m);
    let r = input.read_long_table(n, m);
    let a = input.read_long_table(n, 2);
    let b = input.read_long_table(m, 2);

    /*let mut c = Arr2d::new(n + m + 4, n + m + 4, 0);
    for i in 0..n {
        for j in 0..m {
            c[(i, n + j)] = r[(i, j)] - l[(i, j)];
            c[(n + m + 2, n + j)] += l[(i, j)];
            c[(i, n + m + 3)] += l[(i, j)];
        }
    }
    for i in 0..n {
        c[(n + m, i)] = a[(i, 1)] - a[(i, 0)];
        c[(n + m + 2, i)] += a[(i, 0)];
        c[(n + m, n + m + 3)] += a[(i, 0)];
    }
    for j in 0..m {
        c[(j + n, n + m + 1)] = b[(j, 1)] - b[(j, 0)];
        c[(n + m + 2, n + m + 1)] += b[(j, 0)];
        c[(j + n, n + m + 3)] += b[(j, 0)];
    }
    c[(n + m + 1, n + m)] = i64::MAX / 2;
    let mut graph = Graph::new(n + m + 4);
    for i in 0..n + m + 4 {
        for j in 0..n + m + 4 {
            if c[(i, j)] != 0 {
                graph.add_edge(i, FlowEdge::new(j, c[(i, j)]));
            }
        }
    }
    let expected = c.row(n + m + 2).sum::<i64>();
    if graph.max_flow(n + m + 2, n + m + 3) != expected {
        out.print_line(-1);
        return;
    }*/
    let mut graph = Graph::new(n + m + 2);
    for i in 0..n {
        for j in 0..m {
            graph.add_edge(i, FlowEdge::with_payload(j + n, r[(i, j)], l[(i, j)]));
        }
        graph.add_edge(n + m, FlowEdge::with_payload(i, a[(i, 1)], a[(i, 0)]));
    }
    for j in 0..m {
        graph.add_edge(
            j + n,
            FlowEdge::with_payload(n + m + 1, b[(j, 1)], b[(j, 0)]),
        );
    }
    if !graph.flow_with_demand(n + m, n + m + 1) {
        out.print_line(-1);
        return;
    }
    let mut ans = Arr2d::new(n, m, 0);
    for i in 0..n {
        for e in &graph[i] {
            if e.to() >= n && e.to() < n + m {
                ans[(i, e.to() - n)] += e.flow(&graph);
            }
        }
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
