//{"name":"Find Diameter","group":"CodeChef - START104A","url":"https://www.codechef.com/START104A/problems/FINDIAMETER","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5\n1 1 1 1 1\n3\n1 5 10\n3\n3 4 3\n","output":"0\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FindDiameter"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut graph = Graph::new(n);
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| a[i]);
    for i in 1..n {
        graph.add_edge(i, BiWeightedEdge::new(i - 1, 1));
        graph.add_edge(
            order[i],
            BiWeightedEdge::new(order[i - 1], a[order[i]] - a[order[i - 1]]),
        );
    }
    let mut ans = None;
    let mut queue = vec![Vec::new(); n];
    let mut was = BitSet::new(n);
    for i in 0..n {
        for j in 0..n {
            queue[j].clear();
        }
        was.fill(false);
        queue[0].push(i);
        for i in 0..n {
            while let Some(x) = queue[i].pop() {
                if was[x] {
                    continue;
                }
                ans.maxim(i);
                was.set(x);
                for e in &graph[x] {
                    let to = e.to();
                    let w = i + e.weight();
                    if w < n {
                        queue[w].push(to);
                    }
                }
            }
        }
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
