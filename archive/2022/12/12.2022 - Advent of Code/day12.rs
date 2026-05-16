//{"name":"day12","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day12"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::dirs::D4;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let mut s = Vec::new();

    while !input.is_exhausted() {
        s.push(input.read::<Str>());
        input.skip_whitespace();
    }

    let n = s.len();
    let m = s[0].len();
    // let mut start = None;
    let mut finish = None;
    let mut graph = Graph::new(n * m);
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == b'S' {
                // start = Some(i * m + j);
                s[i][j] = b'a';
            }
            if s[i][j] == b'E' {
                finish = Some(i * m + j);
                s[i][j] = b'z';
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            for (ni, nj) in D4::iter(i, j, n, m) {
                // if s[ni][nj] <= s[i][j] + 1 {
                if s[i][j] <= s[ni][nj] + 1 {
                    graph.add_edge(i * m + j, Edge::new(ni * m + nj));
                }
            }
        }
    }
    // let start = start.unwrap();
    let finish = finish.unwrap();
    // out_line!(graph.edge_distances(start)[finish]);
    let mut ans = None;
    let dist = graph.edge_distances(finish);
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == b'a' {
                ans.minim(dist[i * m + j]);
            }
        }
    }
    out_line!(ans);
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
