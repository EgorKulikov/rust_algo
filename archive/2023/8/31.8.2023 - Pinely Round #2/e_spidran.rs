//{"name":"E. Спидран","group":"Codeforces - Pinely Round 2 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1863/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n4 4 24\n12 16 18 12\n1 2\n1 3\n2 4\n3 4\n4 3 10\n2 6 5 9\n1 4\n2 4\n3 4\n2 1 10\n5 5\n1 2\n5 0 1000\n8 800 555 35 35\n5 0 10\n3 2 5 4 7\n3 2 5\n4 3 2\n1 2\n2 3\n","output":"24\n7\n0\n480\n5\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ESpidran"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::topological_sort::TopologicalSort;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_long();
    let h = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::from_edges(n, &edges);
    let order = graph.topological_sort().unwrap();
    let mut shift = vec![0; n];
    for &i in &order {
        for e in &graph[i] {
            let to = e.to();
            if h[to] < h[i] {
                let c = shift[i] + k - h[i] + h[to];
                shift[to].maxim(c);
            } else {
                let c = shift[i] + h[to] - h[i];
                shift[to].maxim(c);
            }
        }
    }
    let mut segments = Vec::with_capacity(n);
    for i in 0..n {
        segments.push((h[i] - shift[i], h[i]));
    }
    segments.sort();
    let mut end = h.iter().max().copied().unwrap();
    let start = segments[0].0;
    let mut ans = end - start;
    for (s, e) in segments {
        ans.minim(end - s);
        if s >= start + k {
            break;
        }
        end.maxim(e + k);
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
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
