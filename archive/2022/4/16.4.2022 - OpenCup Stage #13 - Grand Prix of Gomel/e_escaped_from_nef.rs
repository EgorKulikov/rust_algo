//{"name":"E. Escaped from NEF","group":"Yandex - Stage 13: Grand Prix of Gomel","url":"https://official.contest.yandex.com/opencupXXII/contest/35270/problems/E/","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n3 3\n1 2\n1 3\n2 3\n5 5\n1 2\n2 3\n3 4\n4 5\n4 2\n","output":"6\n18\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EEscapedFromNEF"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::vec_ext::{IncDec, Qty};
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::topological_sort::TopologicalSort;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::{compress, out_line};
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let edges = input.read_usize_pair_vec(m).dec_by_one();

    let mut graph = Graph::new(n);
    for &(u, v) in &edges {
        graph.add_edge(u, WeightedEdge::new(v, 1));
        graph.add_edge(v, WeightedEdge::new(u, -1));
    }
    let mut was = BitSet::new(n);
    let mut stack: Vec<(usize, i32)> = Vec::new();
    let mut on_stack = BitSet::new(n);
    let mut dsu = DSU::new(n);
    let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, dir: i32| {
        if was[vert] {
            if !on_stack[vert] {
                return;
            }
            let mut id = stack.len() - 1;
            let mut good = true;
            while stack[id].0 != vert {
                if stack[id].1 != dir {
                    good = false;
                    break;
                }
                id -= 1;
            }
            if good {
                for &(i, _) in &stack[id + 1..] {
                    dsu.join(vert, i);
                }
            }
            return;
        }
        stack.push((vert, dir));
        was.set(vert, true);
        on_stack.set(vert, true);
        for e in &graph[vert] {
            if e.to() != prev {
                f.call(e.to(), vert, e.weight());
            }
        }
        on_stack.set(vert, false);
        stack.pop();
    });
    dfs.call(0, 0, 0);
    let mut id = Vec::with_capacity(n);
    for i in 0..n {
        id.push(dsu.get(i));
    }
    let (c, (id,)) = compress!(id);
    let nn = c.len();
    let mut graph = Graph::new(nn);
    let weight = id.qty_bound(nn);
    for &(u, v) in &edges {
        if id[u] != id[v] {
            graph.add_edge(id[u], WeightedEdge::new(id[v], 1));
            graph.add_edge(id[v], WeightedEdge::new(id[u], -1));
        }
    }
    let mut doubles = vec![Vec::new(); nn];
    let mut was2 = BitSet::new(n);
    let mut stack2: Vec<(usize, i32)> = Vec::new();
    let mut on_stack = BitSet::new(n);
    let mut vd = VecDeque::with_capacity(n);
    let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, dir: i32| {
        if was2[vert] {
            if !on_stack[vert] {
                return;
            }
            let mut id = stack2.len() - 1;
            vd.push_back((vert, dir));
            while stack2[id].0 != vert {
                vd.push_back(stack2[id]);
                id -= 1;
            }
            while vd.front().unwrap().1 != -1 || vd.back().unwrap().1 != 1 {
                let front = vd.pop_front().unwrap();
                vd.push_back(front);
            }
            let mut good = true;
            let mut last = -1;
            for &(_, d) in &vd {
                if last == 1 && d == -1 {
                    good = false;
                }
                last = d;
            }
            if good {
                let start = vd[0].0;
                for &(i, d) in &vd {
                    if d == 1 {
                        doubles[start].push(i);
                        break;
                    }
                }
            }
            vd.clear();
            return;
        }
        stack2.push((vert, dir));
        was2.set(vert, true);
        on_stack.set(vert, true);
        for e in &graph[vert] {
            if e.to() != prev {
                f.call(e.to(), vert, e.weight());
            }
        }
        stack2.pop();
        on_stack.set(vert, false);
    });
    dfs.call(0, 0, 0);
    let mut graph = Graph::new(nn);
    for (u, v) in edges {
        if id[u] != id[v] {
            graph.add_edge(id[u], Edge::new(id[v]));
        }
    }
    let order = graph.topological_sort().unwrap();
    let mut res = vec![0i64; nn];
    let mut ans = 0;
    for i in order.into_iter().rev() {
        res[i] = weight[i].into_i64();
        for e in &graph[i] {
            res[i] += res[e.to()];
        }
        for &j in &doubles[i] {
            res[i] -= res[j];
        }
        ans += res[i] * weight[i].into_i64();
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
