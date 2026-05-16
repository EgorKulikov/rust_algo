//{"name":"D. Ela and the Wiring Wizard","group":"Codeforces - Dytechlab Cup 2022","url":"https://codeforces.com/contest/1737/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n8 9\n1 2 3\n6 4 5\n3 5 6\n6 1 3\n7 4 4\n3 8 4\n2 3 3\n7 8 5\n4 5 2\n4 5\n1 2 1\n2 4 1\n3 4 1\n3 1 1\n1 3 2\n8 8\n4 6 92\n7 1 65\n6 5 43\n6 7 96\n4 3 74\n4 8 54\n7 4 99\n2 5 22\n","output":"9\n2\n154\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DElaAndTheWiringWizard"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::all_distances::AllDistances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let edges = input.read_vec::<(usize, usize, usize)>(m);

    let mut g = Graph::new(n);
    for &(u, v, _) in &edges {
        g.add_edge(u - 1, BiWeightedEdge::new(v - 1, 1usize));
    }
    let d = g.all_distances();
    let mut ans = None;
    for (u, v, w) in edges {
        ans.minim((d[(0, u - 1)] + d[(n - 1, v - 1)] + 1) * w);
        ans.minim((d[(n - 1, u - 1)] + d[(0, v - 1)] + 1) * w);
        for i in 0..n {
            ans.minim((d[(i, 0)] + d[(i, n - 1)] + d[(i, u - 1)].max(d[(i, v - 1)]) + 1) * w);
        }
    }
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
