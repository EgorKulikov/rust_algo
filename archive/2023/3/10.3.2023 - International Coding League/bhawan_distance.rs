//{"name":"Bhawan Distance","group":"CodeChef - INCL2023","url":"https://www.codechef.com/INCL2023/problems/ICL_BHWNDIST","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n4\n1 0\n2 0\n3 2\n4 0\n5\n4 0\n5 4\n3 5\n2 4\n1 2\n","output":"1 2 3 3\n1 2 3 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BhawanDistance"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n);

    let mut graph = Graph::new(n + 1);
    for &(u, v) in &edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let lca = graph.lca();
    let mut left = 0;
    let mut right = 0;
    let mut dist = 0;
    let mut ans = Vec::with_capacity(n);
    let mut has = BitSet::new(n + 1);
    has.set(0, true);
    for i in 1..=n {
        let i = if has[edges[i - 1].0] {
            edges[i - 1].1
        } else {
            edges[i - 1].0
        };
        has.set(i, true);
        let d1 = lca.path_length(left, i);
        if dist.maxim(d1) {
            right = i;
        }
        let d2 = lca.path_length(right, i);
        if dist.maxim(d2) {
            left = i;
        }
        ans.push(dist);
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
