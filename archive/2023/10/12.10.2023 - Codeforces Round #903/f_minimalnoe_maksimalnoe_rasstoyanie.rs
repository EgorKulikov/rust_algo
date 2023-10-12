//{"name":"F. Минимальное максимальное расстояние","group":"Codeforces - Codeforces Round 903 (Div. 3)","url":"https://codeforces.com/contest/1881/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n7 3\n2 6 7\n1 2\n1 3\n2 4\n2 5\n3 6\n3 7\n4 4\n1 2 3 4\n1 2\n2 3\n3 4\n5 1\n1\n1 2\n1 3\n1 4\n1 5\n5 2\n4 5\n1 2\n2 3\n1 4\n4 5\n10 8\n1 2 3 4 5 8 9 10\n2 10\n10 5\n5 3\n3 1\n1 7\n7 4\n4 9\n8 9\n6 1\n10 9\n1 2 4 5 6 7 8 9 10\n1 3\n3 9\n9 4\n4 10\n10 6\n6 7\n7 2\n2 5\n5 8\n","output":"2\n2\n0\n1\n4\n5\n"},{"input":"3\n6 1\n3\n1 2\n1 3\n3 4\n3 5\n2 6\n5 3\n1 2 5\n1 2\n1 3\n2 4\n3 5\n7 1\n2\n3 2\n2 6\n6 1\n5 6\n7 6\n4 5\n","output":"0\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMinimalnoeMaksimalnoeRasstoyanie"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_vec(k).dec();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::from_biedges(n, &edges);
    let dist = graph.edge_distances(0);
    let mut best = None;
    for &i in &a {
        best.maxim((dist[i], i));
    }
    let dist = graph.edge_distances(best.unwrap().1);
    let mut ans = None;
    for &i in &a {
        ans.maxim(dist[i].upper_div(2));
    }
    out.print_line(ans);
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
            for i in 0usize..t {
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
