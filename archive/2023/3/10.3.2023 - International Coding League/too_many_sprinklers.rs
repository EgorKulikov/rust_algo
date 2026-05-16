//{"name":"Too Many Sprinklers ","group":"CodeChef - INCL2023","url":"https://www.codechef.com/INCL2023/problems/ICL_SPRNKLRS","interactive":false,"timeLimit":1000,"tests":[{"input":"10 10 3 3\n3 3\n5 5\n3 10\n","output":"1\n"},{"input":"50 50 3 3\n3 3\n5 5\n3 10\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TooManySprinklers"}}}

use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let _l = input.read_int();
    let w = input.read_int();
    let n = input.read_size();
    let r = input.read_int();
    let pos = input.read_int_pair_vec(n);

    let mut graph = Graph::new(2 * n + 2);
    let source = 2 * n;
    let sink = 2 * n + 1;
    for i in 0..n {
        graph.add_edge(2 * i, FlowEdge::new(2 * i + 1, 1));
        if pos[i].1 <= r {
            graph.add_edge(source, FlowEdge::new(2 * i, 600));
        }
        if pos[i].1 >= w - r {
            graph.add_edge(2 * i + 1, FlowEdge::new(sink, 600));
        }
        for j in 0..n {
            if i != j && (pos[i].0 - pos[j].0).pow(2) + (pos[i].1 - pos[j].1).pow(2) <= 4 * r.pow(2)
            {
                graph.add_edge(2 * i + 1, FlowEdge::new(2 * j, 600));
            }
        }
    }
    out_line!(graph.max_flow(source, sink));
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
