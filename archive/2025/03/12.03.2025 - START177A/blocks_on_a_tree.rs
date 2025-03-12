//{"name":"Blocks on a Tree","group":"CodeChef - START177A","url":"https://www.codechef.com/START177A/problems/BLOCKTREE","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 0\n1 2\n2 3\n3 2\n1 2\n2 3\n4 1\n1 2\n2 3\n2 4\n","output":"1\n0 0 0\n2\n1 1 0\n2\n0 0 0 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut k = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let lca = graph.lca();
    let mut size = vec![0; n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        size[vert] += 1;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            size[vert] += f.call(e.to(), vert);
        }
        size[vert]
    });
    dfs.call(0, n);
    let mut ans = vec![0; n];
    for i in 0..n {
        if size[i] == k {
            let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                ans[vert] = 1;
                for e in &graph[vert] {
                    if e.to() == prev {
                        continue;
                    }
                    f.call(e.to(), vert);
                }
            });
            dfs.call(i, lca.parent(i).unwrap_or(n));
            out.print_line(if k == n { 1 } else { 2 });
            out.print_line(ans);
            return;
        }
        if size[i] + k == n {
            if k != 0 {
                let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                    ans[vert] = 1;
                    for e in &graph[vert] {
                        if e.to() == prev {
                            continue;
                        }
                        f.call(e.to(), vert);
                    }
                });
                dfs.call(lca.parent(i).unwrap(), i);
            }
            out.print_line(if k == 0 { 1 } else { 2 });
            out.print_line(ans);
            return;
        }
    }
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        if k > 0 {
            k -= 1;
            ans[vert] = 1;
        }
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert);
        }
    });
    dfs.call(0, n);
    out.print_line(3);
    out.print_line(ans);
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
