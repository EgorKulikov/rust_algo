use algo_lib::collections::default_map::by_index;
use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::graph::Graph;
use algo_lib::graph::dfs_order::DFSOrderTrait;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::collections::BinaryHeap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let c = input.read_size_vec(n).dec();
    let k = input.read_size_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let order = graph.dfs_order();
    let lca = graph.lca();
    let pos = by_index(&c);
    let ans = Vec::with_gen(n, |i| {
        if pos[i].is_empty() {
            -1
        } else {
            let v = pos[i].clone().sorted_by_key(|&x| order.position[x]);
            let mut comp = Graph::new(0, 0);
            let mut stack = Vec::new();
            let mut id = FxHashMap::default();
            let mut is_special = Vec::new();
            let total = v.len();
            for i in v {
                while stack.len() >= 2 {
                    let a = stack[Back(1)];
                    let b = stack[Back(0)];
                    let l = lca.lca(a, b);
                    if lca.level(l) >= lca.level(lca.lca(b, i)) {
                        if a == l || b == l {
                            comp.add_edge(BiWeightedEdge::new(id[&b], id[&a], lca.level(a).abs_diff(lca.level(b))));
                        } else {
                            let x = comp.vertex_count();
                            comp.add_vertices(1);
                            id.insert(l, x);
                            is_special.push(false);
                            comp.add_edge(BiWeightedEdge::new(id[&b], x, lca.level(b).abs_diff(lca.level(l))));
                            comp.add_edge(BiWeightedEdge::new(id[&a], x, lca.level(a).abs_diff(lca.level(l))));
                        }
                        stack.pop();
                        stack.pop();
                        stack.push(l);
                    } else {
                        break;
                    }
                }
                let x = comp.vertex_count();
                comp.add_vertices(1);
                is_special.push(true);
                id.insert(i, x);
                stack.push(i);
            }
            while stack.len() >= 2 {
                let a = stack[Back(1)];
                let b = stack[Back(0)];
                let l = lca.lca(a, b);
                if a == l || b == l {
                    comp.add_edge(BiWeightedEdge::new(id[&b], id[&a], lca.level(a).abs_diff(lca.level(b))));
                } else {
                    let x = comp.vertex_count();
                    comp.add_vertices(1);
                    is_special.push(false);
                    id.insert(l, x);
                    comp.add_edge(BiWeightedEdge::new(id[&b], x, lca.level(b).abs_diff(lca.level(l))));
                    comp.add_edge(BiWeightedEdge::new(id[&a], x, lca.level(a).abs_diff(lca.level(l))));
                }
                stack.pop();
                stack.pop();
                stack.push(l);
            }
            let mut d = vec![0; comp.vertex_count()];
            let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                if is_special[vert] {
                    d[vert] = 1;
                }
                for e in comp.adj(vert) {
                    if e.to() == prev {
                        continue;
                    }
                    d[vert] += f.call(e.to(), vert);
                }
                d[vert]
            });
            dfs.call(0, 0);
            let mut center = 0;
            let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                if d[vert] * 2 < total {
                    return;
                }
                center = vert;
                for e in comp.adj(vert) {
                    if e.to() == prev {
                        continue;
                    }
                    f.call(e.to(), vert);
                }
            });
            dfs.call(0, 0);
            let mut rem = k[i] - 1;
            let mut ans = 0;
            let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> usize {
                let mut cur = if is_special[vert] { 1 } else { 0 };
                for e in comp.adj(vert) {
                    if e.to() == prev {
                        continue;
                    }
                    let call = f.call(e.to(), vert);
                    cur += call;
                    ans += call * e.weight();
                }
                d[vert] = cur;
                cur
            });
            dfs.call(center, center);
            let mut heap = BinaryHeap::new();
            for e in comp.adj(center) {
                heap.push((d[e.to()], e.weight(), e.to(), center));
            }
            while rem > 0 {
                let (weight, qty, to, from) = heap.pop().unwrap();
                let cur = rem.min(qty);
                ans -= cur * weight;
                rem -= cur;
                for e in comp.adj(to) {
                    if e.to() != from {
                        heap.push((d[e.to()], e.weight(), e.to(), to));
                    }
                }
            }
            ans as i64
        }
    });
    out.print_line(ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();

    match TEST_TYPE {
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}


#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
