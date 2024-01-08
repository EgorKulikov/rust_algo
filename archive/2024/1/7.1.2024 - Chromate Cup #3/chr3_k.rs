//{"name":"chr3_k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"chr3_k"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::collections::slice_ext::reversed::BackwardSlice;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use std::collections::VecDeque;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();

    let mut graph = Graph::new(n);
    for (u, v, w) in edges {
        graph.add_edge(BiWeightedEdge::new(u, v, w));
    }
    let mut visited = BitSet::new(n);
    let mut on_stack = BitSet::new(n);
    let mut stack = Vec::new();
    let mut cycles = vec![Vec::new(); n];
    let mut tree_edges = vec![Vec::new(); n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> bool {
        if on_stack[vert] {
            let mut cycle = Vec::new();
            for (v, w, on_cycle) in stack.iter_mut().rev() {
                cycle.push((*v, *w));
                *on_cycle = true;
                if *v == vert {
                    break;
                }
            }
            cycle.reverse();
            cycles[vert].push(cycle);
            return true;
        }
        if visited[vert] {
            return false;
        }
        visited.set(vert);
        on_stack.set(vert);
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            stack.push((vert, e.weight(), false));
            if f.call(e.to(), vert) && !stack.backward()[0].2 {
                tree_edges[vert].push((e.to(), e.weight()));
            }
            stack.pop();
        }
        on_stack.unset(vert);
        true
    });
    let mut bases = VecDeque::new();
    for i in 0..n {
        if dfs.call(i, n) {
            bases.push_back(i);
        }
    }
    let mut x = vec![0; n];
    while let Some(vert) = bases.pop_front() {
        for &(to, w) in &tree_edges[vert] {
            bases.push_back(to);
            x[to] = x[vert] + w;
        }
        for cycle in &cycles[vert] {
            let base_cycle = cycle.iter().map(|&(_, w)| w as usize).collect_vec();
            let sum = base_cycle.iter().sum::<usize>();
            if sum % 2 == 1 {
                out.print_line(false);
                return;
            }
            let qty = base_cycle.qty();
            let mut cur = Arr2d::new(qty.len(), 2 * sum + 1, u32::MAX);
            cur[(0, sum)] = 0;
            for i in qty.indices().skip(1) {
                let step = i * qty[i];
                for j in 0..2 * i {
                    let mut until = 0;
                    for k in (step + j..=2 * sum).step_by(2 * i) {
                        if cur[(i - 1, k)] != u32::MAX {
                            until = k + step + 1;
                        }
                        if k - step < until {
                            cur[(i, k - step)] = (until - step - 1) as u32;
                        }
                    }
                }
            }
            if cur[(qty.len() - 1, sum)] == u32::MAX {
                out.print_line(false);
                return;
            }
            let mut positive = vec![0; qty.len()];
            let mut pos = sum;
            for i in qty.indices().skip(1).rev() {
                let last = cur[(i, pos)];
                let base = (last as usize) - i * qty[i];
                let posit = (pos - base) / (2 * i);
                positive[i] = posit;
                pos = last as usize;
            }
            let mut cur = x[vert];
            for &(v, w) in cycle {
                x[v] = cur;
                if v != vert {
                    bases.push_back(v);
                }
                let ww = w as usize;
                if positive[ww] > 0 {
                    cur += w;
                    positive[ww] -= 1;
                } else {
                    cur -= w;
                }
            }
        }
    }
    out.print_line(true);
    out.print_line(x);
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
    tester::stress_test();
}
//END MAIN
