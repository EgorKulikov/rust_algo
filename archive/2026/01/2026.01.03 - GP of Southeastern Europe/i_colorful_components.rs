//{"name":"I. Colorful Components","group":"Universal Cup - GP of Southeastern Europe","url":"https://contest.ucup.ac/contest/2828/problem/16123","interactive":false,"timeLimit":1000,"tests":[{"input":"4 4\n1 1 2 2\n1 3\n1 4\n2 3\n2 4\n","output":"0\n2\n1 3\n2 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::mem::take;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let c = input.read_size_vec(n).dec();
    let ee = input.read_size_pair_vec(m).dec();

    let mut edges: Vec<FxHashSet<usize>> = vec![FxHashSet::default(); n];
    for cc in 0..n {
        let mut graph = Graph::new(n + 2);
        for (u, v) in ee.copy_iter() {
            if c[u] == cc && c[v] != cc {
                graph.add_edge(FlowEdge::new(u, v, 1));
            } else if c[v] == cc && c[u] != cc {
                graph.add_edge(FlowEdge::new(v, u, 1));
            }
        }
        for i in 0..n {
            if c[i] == cc {
                graph.add_edge(FlowEdge::new(n, i, 1));
            } else {
                graph.add_edge(FlowEdge::new(i, n + 1, 1));
            }
        }
        graph.max_flow(n, n + 1);
        let mut new_edge_to = vec![None; n];
        for e in &graph[n] {
            if e.flow(&graph) == 1 {
                for f in &graph[e.to()] {
                    if f.flow(&graph) == 1 {
                        new_edge_to[e.to()] = Some(f.to());
                        new_edge_to[f.to()] = Some(e.to());
                        break;
                    }
                }
            }
        }
        let mut queue = Vec::new();
        for i in 0..n {
            if c[i] != cc && new_edge_to[i].is_none() {
                queue.push(i);
            }
        }
        while let Some(i) = queue.pop() {
            if edges[i].len() == 1 {
                let v = *edges[i].iter().next().unwrap();
                if c[v] == cc {
                    if let Some(x) = new_edge_to[v] {
                        new_edge_to[x] = None;
                        new_edge_to[v] = None;
                        queue.push(x);
                    }
                }
            }
        }
        for i in 0..n {
            if c[i] == cc {
                if let Some(_) = new_edge_to[i] {
                    let was = take(&mut edges[i]);
                    for x in was {
                        edges[x].remove(&i);
                    }
                }
            }
        }
        for i in 0..n {
            if c[i] == cc {
                if let Some(v) = new_edge_to[i] {
                    edges[i].insert(v);
                    if edges[v].len() == 1 {
                        let other = *edges[v].iter().next().unwrap();
                        if edges[other].len() != 1 || new_edge_to[other].is_some() {
                            edges[other].remove(&v);
                            edges[v].remove(&other);
                        }
                    }
                    edges[v].insert(i);
                }
            }
        }
    }
    let mut dsu = DSU::new(n);
    for i in 0..n {
        for v in &edges[i] {
            dsu.union(i, *v);
        }
    }
    let mut bad = 0;
    for i in 0..n {
        if dsu.size(i) == 1 {
            bad += 1;
        }
    }
    out.print_line(bad);
    let mut ans = Vec::new();
    for i in 0..n {
        for v in &edges[i] {
            if ee.contains(&(i, *v)) {
                ans.push((i, *v));
            }
        }
    }
    out.print_line(ans.len());
    out.print_per_line(&ans.inc());
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
