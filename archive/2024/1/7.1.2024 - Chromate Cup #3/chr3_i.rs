//{"name":"chr3_i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"chr3_i"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, usize)>(m).dec();

    let mut graph = Graph::new(n);
    for (u, v, w) in edges {
        graph.add_edge(BiWeightedEdge::new(u, v, w));
    }
    let mut visited = BitSet::new(n);
    let mut on_stack = BitSet::new(n);
    let mut stack = Vec::new();
    let mut cycles = Vec::new();
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        if on_stack[vert] {
            let mut cycle = Vec::new();
            for (v, w) in stack.iter().rev() {
                cycle.push(*w);
                if *v == vert {
                    break;
                }
            }
            cycles.push(cycle);
            return;
        }
        if visited[vert] {
            return;
        }
        visited.set(vert);
        on_stack.set(vert);
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            stack.push((vert, e.weight()));
            f.call(e.to(), vert);
            stack.pop();
        }
        on_stack.unset(vert);
    });
    for i in 0..n {
        dfs.call(i, n);
    }
    for cycle in cycles {
        let sum = cycle.iter().sum::<usize>();
        if sum % 2 == 1 {
            out.print_line(false);
            return;
        }
        let mut cur = BitSet::new(2 * sum + 1);
        let mut next = BitSet::new(2 * sum + 1);
        let qty = cycle.qty();
        cur.set(sum);
        for i in qty.indices().skip(1) {
            next.fill(false);
            let step = i * qty[i];
            for j in 0..2 * i {
                let mut until = 0;
                for k in (step + j..=2 * sum).step_by(2 * i) {
                    if cur[k] {
                        until = k + step + 1;
                    }
                    if k - step < until {
                        next.set(k - step);
                    }
                }
            }
            swap(&mut cur, &mut next);
        }
        if !cur[sum] {
            out.print_line(false);
            return;
        }
    }
    out.print_line(true);
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
