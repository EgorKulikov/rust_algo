//{"name":"Disgust Minimization","group":"CodeChef - START55A","url":"https://www.codechef.com/START55A/problems-old/DISGUST","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 2\n3 3 1\n1 2 1\n3 2 1\n3 2 2\n1 2 3\n2 1\n2 2\n2 2\n1 1 3\n3 2\n3 3 1\n1 2 1\n3 2 1\n2 1 1\n3 2 1\n","output":"10\n0\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DisgustMinimization"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::all_distances::AllDistances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::sign::Unsigned;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_table::<usize>(n, n);
    let edges = input.read_vec::<(usize, usize, usize)>(m);

    let mut graph = Graph::new(n);
    for (x, y, z) in edges {
        graph.add_edge(x - 1, WeightedEdge::new(y - 1, z));
    }
    let dist = graph.all_distances();
    let inter = Arr2d::generate(n, n, |i, j| {
        let mut res = usize::MAX;
        for k in 0..n {
            if dist[(j, k)] != usize::MAX {
                let dst = i.distance(k);
                res.minim(2 * dst * dst + dist[(j, k)]);
            }
        }
        res
    });
    let res = Arr2d::generate(n, n, |i, j| {
        let mut res = usize::MAX;
        for k in 0..n {
            if inter[(k, j)] != usize::MAX && dist[(i, k)] != usize::MAX {
                res.minim(dist[(i, k)] + inter[(k, j)]);
            }
        }
        res
    });
    let mut ans = 0;
    for i in 0..n {
        for j in 0..i {
            ans += res[(a[(i, j)] - 1, a[(j, i)] - 1)];
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
