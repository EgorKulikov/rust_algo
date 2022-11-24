//{"name":"PIZZA LAND !","group":"CodeChef - STRV2022","url":"https://www.codechef.com/STRV2022/problems/MYPROB2","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n11\n1 3\n1 2\n4 2\n5 2\n2 6\n5 9\n10 5\n7 3\n3 8\n7 11\n3\n6\n5 10 1 3 11 7\n5\n9 5 3 6 2\n4\n11 5 3 9\n","output":"YES 1\nNO\nYES 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PIZZALAND"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let edges = input.read_usize_pair_vec(n - 1).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let lca = graph.lca();

    let q = input.read_usize();
    for _ in 0..q {
        let m = input.read_usize();
        let l = input.read_usize_vec(m).dec_by_one();
        let mut from = l[0];
        let mut to = l[0];
        let mut good = true;
        for i in l {
            if lca.on_path(from, i, to) {
                to = i;
            } else if lca.on_path(to, i, from) {
                from = i;
            } else if !lca.on_path(from, to, i) {
                good = false;
                break;
            }
        }
        if good {
            out_line!("YES", lca.path_length(from, to) + 1 - m);
        } else {
            out_line!("NO");
        }
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
