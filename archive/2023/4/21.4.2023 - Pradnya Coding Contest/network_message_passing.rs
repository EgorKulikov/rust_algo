//{"name":"Network Message Passing","group":"CodeChef - PRAD2023","url":"https://www.codechef.com/PRAD2023/problems/NET_MES_PAS","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 2 3 4\n1 2\n2 3\n2 4\n","output":"2\n"},{"input":"6\n1 3 0 5 3 7\n1 2\n2 3\n3 4\n2 5\n1 6\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NetworkMessagePassing"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let scope = input.read_size_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let mut good = true;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> usize {
        let mut calls = Vec::new();
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            calls.push(f.call(e.to(), vert));
        }
        if calls.is_empty() {
            return 0;
        }
        if scope[vert] == 0 {
            good = false;
            return 0;
        }
        calls.sort();
        calls.reverse();
        let mut ans = calls[0] + 1;
        for (i, c) in calls.into_iter().step_by(scope[vert]).enumerate() {
            ans.maxim(c + i + 1);
        }
        ans
    });
    let ans = dfs.call(0, n);
    if good {
        out_line!(ans);
    } else {
        out_line!(-1);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
