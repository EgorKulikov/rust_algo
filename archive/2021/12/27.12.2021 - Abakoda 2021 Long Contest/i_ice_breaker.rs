//{"name":"I. Ice Breaker","group":"Codeforces - Abakoda 2021 Long Contest","url":"http://codeforces.com/gym/103496/problem/I","interactive":false,"timeLimit":1500,"tests":[{"input":"10 4\nAlice Bob Cindy Dani Eve Franz Gabi Hans Isa Jules\n3\nAlice Bob Cindy\n2\nCindy Dani\n4\nDani Eve Franz Gabi\n2\nHans Isa\n","output":"0 1 1 2 3 3 3 -1 -1 -1\n"},{"input":"10 4\nAlice Bob Cindy Dani Eve Franz Gabi Hans Isa Jules\n3\nAlice Bob Cindy\n2\nBob Dani\n2\nDani Eve\n2\nCindy Eve\n","output":"0 1 1 2 2 -1 -1 -1 -1 -1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IIceBreaker"}}}

use algo_lib::collections::id::Id;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let names: Vec<String> = input.read_vec(n);

    let mut id = Id::new();
    for name in names {
        id.get(name);
    }
    let mut graph: Graph<WeightedEdge<u32>> = Graph::new(n + m);
    for i in 0..m {
        let club: Vec<String> = input.read();
        for name in club {
            let id = id.get(name);
            graph.add_edge(id, WeightedEdge::new(n + i, 1));
            graph.add_edge(n + i, WeightedEdge::new(id, 0));
        }
    }

    out_line!(graph
        .distances_from(id.get("Alice".to_string()))
        .into_iter()
        .take(n)
        .map(|v| v.map(|(v, _, _)| v))
        .collect_vec());
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
