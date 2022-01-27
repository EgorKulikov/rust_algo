//{"name":"E. Ecofuel","group":"Yandex - SNWS-2022, Round 4","url":"https://contest.yandex.ru/snws2022/contest/23960/problems/E/","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n2 1 3\n1 2 2\n3 2 3\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EEcofuel"}}}

use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let v = input.read_usize() - 1;
    let f = input.read_usize() - 1;
    let c = input.read_usize() - 1;
    let edges: Vec<(usize, usize, u32)> = input.read_vec(m);

    let mut left = 0;
    let mut right = u32::MAX / 2;
    while left < right {
        let mid = (left + right + 1) / 2;
        let mut graph = Graph::new(n + 1);
        graph.add_edge(n, FlowEdge::new(f, mid));
        graph.add_edge(n, FlowEdge::new(c, mid));
        for &(u, v, x) in &edges {
            graph.add_edge(u - 1, FlowEdge::new(v - 1, x * 2));
            graph.add_edge(v - 1, FlowEdge::new(u - 1, x * 2));
        }
        if graph.max_flow(n, v) == 2 * mid {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    out_line!(left);
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
