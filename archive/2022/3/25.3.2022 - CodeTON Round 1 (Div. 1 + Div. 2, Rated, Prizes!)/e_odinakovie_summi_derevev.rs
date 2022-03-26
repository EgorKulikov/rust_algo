//{"name":"E. Одинаковые суммы деревьев","group":"Codeforces - CodeTON Round 1 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1656/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5\n1 2\n1 3\n3 4\n3 5\n3\n1 2\n1 3\n","output":"-3 5 1 2 2\n1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EOdinakovieSummiDerevev"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable4, RecursiveFunction4};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let edges = input.read_usize_pair_vec(n - 1).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let mut a = vec![0; n];
    let mut dfs = RecursiveFunction4::new(|f, vert: usize, prev: usize, w: i32, wn: i32| {
        let mut children = 0;
        for e in &graph[vert] {
            if e.to() != prev {
                children += 1;
                f.call(e.to(), vert, wn, -wn);
            }
        }
        a[vert] = w - children * wn;
    });
    dfs.call(0, 0, 0, 1);
    out_line!(a);
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
