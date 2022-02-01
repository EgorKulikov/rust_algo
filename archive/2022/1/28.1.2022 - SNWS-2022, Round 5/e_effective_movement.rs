//{"name":"E. Effective Movement","group":"Yandex - SNWS-2022, Round 5","url":"https://contest.yandex.ru/snws2022/contest/23961/problems/E/","interactive":false,"timeLimit":2000,"tests":[{"input":"2 0 2 2\n","output":"2.0000000000\n"},{"input":"21 1 20 22\n","output":"21.4142135624\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EEffectiveMovement"}}}

use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let xs = input.read_usize();
    let ys = input.read_usize();
    let xf = input.read_usize();
    let yf = input.read_usize();
    // let xs = 2;
    // let ys = 0;
    // let xf = 300;
    // let yf = 300;

    let mut graph = Graph::new(301 * 301);
    for i in 0..=300 {
        for j in 0..=300 {
            if i < 300 {
                graph.add_edge(i * 301 + j, BiWeightedEdge::new((i + 1) * 301 + j, 1f64));
            }
            if j < 300 {
                graph.add_edge(i * 301 + j, BiWeightedEdge::new(i * 301 + j + 1, 1.));
            }
        }
    }

    let dx = if xs > xf { xs - xf } else { xf - xs };
    let dy = if ys > yf { ys - yf } else { yf - ys };
    let ratio = if dx > dy {
        dx / (dy + 1)
    } else {
        dy / (dx + 1)
    };
    let mut lens = Vec::with_capacity(301);
    for i in 0..=300 {
        lens.push(f64::hypot(1., i as f64));
    }
    for i in (1..300).step_by(2) {
        for j in 0..=300 {
            for k in 0..j {
                if j - k + 40 < ratio || j - k > ratio + 40 {
                    continue;
                }
                graph.add_edge(
                    i * 301 + j,
                    BiWeightedEdge::new((i + 1) * 301 + k, lens[j - k]),
                );
                graph.add_edge(
                    i * 301 + k,
                    BiWeightedEdge::new((i + 1) * 301 + j, lens[j - k]),
                );
                graph.add_edge(
                    j * 301 + i,
                    BiWeightedEdge::new(k * 301 + i + 1, lens[j - k]),
                );
                graph.add_edge(
                    k * 301 + i,
                    BiWeightedEdge::new(j * 301 + i + 1, lens[j - k]),
                );
            }
        }
    }

    out_line!(graph.distance(xs * 301 + ys, xf * 301 + yf).unwrap().0);
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
