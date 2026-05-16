//{"name":"Medicine Strength","group":"CodeChef - START74A","url":"https://www.codechef.com/START74A/problems/MEDSTRONG","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2 3\n2 1\n3 1\n6 5\n4 1\n5 1\n4 5\n1 3\n3 2\n2 1\n","output":"4\n28\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MedicineStrength"}}}

use algo_lib::collections::vec_ext::{IncDec, Qty};
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponents;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(n).dec_by_one();

    let mut graph = Graph::new(m);
    for (u, v) in edges {
        graph.add_edge(u, Edge::new(v));
    }
    let (c, graph) = graph.strongly_connected_components();
    let qty = c.qty_bound(graph.vertex_count());
    let mut free = 0;
    let mut bad = Vec::new();
    for i in 0..graph.vertex_count() {
        if graph[i].is_empty() {
            bad.push(qty[i]);
        } else {
            free += qty[i];
        }
    }
    type Mod = ModInt7;
    let mut ans = Mod::new(2).power(free);
    for x in bad {
        ans *= Mod::new(2).power(x) - Mod::one();
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
