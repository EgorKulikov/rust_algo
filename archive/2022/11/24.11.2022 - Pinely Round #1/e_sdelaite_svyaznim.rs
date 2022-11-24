//{"name":"E. Сделайте связным","group":"Codeforces - Pinely Round 1 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1761/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n011\n100\n100\n3\n000\n001\n010\n4\n0100\n1000\n0001\n0010\n6\n001100\n000011\n100100\n101000\n010001\n010010\n","output":"0\n1\n1\n2\n3 4\n3\n2 5 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ESdelaiteSvyaznim"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s = input.read_table::<char>(n, n);

    let mut graph = Graph::new(n);
    for i in 0..n {
        for j in 0..i {
            if s[(i, j)] == '1' {
                graph.add_edge(i, BiEdge::new(j));
            }
        }
    }
    let mut dsu = DSU::new(n);
    for i in 0..n {
        for e in &graph[i] {
            dsu.join(i, e.to());
        }
    }
    if dsu.count() == 1 {
        out_line!(0);
        return;
    }
    let mut sz = vec![0; n];
    let mut edges = vec![0; n];
    let mut mn = vec![n; n];
    for i in 0..n {
        sz[dsu.get(i)] += 1;
        edges[dsu.get(i)] += graph[i].len();
        mn[dsu.get(i)].minim(graph[i].len());
    }
    for i in 0..n {
        if sz[i] == 1 || graph[i].len() + 1 != sz[dsu.get(i)] && graph[i].len() == mn[dsu.get(i)] {
            out_line!(1);
            out_line!(i + 1);
            return;
        }
    }
    if dsu.count() > 2 {
        out_line!(2);
        for i in 1..n {
            if dsu.get(i) != dsu.get(0) {
                out_line!(1, i + 1);
                return;
            }
        }
        unreachable!();
    }
    let mut first = Vec::new();
    let mut second = Vec::new();
    for i in 0..n {
        if dsu.get(i) == dsu.get(0) {
            first.push(i + 1);
        } else {
            second.push(i + 1);
        }
    }
    if first.len() < second.len() {
        out_line!(first.len());
        out_line!(first);
    } else {
        out_line!(second.len());
        out_line!(second);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
