//{"name":"Arithmetic Jumping","group":"CodeChef - START55A","url":"https://www.codechef.com/START55A/problems-old/AJ","interactive":false,"timeLimit":2000,"tests":[{"input":"7 3\n1 2\n2 3\n1 4\n4 6\n6 7\n1 5\n1 2\n6 6\n6 5\n","output":"3\n5\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ArithmeticJumping"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let q = input.read_usize();
    let edges = input.read_usize_pair_vec(n - 1).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let lca = graph.lca();
    let mut ends = Vec::with_capacity(2);
    let mut cur = 0;
    for _ in 0..2 {
        let mut end = 0;
        let mut max = 0;
        for i in 0..n {
            let d = lca.path_length(cur, i);
            if d > max {
                max = d;
                end = i;
            }
        }
        ends.push(end);
        cur = end;
    }
    for _ in 0..q {
        let p = input.read_usize() - 1;
        let q = input.read_usize() - 1;
        let mut max_dist = ends
            .iter()
            .map(|&end| lca.path_length(q, end))
            .max()
            .unwrap();
        while lca.path_length(p, q) % 2 != max_dist * (max_dist + 1) / 2 % 2 {
            max_dist -= 1;
        }
        out_line!(max_dist + 1);
    }
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
