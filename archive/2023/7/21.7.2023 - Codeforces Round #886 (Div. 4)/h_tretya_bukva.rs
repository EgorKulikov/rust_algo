//{"name":"H. Третья буква","group":"Codeforces - Codeforces Round 886 (Div. 4)","url":"https://codeforces.com/contest/1850/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 3\n1 2 2\n2 3 4\n4 2 -6\n6 5\n1 2 2\n2 3 4\n4 2 -6\n5 4 4\n3 5 100\n2 2\n1 2 5\n1 2 4\n4 1\n1 2 3\n","output":"YES\nNO\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HTretyaBukva"}}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let c = input.read_vec::<(usize, usize, i64)>(m);

    let mut graph = Graph::new(n);
    for (a, b, w) in c {
        graph.add_edge(a - 1, WeightedEdge::new(b - 1, w));
        graph.add_edge(b - 1, WeightedEdge::new(a - 1, -w));
    }
    let mut pos = vec![None; n];
    for i in 0..n {
        if pos[i].is_some() {
            continue;
        }
        let mut rec = RecursiveFunction2::new(|f, v: usize, d: i64| -> bool {
            if pos[v].is_some() {
                return pos[v].unwrap() == d;
            }
            pos[v] = Some(d);
            for e in &graph[v] {
                if !f.call(e.to(), d + e.weight()) {
                    return false;
                }
            }
            true
        });
        if !rec.call(i, 0) {
            out_line!(false);
            return;
        }
    }
    out_line!(true);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
