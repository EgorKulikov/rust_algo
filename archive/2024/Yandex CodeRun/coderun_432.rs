//{"name":"coderun_432","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"coderun_432"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::VecDeque;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_table(n, n);

    let mut graph = Graph::new(n);
    for i in 0..n {
        for j in 0..i {
            if a[(i, j)] != -1 {
                graph.add_edge(BiWeightedEdge::new(i, j, a[(i, j)]));
            }
        }
    }
    let dist = graph
        .distances_from(0)
        .into_iter()
        .map(|x| x.map(|(w, ..)| w))
        .collect::<Vec<_>>();
    let base = dist.iter().filter(|x| x.is_some()).count();
    let mut ans = 0;
    let mut reachable = BitSet::new(n);
    let mut queue = VecDeque::new();
    for i in 0..n {
        for j in 0..i {
            if a[(i, j)] == -1 {
                continue;
            }
            if dist[i].is_none()
                || dist[j].is_none()
                || (dist[i].unwrap() - dist[j].unwrap()).abs() != a[(i, j)]
            {
                continue;
            }
            reachable.fill(false);
            reachable.set(0);
            queue.clear();
            queue.push_back(0);
            while let Some(cur) = queue.pop_front() {
                for e in &graph[cur] {
                    let next = e.to();
                    if reachable[next] || (cur, next) == (i, j) || (cur, next) == (j, i) {
                        continue;
                    }
                    if dist[next].unwrap() - dist[cur].unwrap() == e.weight() {
                        reachable.set(next);
                        queue.push_back(next);
                    }
                }
            }
            ans.maxim(base - reachable.iter().count());
        }
    }
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
