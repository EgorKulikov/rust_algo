//{"name":"J. Tournament","group":"CPython.uz - CPython Programming Contest #3","url":"https://cpython.uz/competitions/contests/contest/326/problem/J","interactive":false,"timeLimit":1000,"tests":[{"input":"3 6L XLXL LXXL XLS XSM SM L\n","output":"YES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JTournament"}}}

use algo_lib::collections::iter_ext::find_count::IterFindCount;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_int();
    let m = input.read_size();
    let mut sz = Vec::with_capacity(m);
    let variants: [&[u8]; 6] = [b"XXL", b"XL", b"L", b"M", b"S", b"XS"];
    for _ in 0..m {
        let s1 = variants
            .iter()
            .find_eq(&input.read_str().as_slice())
            .unwrap();
        let s2 = variants
            .iter()
            .find_eq(&input.read_str().as_slice())
            .unwrap();
        sz.push((s1, s2));
    }

    let mut graph = Graph::new(m + 8);
    for (pos, (i, j)) in sz.into_iter().enumerate() {
        graph.add_edge(FlowEdge::new(m + 6, pos, 1));
        graph.add_edge(FlowEdge::new(pos, m + i, 1));
        graph.add_edge(FlowEdge::new(pos, m + j, 1));
    }
    for i in 0..6 {
        graph.add_edge(FlowEdge::new(m + i, m + 7, n));
    }
    out.print_line(graph.max_flow(m + 6, m + 7) == m as i32);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test(run, tester::check);
}
//END MAIN
