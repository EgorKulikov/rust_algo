//{"name":"C. Arrow Path","group":"Codeforces - Educational Codeforces Round 163 (Rated for Div. 2)","url":"https://codeforces.com/contest/1948/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n>><<\n>>><\n2\n><\n><\n4\n>>><\n>><<\n6\n>><<><\n><>>><\n","output":"YES\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CArrowPath"}}}

use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let arrows = input.read_str_vec(2);

    let mut graph = Graph::new(2 * n);
    for i in 0..2 {
        for j in 0..n {
            for (r, mut c) in D4::iter(i, j, 2, n) {
                if arrows[r][c] == b'>' {
                    c += 1;
                } else {
                    c -= 1;
                }
                graph.add_edge(Edge::new(i * n + j, r * n + c));
            }
        }
    }
    out.print_line(graph.edge_distances(0)[2 * n - 1] != u32::MAX);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
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
    //    tester::stress_test();
}
//END MAIN
