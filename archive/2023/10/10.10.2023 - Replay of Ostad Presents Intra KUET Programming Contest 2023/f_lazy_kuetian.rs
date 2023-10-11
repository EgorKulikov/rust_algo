//{"name":"F. Lazy KUETian","group":"Codeforces - Replay of Ostad Presents Intra KUET Programming Contest 2023","url":"https://codeforces.com/gym/104663/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"10 10 2 1\n1 2 5\n10 1 3\n4 2 3\n2 3 8\n3 10 1\n3 5 4\n4 3 2\n6 4 7\n7 8 3\n8 9 0\n5\n3\n5\n6\n8\n4\n","output":"8\n12\n25\n-1\n11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FLazyKUETian"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let s = input.read_size() - 1;
    let edges = input.read_vec::<(usize, usize, i64)>(m);

    let mut graph = Graph::new(n * (k + 1));
    for (mut u, mut v, t) in edges {
        u -= 1;
        v -= 1;
        for i in 0..=k {
            graph.add_edge(u + i * n, WeightedEdge::new(v + i * n, t));
        }
        for i in 0..k {
            graph.add_edge(v + i * n, WeightedEdge::new(u + (i + 1) * n, 2 * t));
        }
    }
    let distances = graph.distances_from(s);
    let mut ans = vec![None; n];
    for i in 0..n {
        for j in 0..=k {
            let d = distances[i + j * n];
            if let Some((d, ..)) = d {
                ans[i].minim(d);
            }
        }
    }

    let q = input.read_size();
    for _ in 0..q {
        let v = input.read_size() - 1;
        out.print_line(ans[v]);
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
    let test_type = TestType::Single;
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
