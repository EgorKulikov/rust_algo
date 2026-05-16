//{"name":"D. Concept album","group":"Yandex - Yandex Cup 2023 — Algorithm — Qualification","url":"https://contest.yandex.com/contest/54452/problems/D/","interactive":false,"timeLimit":1000,"tests":[{"input":"5 8\n1 3 >= 10\n2 4 <= 15\n3 5 >= 6\n2 5 >= 10\n4 4 <= 3\n1 2 >= 8\n1 5 <= 15\n3 4 >= -5\n","output":"YES\n"},{"input":"4 3\n1 4 >= 40\n1 2 <= 19\n3 4 <= 20\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DConceptAlbum"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        let f = input.read_size() - 1;
        let t = input.read_size();
        let d = input.read_char();
        input.read_char();
        let w = input.read_long();

        if d == '<' {
            edges.push((f, t, w));
        } else {
            edges.push((t, f, -w));
        }
    }

    let mut graph = Graph::new(n + 2);
    for &(f, t, w) in &edges {
        graph.add_edge(f, WeightedEdge::new(t, w));
    }
    for i in 0..=n {
        graph.add_edge(n + 1, WeightedEdge::new(i, 0));
    }
    let mut h = vec![None; n + 2];
    h[n + 1] = Some(0);
    for _ in 0..n + 2 {
        for i in 0..n + 2 {
            if let Some(d) = h[i] {
                for e in &graph[i] {
                    let next = e.to();
                    let new_dist = d + e.weight();
                    h[next].minim(new_dist);
                }
            }
        }
    }
    for i in 0..n + 2 {
        if let Some(d) = h[i] {
            for e in &mut graph[i] {
                let next = e.to();
                let new_dist = d + e.weight();
                if h[next].minim(new_dist) {
                    out.print_line(false);
                    return;
                }
            }
        }
    }
    out.print_line(true);
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
