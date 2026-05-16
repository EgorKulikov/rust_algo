//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, usize)>(m);

    let mut graph = Graph::new(n);
    for (u, v, c) in edges {
        graph.add_edge(u - 1, BiWeightedEdge::new(v - 1, c));
    }
    fn cut_points(graph: &Graph<BiWeightedEdge<usize>>) -> Vec<Vec<(usize, usize, usize)>> {
        let n = graph.vertex_count();
        let mut timer = 0;
        let mut tin = vec![0; n];
        let mut fup = vec![0; n];
        let mut used = vec![0; n];
        let mut ans = Vec::new();
        let mut stack = Vec::new();
        for i in 0..n {
            if used[i] == 0 {
                let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                    used[vert] = 1;
                    tin[vert] = timer;
                    fup[vert] = timer;
                    timer += 1;
                    for e in &graph[vert] {
                        if e.to() == prev {
                            continue;
                        }
                        let to = e.to();
                        if used[to] != 2 {
                            stack.push((vert, e.to(), e.weight()));
                        }
                        if used[to] != 0 {
                            fup[vert].minim(tin[to]);
                        } else {
                            f.call(to, vert);
                            let cand = fup[to];
                            fup[vert].minim(cand);
                            if fup[to] >= tin[vert] {
                                let mut cur = Vec::new();
                                while let Some((u, v, c)) = stack.pop() {
                                    cur.push((u, v, c));
                                    if u == vert && v == to {
                                        break;
                                    }
                                }
                                ans.push(cur);
                            }
                        }
                    }
                    used[vert] = 2;
                    // if prev == n && children > 1 {
                    // ans.push(stack.clone());
                    // }
                });
                dfs.call(i, n);
            }
        }
        ans
    }
    let cut_points = cut_points(&graph);
    for comp in cut_points {
        let mut col = DefaultHashMap::<_, u8>::new();
        for (u, v, c) in comp {
            col[u].set_bit(c);
            col[v].set_bit(c);
        }
        let mut qty = 0;
        let mut cols = 0;
        for &i in col.values() {
            if i.count_ones() > 1 {
                qty += 1;
            }
            cols |= i;
        }
        if qty > 2 && cols.count_ones() == 3 {
            out.print_line(true);
            return;
        }
    }
    out.print_line(false);
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
    output.set_bool_output(BoolOutput::YesNo);
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
    stress_test::stress_test(run, tester::check);
}
//END MAIN
