//{"name":"G. Graph Reduction","group":"Yandex - Stage 16: Grand Prix of Urals","url":"https://official.contest.yandex.com/opencupXXII/contest/38278/problems/G/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 4\n1 5\n1 6\n2 4\n2 5\n2 6\n3 4\n3 5\n3 6\n","output":"Possible\n1 2 3\n"},{"input":"2\n1 2\n1 3\n1 4\n2 3\n2 4\n3 4\n","output":"Possible\n2 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GGraphReduction"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::BTreeSet;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let edges = input.read_usize_pair_vec(3 * n).dec_by_one();

    let mut graph = Graph::new(2 * n);
    for &(u, v) in &edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let mut side = BitSet::new(2 * n);
    let mut same = vec![3; 2 * n];
    let mut other = vec![0; 2 * n];
    let mut queue = (0..2 * n).collect::<BTreeSet<_>>();
    while let Some(&v) = queue.iter().next() {
        queue.remove(&v);
        for e in &graph[v] {
            if side[e.to()] == side[v] {
                same[e.to()] -= 1;
                other[e.to()] += 1;
                if same[e.to()] < other[e.to()] {
                    queue.remove(&e.to());
                }
            } else {
                same[e.to()] += 1;
                other[e.to()] -= 1;
                if same[e.to()] > other[e.to()] {
                    queue.insert(e.to());
                }
            }
        }
        side.set(v, !side[v]);
        swap(&mut same[v], &mut other[v]);
    }
    let mut ans = BTreeSet::new();
    for (i, (u, v)) in edges.into_iter().enumerate() {
        if side[u] == side[v] {
            ans.insert(i + 1);
        }
    }
    for i in 1..=3 * n {
        if ans.len() < n {
            ans.insert(i);
        }
    }
    out_line!("Possible");
    out_line!(ans.into_iter().collect_vec());
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
