//{"name":"C. Keshi ищет AmShZ","group":"Codeforces - Codeforces Round #800 (Div. 1)","url":"https://codeforces.com/contest/1693/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"2 1\n1 2\n","output":"1\n"},{"input":"4 4\n1 2\n1 4\n2 4\n1 4\n","output":"2\n"},{"input":"5 7\n1 2\n2 3\n3 5\n1 4\n4 3\n4 5\n3 1\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CKeshiIshchetAmShZ"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::BTreeSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let edges = input.read_usize_pair_vec(m).dec_by_one();

    let mut processed = BitSet::new(n);
    let mut rem_edges = vec![0; n];
    let mut graph = Graph::new(n);
    let mut answer = vec![None; n];
    for (u, v) in edges {
        if u == n - 1 {
            continue;
        }
        rem_edges[u] += 1;
        graph.add_edge(v, Edge::new(u));
    }
    answer[n - 1] = Some(0);
    let mut queue = BTreeSet::new();
    queue.insert((0, n - 1));
    while let Some(&(len, vert)) = queue.iter().next() {
        processed.set(vert, true);
        queue.remove(&(len, vert));
        for e in &graph[vert] {
            let cur = e.to();
            if processed[cur] {
                continue;
            }
            if let Some(c_ans) = answer[cur] {
                queue.remove(&(c_ans, cur));
            }
            rem_edges[cur] -= 1;
            answer[cur].minim(len + rem_edges[cur] + 1);
            queue.insert((answer[cur].unwrap(), cur));
        }
    }
    out_line!(answer[0]);
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
