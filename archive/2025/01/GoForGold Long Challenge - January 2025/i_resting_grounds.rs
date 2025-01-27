//{"name":"I. Resting Grounds","group":"Codeforces - GoForGold Long Challenge - January 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/579716/problem/I","interactive":false,"timeLimit":2000,"tests":[{"input":"3 5 1\n2 3 2 3 17\n2 3 2 2 16\n2 2 2 3 3\n3 3 1 1 12\n1 3 3 17\n","output":"0 28 12\n"},{"input":"4 3 1\n3 4 1 3 12\n2 2 3 4 10\n1 2 4 16\n","output":"0 -1 -1 12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_size() - 1;

    let mut graph = Graph::new(n);
    let mut outgoing = FxHashMap::default();
    let mut rec = RecursiveFunction2::new(|rec, f: usize, t: usize| -> usize {
        if f + 1 == t {
            outgoing.insert((f, t), f);
            f
        } else {
            let mid = (f + t) / 2;
            let a = rec.call(f, mid);
            let b = rec.call(mid, t);
            let cur = graph.vertex_count();
            graph.add_vertices(1);
            graph.add_edge(WeightedEdge::new(cur, a, 0));
            graph.add_edge(WeightedEdge::new(cur, b, 0));
            outgoing.insert((f, t), cur);
            cur
        }
    });
    rec.call(0, n);
    let mut incoming = FxHashMap::default();
    let mut rec = RecursiveFunction2::new(|rec, f: usize, t: usize| -> usize {
        if f + 1 == t {
            incoming.insert((f, t), f);
            f
        } else {
            let mid = (f + t) / 2;
            let a = rec.call(f, mid);
            let b = rec.call(mid, t);
            let cur = graph.vertex_count();
            graph.add_vertices(1);
            graph.add_edge(WeightedEdge::new(a, cur, 0));
            graph.add_edge(WeightedEdge::new(b, cur, 0));
            incoming.insert((f, t), cur);
            cur
        }
    });
    rec.call(0, n);

    for _ in 0..q {
        let t = input.read_size();
        match t {
            1 => {
                let x = input.read_size() - 1;
                let y = input.read_size() - 1;
                let w = input.read_long();
                graph.add_edge(WeightedEdge::new(x, y, w));
            }
            2 => {
                let x = input.read_size() - 1;
                let l = input.read_size() - 1;
                let r = input.read_size();
                let w = input.read_long();
                let mut rec = RecursiveFunction2::new(|rec, f: usize, t: usize| {
                    if f >= r || t <= l {
                        return;
                    }
                    if f >= l && t <= r {
                        graph.add_edge(WeightedEdge::new(x, outgoing[&(f, t)], w));
                        return;
                    }
                    let mid = (f + t) / 2;
                    rec.call(f, mid);
                    rec.call(mid, t);
                });
                rec.call(0, n);
            }
            3 => {
                let x = input.read_size() - 1;
                let l = input.read_size() - 1;
                let r = input.read_size();
                let w = input.read_long();
                let mut rec = RecursiveFunction2::new(|rec, f: usize, t: usize| {
                    if f >= r || t <= l {
                        return;
                    }
                    if f >= l && t <= r {
                        graph.add_edge(WeightedEdge::new(incoming[&(f, t)], x, w));
                        return;
                    }
                    let mid = (f + t) / 2;
                    rec.call(f, mid);
                    rec.call(mid, t);
                });
                rec.call(0, n);
            }
            _ => unreachable!(),
        }
    }

    let dist = graph.distances_from(s);
    out.print_line_iter(dist.iter_map(|x| x.map(|y| y.0)).take(n));
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
