use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::graph::Graph;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::VecDeque;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut abc = input.read_vec::<(usize, usize, usize)>(n);

    for (a, b, c) in abc.iter_mut() {
        *a -= 1;
        *b -= 1;
        *c -= 1;
    }

    let mut adj = vec![Vec::new(); 2 * n];
    for i in 0..n {
        let (a, b, c) = abc[i];
        adj[a].push(n + i);
        adj[b].push(n + i);
        adj[c].push(n + i);
        adj[n + i].push(a);
        adj[n + i].push(b);
        adj[n + i].push(c);
    }
    // for &(u, v) in edges.iter() {
    //     adj[u].push(v + l);
    //     adj[v + l].push(u);
    // }
    let mut mate = vec![None; 2 * n];

    let mut level = Vec::new();
    let mut queue: VecDeque<_> = (n..2 * n).collect();
    for iter in (0..2 * n).cycle() {
        // global relabeling heuristics
        if iter == 0 {
            level = vec![2 * n; 2 * n];

            let mut bfs = VecDeque::new();
            for u in 0..n {
                if mate[u].is_none() {
                    level[u] = 0;
                    bfs.push_back(u);
                }
            }
            while let Some(u) = bfs.pop_front() {
                for &v in adj[u].iter() {
                    if level[v] > level[u] + 1 {
                        level[v] = level[u] + 1;
                        if let Some(w) = mate[v] {
                            level[w] = level[v] + 1;
                            bfs.push_back(w);
                        }
                    }
                }
            }
        }

        // push-relabel iteration
        if let Some(v) = queue.pop_front() {
            if let Some(&u) = adj[v].iter().min_by_key(|&&u| level[u]) {
                if level[u] < 2 * n {
                    level[v] = level[u] + 1;
                    if let Some(w) = mate[u].take() {
                        mate[w] = None;
                        queue.push_back(w);
                    }
                    mate[v] = Some(u);
                    mate[u] = Some(v);
                    level[u] += 2;
                }
            }
        } else {
            break;
        }
    }


    /*let mut graph = Graph::new(2 * n + 2, 5 * n);
    for i in 0..n {
        graph.add_edge(FlowEdge::new(2 * n, i, 1));
        graph.add_edge(FlowEdge::new(n + i, 2 * n + 1, 1));
        let (a, b, c) = abc[i];
        graph.add_edge(FlowEdge::new(a, n + i, 1));
        graph.add_edge(FlowEdge::new(b, n + i, 1));
        graph.add_edge(FlowEdge::new(c, n + i, 1));
    }
    assert_eq!(graph.max_flow(2 * n, 2 * n + 1), n);*/
    let mut n_graph = Graph::new(2 * n, 3 * n);
    for i in 0..n {
        if let Some(v) = mate[i] {
            for x in adj[i].copy_iter() {
                if x == v {
                    n_graph.add_edge(Edge::new(x, i));
                } else {
                    n_graph.add_edge(Edge::new(i, x));
                }
            }
        }
        /*for e in graph.adj(i) {
            if e.to() >= n && e.to() < 2 * n {
                if e.flow(&graph) == 1 {
                    n_graph.add_edge(Edge::new(e.to(), i));
                } else {
                    n_graph.add_edge(Edge::new(i, e.to()));
                }
            }
        }*/
    }
    let graph = n_graph;
    let mut ans = vec![(-1, -1); n];
    let start = graph.adj(2 * n - 1).iter().next().unwrap().to();
    let mut visited = BitSet::new(2 * n);
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        visited.set(vert);
        for e in graph.adj(vert) {
            if !visited[e.to()] {
                if vert >= n {
                    ans[vert - n] = (prev as i32 + 1, e.to() as i32 + 1);
                }
                f.call(e.to(), vert);
            }
        }
    });
    dfs.call(start, 2 * n);
    if visited.count_ones() != 2 * n {
        out.print_line(false);
    } else {
        out.print_line(true);
        out.print_per_line(&ans);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
