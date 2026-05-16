//{"name":"D. Разделение рёбер","group":"Codeforces - Codeforces Round #819 (Div. 1 + Div. 2) and Grimoire of Code Annual Contest 2022","url":"https://codeforces.com/contest/1726/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 7\n1 2\n2 3\n3 4\n4 5\n5 1\n1 3\n3 5\n4 4\n1 2\n2 3\n1 4\n3 4\n6 7\n1 2\n1 3\n3 4\n4 5\n1 4\n5 6\n6 2\n2 1\n1 2\n","output":"0111010\n1001\n0001111\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRazdelenieRyober"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdgeWithId;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let edges = input.read_usize_pair_vec(m).dec_by_one();

    let mut color: Str = vec![b'0'; m].into();
    let mut graph = Graph::new(n);
    for &(u, v) in &edges {
        graph.add_edge(u, BiEdgeWithId::new(v));
    }
    let mut back_edges = Vec::with_capacity(m + 1 - n);
    let mut on_stack = BitSet::new(n);
    let mut visited = BitSet::new(n);
    let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, red: bool| {
        on_stack.set(vert, true);
        visited.set(vert, true);
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            if on_stack[e.to()] {
                back_edges.push(e.id());
                continue;
            }
            if visited[e.to()] {
                continue;
            }
            color[e.id()] = if red { b'0' } else { b'1' };
            f.call(e.to(), vert, !red);
        }
        on_stack.set(vert, false);
    });
    dfs.call(0, n, false);
    assert_eq!(back_edges.len(), m + 1 - n);
    for i in 0..(1 << back_edges.len()) {
        for j in 0..back_edges.len() {
            if i.is_set(j) {
                color[back_edges[j]] = b'1';
            } else {
                color[back_edges[j]] = b'0';
            }
        }
        let mut red = DSU::new(n);
        let mut blue = DSU::new(n);
        let mut good = true;
        for j in 0..m {
            if color[j] == b'1' {
                if !red.join(edges[j].0, edges[j].1) {
                    good = false;
                    break;
                }
            } else {
                if !blue.join(edges[j].0, edges[j].1) {
                    good = false;
                    break;
                }
            }
        }
        if good {
            out_line!(color);
            return;
        }
    }
    unreachable!();
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
