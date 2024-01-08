//{"name":"day25","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day25"}}}

use algo_lib::collections::id::Id;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::io::eol_string::EolString;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::scan;
use algo_lib::string::str::Str;
use std::collections::{HashSet, VecDeque};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut edges = Vec::new();

    while !input.is_empty() {
        scan!(input, "@: @", from: Str<'static>, to: EolString);
        for to in to.split(b' ') {
            edges.push((from.clone(), to.into_owned()));
        }
    }

    {
        // part 1
        let mut id = Id::new();
        for (f, t) in &edges {
            id.get(f);
            id.get(t);
        }
        let mut graph = Graph::new(id.len());
        for (f, t) in &edges {
            let f = id.get(f);
            let t = id.get(t);
            graph.add_edge(FlowEdge::new(f, t, 1));
            graph.add_edge(FlowEdge::new(t, f, 1));
        }
        for i in 1..id.len() {
            let mut graph = graph.clone();
            if graph.max_flow(0, i) == 3 {
                let mut left = HashSet::new();
                left.insert(0);
                let mut queue = VecDeque::from(vec![0]);
                while let Some(cur) = queue.pop_front() {
                    for e in &graph[cur] {
                        if e.capacity() == 1 && !left.contains(&e.to()) {
                            left.insert(e.to());
                            queue.push_back(e.to());
                        }
                    }
                }
                out.print_line(left.len() * (id.len() - left.len()));
                return;
            }
        }
    }
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
