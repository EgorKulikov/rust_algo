//{"name":"F. Fast Climb","group":"Yandex - SNWS-2022, Round 5","url":"https://contest.yandex.ru/snws2022/contest/23961/problems/F/","interactive":false,"timeLimit":2000,"tests":[{"input":"5 6\n0 3 3\n0 1 9\n3 2 12\n2 1 6\n2 4 2\n4 3 10\n","output":"7\n"},{"input":"7 8\n1 0 8\n3 6 0\n0 6 12\n2 1 6\n2 4 2\n4 3 10\n4 5 3\n4 4 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FFastClimb"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let edges: Vec<(usize, usize, usize)> = input.read_vec(m);

    if n == 1 {
        out_line!(0);
        return;
    }
    let mut graph = Graph::new(n);
    for &(u, v, t) in &edges {
        graph.add_edge(u, BiWeightedEdge::new(v, t));
    }
    let f = graph.distance(0, n - 1).unwrap().0;
    let first = if f == 0 {
        0
    } else {
        (f - 1) / 12 * 24 + if f % 12 == 0 { 12 } else { f % 12 }
    };
    let mut graph = Graph::new(n * 13);
    for (u, v, t) in edges {
        for j in 0..=12 - t {
            graph.add_edge(u * 13 + j, WeightedEdge::new(v * 13 + j + t, t));
            graph.add_edge(v * 13 + j, WeightedEdge::new(u * 13 + j + t, t));
        }
    }
    for i in 0..n {
        for j in 1..=12 {
            graph.add_edge(i * 13 + j, WeightedEdge::new(i * 13, 24 - j));
        }
    }
    let mut second = None;
    let dist = graph.distances_from(0);
    for i in 0..=12 {
        match dist[(n - 1) * 13 + i] {
            None => {}
            Some((val, _, _)) => {
                second.minim(val);
            }
        }
    }
    out_line!(second.unwrap() - first);
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
