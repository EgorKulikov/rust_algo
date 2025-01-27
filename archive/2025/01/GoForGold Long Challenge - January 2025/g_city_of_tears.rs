//{"name":"G. City of Tears","group":"Codeforces - GoForGold Long Challenge - January 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/579716/problem/G","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n\n6 7\n1 4\n1 3\n3 4\n4 5\n2 1\n5 5\n5 6\n\n1 0\n\n3 3\n1 2\n2 3\n3 1\n\n5 0\n\n4 4\n1 2\n2 3\n1 4\n4 3\n","output":"1 0 1 2 -1 -1\n1\n-1 -1 -1\n1 0 0 0 0\n1 1 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::with_edges(n, &edges);
    let mut ans = vec![0; n];
    let mut on_stack = BitSet::new(n);
    let mut dfs = RecursiveFunction::new(|dfs, vert: usize| {
        if on_stack[vert] {
            ans[vert] = 3;
            return;
        }
        if ans[vert] != 0 {
            ans[vert].maxim(2);
            return;
        }
        ans[vert] = 1;
        on_stack.set(vert);
        for e in &graph[vert] {
            dfs.call(e.to());
        }
        on_stack.unset(vert);
    });
    dfs.call(0);
    let mut vis = BitSet::new(n);
    for i in 0..n {
        if ans[i] == 3 {
            let mut dfs = RecursiveFunction::new(|dfs, vert: usize| {
                if vis[vert] {
                    return;
                }
                vis.set(vert);
                ans[vert] = -1;
                for e in &graph[vert] {
                    dfs.call(e.to());
                }
            });
            dfs.call(i);
        }
    }
    vis.fill(false);
    for i in 0..n {
        if ans[i] == 2 {
            let mut dfs = RecursiveFunction::new(|dfs, vert: usize| {
                if vis[vert] {
                    return;
                }
                vis.set(vert);
                if ans[vert] == -1 {
                    return;
                }
                ans[vert] = 2;
                for e in &graph[vert] {
                    dfs.call(e.to());
                }
            });
            dfs.call(i);
        }
    }
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
