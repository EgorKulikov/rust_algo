//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::BiEdgeAlgos;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let d = graph.diameter();
    if d + 1 == n {
        out.print_line(1);
        return;
    }
    let mut end = BitSet::new(n);
    let mut queue = Vec::new();
    let mut deg = vec![0; n];
    for i in 0..n {
        deg[i] = graph[i].len();
        if deg[i] == 1 {
            end.set(i);
            queue.push(i);
        }
    }
    let mut good = BitSet::new(n);
    while let Some(v) = queue.pop() {
        let mut num_non_end = 0;
        let mut num_good = 0;
        for e in &graph[v] {
            let u = e.to();
            if end[u] {
                continue;
            }
            if good[u] {
                num_good += 1;
            } else {
                num_non_end += 1;
            }
        }
        assert!(num_non_end <= 1);
        if num_good <= 1 {
            good.set(v);
            for e in &graph[v] {
                let u = e.to();
                if !end[u] && !good[u] {
                    deg[u] -= 1;
                    if deg[u] == 1 {
                        queue.push(u);
                    }
                }
            }
        }
    }
    let mut four = BitSet::new(n);
    for i in 0..n {
        if good[i] || end[i] {
            continue;
        }
        let mut num_good = 0;
        let mut num_non_good = 0;
        for e in &graph[i] {
            let u = e.to();
            if !end[u] {
                if good[u] {
                    num_good += 1;
                } else {
                    num_non_good += 1;
                }
            }
        }
        if num_non_good > 2 || num_non_good + num_good > 4 {
            out.print_line(3);
            return;
        }
        if num_non_good + num_good == 4 {
            four.set(i);
        }
    }
    for i in 0..n {
        if four[i] {
            let mut good = true;
            let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                if graph[vert].len() == 2 {
                    return;
                }
                if i != vert && four[vert] {
                    good = false;
                    return;
                }
                for e in &graph[vert] {
                    if e.to() == prev {
                        continue;
                    }
                    f.call(e.to(), vert);
                }
            });
            dfs.call(i, n);
            if !good {
                out.print_line(3);
                return;
            }
        }
    }
    out.print_line(2);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
