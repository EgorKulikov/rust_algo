//{"name":"Single source shortest path, time table","group":"Kattis","url":"https://open.kattis.com/problems/shortestpath2","interactive":false,"timeLimit":4000,"tests":[{"input":"4 4 4 0\n0 1 15 10 5\n1 2 15 10 5\n0 2 5 5 30\n3 0 0 1 1\n0\n1\n2\n3\n2 1 1 0\n0 1 100 0 5\n1\n0 0 0 0\n","output":"0\n20\n30\nImpossible\n\n105\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::indexed_heap::IndexedHeap;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let s = input.read_size();
    let edges: Vec<(usize, usize, i32, i32, i32)> = input.read_vec(m);
    let t = input.read_size_vec(q);

    if n == 0 {
        return;
    }

    let graph = Graph::new(n).do_with(|g| {
        for (u, v, t0, p, d) in edges {
            g.add_edge(Edge::with_payload(u, v, (t0, p, d)));
        }
    });

    let distances_from = || -> Vec<Option<i32>> {
        let mut res = vec![None; n];
        let mut heap = IndexedHeap::new(n);
        res[s] = Some(0);
        heap.add_or_adjust(s, 0);
        while let Some((cur, dist)) = heap.pop() {
            res[cur] = Some(dist);
            for e in &graph[cur] {
                let next = e.to();
                if res[next].is_some() {
                    continue;
                }
                let &(t0, p, d) = e.payload();
                let dep = if dist <= t0 {
                    t0
                } else {
                    if p == 0 {
                        continue;
                    }
                    (dist - t0).upper_div(p) * p + t0
                };
                let total = dep + d;
                heap.add_or_relax(next, total);
            }
        }
        res
    };
    let dist = distances_from();
    for i in t {
        if let Some(d) = dist[i] {
            out.print_line(d);
        } else {
            out.print_line("Impossible");
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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

//START MAIN
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
//END MAIN
