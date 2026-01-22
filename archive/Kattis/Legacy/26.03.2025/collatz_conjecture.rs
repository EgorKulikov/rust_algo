//{"name":"Collatz Conjecture","group":"Kattis","url":"https://open.kattis.com/problems/collatz","interactive":false,"timeLimit":1000,"tests":[{"input":"7 8\n27 30\n0 0\n","output":"7 needs 13 steps, 8 needs 0 steps, they meet at 8\n27 needs 95 steps, 30 needs 2 steps, they meet at 46\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::id::Id;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::misc::time_tracker::TimeTracker;
use algo_lib::output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut tt = TimeTracker::new();
    let mut graph = Graph::new(2168612);
    let mut id = Id::new();
    for i in 0..=1_000_000 {
        id.get(i);
    }
    graph.add_edge(Edge::new(0, 1));
    let mut size = 1_000_001;
    let mut queue = Vec::new();
    for i in 2..=1_000_000 {
        let to = if i % 2 == 0 { i / 2 } else { 3 * i + 1 };
        if to > 1_000_000 {
            if id.get(to) == size {
                size += 1;
                queue.push(to);
            }
            graph.add_edge(Edge::new(id.get(to), i));
        } else {
            graph.add_edge(Edge::new(to, i));
        }
    }
    while let Some(i) = queue.pop() {
        let to = if i % 2 == 0 { i / 2 } else { 3 * i + 1 };
        if to > 1_000_000 {
            if id.get(to) == size {
                size += 1;
                queue.push(to);
            }
            graph.add_edge(Edge::new(id.get(to), id.get(i)));
        } else {
            graph.add_edge(Edge::new(to, id.get(i)));
        }
    }
    tt.milestone("build graph");
    let mut level = vec![0; graph.vertex_count()];
    let mut parent = vec![0; graph.vertex_count()];
    let mut dfs = RecursiveFunction2::new(|dfs, vert: usize, prev: usize| {
        parent[vert] = prev;
        level[vert] = level[prev] + 1;
        for e in &graph[vert] {
            dfs.call(e.to(), vert);
        }
    });
    dfs.call(1, 0);

    tt.milestone("levels");
    let by_id = id.by_id();
    eprintln!("Len = {}", by_id.len());
    tt.milestone("by id");

    loop {
        let a = input.read_size();
        let b = input.read_size();
        if a == 0 && b == 0 {
            break;
        }

        let mut aa = id.get(a);
        let mut bb = id.get(b);
        let mut sa = 0;
        let mut sb = 0;
        while aa != bb {
            match level[aa].cmp(&level[bb]) {
                std::cmp::Ordering::Less => {
                    bb = parent[bb];
                    sb += 1;
                }
                std::cmp::Ordering::Greater => {
                    aa = parent[aa];
                    sa += 1;
                }
                std::cmp::Ordering::Equal => {
                    aa = parent[aa];
                    sa += 1;
                    bb = parent[bb];
                    sb += 1;
                }
            }
        }

        output!(
            out,
            "{a} needs {sa} steps, {b} needs {sb} steps, they meet at {}",
            by_id[aa],
        );
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
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
