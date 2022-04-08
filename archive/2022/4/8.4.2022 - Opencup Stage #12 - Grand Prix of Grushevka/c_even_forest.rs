//{"name":"C. Even Forest","group":"Yandex - Stage 12: Grand Prix of Grushevka","url":"https://official.contest.yandex.com/opencupXXII/contest/35268/problems/C/","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n1 2\n2 3\n3 4\n","output":"1\n"},{"input":"4\n1 2\n1 3\n1 4\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CEvenForest"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let edges = input.read_usize_pair_vec(n - 1).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    const INF: i32 = i32::MAX / 3;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (i32, i32, i32) {
        let mut odd_single = INF;
        let mut odd_mult = INF;
        let mut even = 0;
        let mut only = 0;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let (call_odd_single, call_odd_mult, call_even) = f.call(e.to(), vert);
            let new_odd_single =
                (odd_single + 1 + call_even.min(call_odd_mult)).min(only + call_even);
            let new_odd_mult = (odd_single + call_even)
                .min(odd_mult + call_even)
                .min(odd_mult + 1 + call_odd_mult);
            let new_even = (even + call_odd_single.min(call_odd_mult)).min(even + 1 + call_even);
            only += 1 + call_even.min(call_odd_mult);
            odd_single = new_odd_single;
            odd_mult = new_odd_mult;
            even = new_even;
        }
        (odd_single, odd_mult, even)
    });
    let (_, odd_mult, even) = dfs.call(0, 0);
    out_line!(odd_mult.min(even));
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
