//{"name":"C. Арена покемонов","group":"Codeforces - Codeforces Round 930 (Div. 1)","url":"https://codeforces.com/contest/1936/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n3 3\n2 3 1\n2 9 9\n6 1 7\n1 2 1\n3 3\n2 3 1\n9 9 9\n6 1 7\n1 2 1\n4 2\n2 8 3 5\n18 24\n17 10\n1 10\n1 1\n6 3\n21412674 3212925 172015806 250849370 306960171 333018900\n950000001 950000001 950000001\n821757276 783362401 760000001\n570000001 700246226 600757652\n380000001 423513575 474035234\n315201473 300580025 287023445\n1 1 1\n","output":"2\n6\n17\n1224474550\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CArenaPokemonov"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let c = input.read_long_vec(n);
    let a = input.read_long_table(n, m);

    let mut graph = Graph::new(n * (m + 1));
    for j in 0..m {
        let mut vals = a.column(j).copied().collect_vec();
        vals.sort();
        for i in 1..n {
            graph.add_edge(WeightedEdge::new((j + 1) * n + i - 1, (j + 1) * n + i, 0));
            graph.add_edge(WeightedEdge::new(
                (j + 1) * n + i,
                (j + 1) * n + i - 1,
                vals[i] - vals[i - 1],
            ));
        }
        for i in 0..n {
            let pos = vals.lower_bound(&a[(i, j)]);
            graph.add_edge(WeightedEdge::new(i, (j + 1) * n + pos, 0));
            graph.add_edge(WeightedEdge::new((j + 1) * n + pos, i, c[i]));
        }
    }
    out.print_line(graph.distance(0, n - 1).unwrap().0);
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
