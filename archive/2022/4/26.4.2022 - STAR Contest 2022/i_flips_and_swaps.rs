//{"name":"I. Flips and Swaps","group":"Codeforces - STAR Contest 2022","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378214/problem/I","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n11010\n3\n1 3\n2 4 3\n2 4 5\n","output":"4\n"},{"input":"2\n00\n1\n1 1\n","output":"-1\n"},{"input":"1\n1\n1\n1 1\n","output":"0\n"},{"input":"5\n00000\n6\n2 3 5\n2 2 5\n1 1\n2 2 4\n2 1 3\n2 1 4\n","output":"11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IFlipsAndSwaps"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a: Str = input.read();
    let m = input.read_usize();
    let mut can_change = BitSet::new(n);
    let mut edges = Vec::new();
    let mut any = None;
    for _ in 0..m {
        let tp = input.read_int();
        if tp == 1 {
            let x = input.read_usize() - 1;
            can_change.set(x, true);
            any = Some(x);
        } else {
            let x = input.read_usize() - 1;
            let y = input.read_usize() - 1;
            edges.push((x, y));
        }
    }

    if a.iter().find(b'0').is_none() {
        out_line!(0);
        return;
    }
    if any.is_none() {
        out_line!(-1);
        return;
    }
    let any = any.unwrap();
    let mut graph = Graph::new(n);
    for (mut x, mut y) in edges {
        if can_change[x] {
            x = any;
        }
        if can_change[y] {
            y = any;
        }
        if x != y {
            graph.add_edge(x, BiEdge::new(y));
        }
    }
    let dist = graph.edge_distances(any);
    let mut ans = 0i64;
    for (i, c) in a.iter().enumerate() {
        if c == b'0' {
            if can_change[i] {
                ans += 1;
            } else if dist[i] == u32::MAX {
                out_line!(-1);
                return;
            } else {
                ans += dist[i].into_i64() + 1;
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
