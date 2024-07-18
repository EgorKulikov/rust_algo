//{"name":"C. Циркуль для дерева","group":"Codeforces - Codeforces Round 934 (Div. 1)","url":"https://codeforces.com/contest/1943/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n2\n1 2\n4\n1 2\n1 3\n1 4\n7\n2 7\n3 2\n6 4\n5 7\n1 6\n6 7\n","output":"1\n1 0\n2\n1 1\n2 1\n2\n1 1\n2 1\n3\n6 1\n7 1\n2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTsirkulDlyaDereva"}}}

use algo_lib::collections::iter_ext::min_max::IterMinMaxPos;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    let dist = graph.edge_distances(0);
    let end1 = dist.into_iter().max_position().unwrap();
    let dist1 = graph.edge_distances(end1);
    let end2 = dist1.iter().max_position().unwrap();
    let diameter = dist1[end2];
    let dist2 = graph.edge_distances(end2);
    let mut centers = Vec::new();
    let r1 = diameter / 2;
    let r2 = (diameter + 1) / 2;
    for i in 0..n {
        if dist1[i] == r1 && dist2[i] == r2 || dist1[i] == r2 && dist2[i] == r1 {
            centers.push(i);
        }
    }
    if centers.len() == 1 || r1 % 2 == 0 {
        out.print_line(r2 + 1);
        for i in 0..=r2 {
            out.print_line((centers[0] + 1, i));
        }
    } else {
        out.print_line(r1 + 1);
        for i in (1..=r1).step_by(2) {
            out.print_line((centers[0] + 1, i));
        }
        for i in (1..=r1).step_by(2) {
            out.print_line((centers[1] + 1, i));
        }
    }
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
