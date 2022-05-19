//{"name":"G. Алгоритм Люсине","group":"Codeforces - Codeforces Round #792 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1684/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"7 20\n1 8 1 6 3 2 3\n","output":"3\n19 11\n15 9\n3 7\n"},{"input":"2 10\n7 1\n","output":"-1\n"},{"input":"2 15\n1 7\n","output":"1\n15 8\n"},{"input":"1 1000000000\n845063470\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GAlgoritmLyusine"}}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let t = input.read_usize_vec(n);

    let mut left = Vec::new();
    let mut right = Vec::new();
    for i in t {
        if 2 * i >= m {
            out_line!(-1);
            return;
        }
        if 3 * i > m {
            left.push(i);
        } else {
            right.push(i);
        }
    }
    let mut graph = Graph::new(n + 2);
    for i in 0..left.len() {
        graph.add_edge(n, FlowEdge::new(i, 1));
    }
    for j in left.len()..n {
        graph.add_edge(j, FlowEdge::new(n + 1, 1));
    }
    for i in 0..left.len() {
        for j in 0..right.len() {
            if left[i] % right[j] == 0 && 2 * left[i] + right[j] <= m {
                graph.add_edge(i, FlowEdge::new(j + left.len(), 1));
            }
        }
    }
    if graph.max_flow(n, n + 1) != left.len() {
        out_line!(-1);
        return;
    }
    let mut ans = Vec::new();
    for i in 0..left.len() {
        for e in &graph[i] {
            if e.flow(&graph) == 1 {
                let j = e.to() - left.len();
                ans.push((2 * left[i] + right[j], left[i] + right[j]));
                break;
            }
        }
    }
    for j in 0..right.len() {
        if graph[j + left.len()][0].flow(&graph) == 0 {
            ans.push((3 * right[j], 2 * right[j]));
        }
    }
    out_line!(ans.len());
    output().print_per_line(&ans);
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
