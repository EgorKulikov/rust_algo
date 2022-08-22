//{"name":"Tree Coloring","group":"CodeChef - LTIME111A","url":"https://www.codechef.com/LTIME111A/problems-old/TREECLR","interactive":false,"timeLimit":1000,"tests":[{"input":"3 3\n1 2\n1 3\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TreeColoring"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let c = input.read_usize();
    let edges = input.read_usize_pair_vec(n - 1).dec_by_one();

    if n == 1 {
        out_line!(c);
        return;
    }
    if c == 1 {
        out_line!(0);
        return;
    }
    type Mod = ModInt7;
    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> Mod {
        let mut next_ways = Mod::from_index(c - 1);
        if prev != n {
            next_ways -= Mod::one();
        }
        let mut res = Mod::one();
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            res *= f.call(e.to(), vert) * next_ways;
            next_ways -= Mod::one();
        }
        res
    });
    out_line!(dfs.call(0, n) * Mod::from_index(c));
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
