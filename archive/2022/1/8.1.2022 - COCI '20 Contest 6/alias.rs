//{"name":"#2 - Alias","group":"DMOJ - COCI '20 Contest 6","url":"https://dmoj.ca/problem/coci20c6p2","interactive":false,"timeLimit":1000,"tests":[{"input":"3 2\nnovak goat 1\ngoat simulator 3\n2\nnovak simulator\nsimulator goat\n","output":"4\nRoger\n"},{"input":"3 3\nkile legend 4\nlegend beer 5\nbeer kile 6\n2\nkile beer\nlegend kile\n","output":"9\n11\n"},{"input":"4 5\nrafael me 5\nme ow 6\now ausopenfinal 2012\nausopenfinal me 2\nrafael ausopenfinal 2\n3\nrafael me\nme rafael\now me\n","output":"4\nRoger\n2014\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Alias"}}}

use algo_lib::collections::id::Id;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let m = input.read_usize();

    let mut graph = Graph::new(n);
    let mut id = Id::new();
    for _ in 0..m {
        let from = input.read_string();
        let to = input.read_string();
        let time = input.read_long();
        graph.add_edge(id.get(from), WeightedEdge::new(id.get(to), time));
    }
    let q = input.read_usize();
    for _ in 0..q {
        let from = input.read_string();
        let to = input.read_string();
        match graph.distance(id.get(from), id.get(to)) {
            None => {
                out_line!("Roger");
            }
            Some((dist, _)) => {
                out_line!(dist);
            }
        }
    }
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
