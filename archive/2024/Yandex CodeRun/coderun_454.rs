//{"name":"coderun_454","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"coderun_454"}}}

use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponents;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();

    let mut r = vec![Vec::new(); n];
    for _ in 0..k {
        let a = input.read_size() - 1;
        let b = input.read_int();
        if b > 0 {
            r[a].push(2 * (b - 1) as usize);
        } else {
            r[a].push(2 * (-b - 1) as usize + 1);
        }
    }

    let mut graph = Graph::new(2 * m);
    for r in r {
        if r.is_empty() {
            continue;
        }
        let mut rec = RecursiveFunction2::new(|rec, from: usize, to: usize| -> (usize, usize) {
            if from + 1 == to {
                return (r[from], r[from] ^ 1);
            }
            let mid = (from + to) / 2;
            let (left_root, left_sink) = rec.call(from, mid);
            let (right_root, right_sink) = rec.call(mid, to);
            graph.add_edge(Edge::new(left_sink, right_root));
            graph.add_edge(Edge::new(right_sink, left_root));
            if from != 0 || to != r.len() {
                graph.add_vertices(1);
                let root = graph.vertex_count() - 1;
                graph.add_edge(Edge::new(root, left_root));
                graph.add_edge(Edge::new(root, right_root));
                graph.add_vertices(1);
                let sink = graph.vertex_count() - 1;
                graph.add_edge(Edge::new(left_sink, sink));
                graph.add_edge(Edge::new(right_sink, sink));
                (root, sink)
            } else {
                (0, 0)
            }
        });
        rec.call(0, r.len());
    }

    let (color, _) = graph.strongly_connected_components();
    let mut ans = Vec::new();
    for i in 0..m {
        if color[2 * i] == color[2 * i + 1] {
            out.print_line(-1);
            return;
        }
        if color[2 * i + 1] < color[2 * i] {
            ans.push(i + 1);
        }
    }
    out.print_line(ans.len());
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
