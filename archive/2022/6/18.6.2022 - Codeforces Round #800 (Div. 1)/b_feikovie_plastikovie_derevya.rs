//{"name":"B. Фейковые пластиковые деревья","group":"Codeforces - Codeforces Round #800 (Div. 1)","url":"https://codeforces.com/contest/1693/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n1\n1 5\n2 9\n3\n1 1\n4 5\n2 4\n6 10\n4\n1 2 1\n6 9\n5 6\n4 5\n2 4\n5\n1 2 3 4\n5 5\n4 4\n3 3\n2 2\n1 1\n","output":"1\n2\n2\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFeikoviePlastikovieDerevya"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let p = input.read_usize_vec(n - 1).dec_by_one();
    let limits = input.read_long_pair_vec(n);

    let mut graph = Graph::new(n);
    for (i, p) in p.into_iter().enumerate() {
        graph.add_edge(p, Edge::new(i + 1));
    }
    let mut ans = 0;
    let mut dfs = RecursiveFunction::new(|f, v: usize| -> i64 {
        let mut sum = 0;
        for e in &graph[v] {
            sum += f.call(e.to());
        }
        let (l, r) = limits[v];
        if sum >= l {
            sum.min(r)
        } else {
            ans += 1;
            r
        }
    });
    dfs.call(0);
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
