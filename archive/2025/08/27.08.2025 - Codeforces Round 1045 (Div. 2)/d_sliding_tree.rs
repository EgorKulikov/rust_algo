//{"name":"D. Sliding Tree","group":"Codeforces - Codeforces Round 1045 (Div. 2)","url":"https://codeforces.com/contest/2134/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n6\n4 3\n3 5\n3 1\n1 2\n3 6\n1\n2\n1 2\n5\n5 4\n2 3\n4 2\n1 4\n","output":"4 3 5\n-1\n-1\n2 4 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    if n <= 1 {
        out.print_line(-1);
        return;
    }
    let graph = Graph::with_biedges(n, &edges);
    let mut d1 = vec![0; n];
    let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, mut dd| {
        dd += 1;
        d1[vert] = dd;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert, dd);
        }
    });
    let mut start = n;
    for i in 0..n {
        if graph[i].len() == 1 {
            start = i;
            break;
        }
    }
    dfs.call(start, n, 0);
    assert!(start < n);
    let mut end = n;
    let max = d1.copy_max();
    for i in 0..n {
        if d1[i] == max && graph[i].len() == 1 {
            end = i;
            break;
        }
    }
    assert!(end < n);
    let mut d2 = vec![0; n];
    let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, mut dd| {
        dd += 1;
        d2[vert] = dd;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert, dd);
        }
    });
    dfs.call(end, n, 0);
    let max = d2.copy_max();
    if max >= n {
        out.print_line(-1);
        return;
    }
    if max == 0 {
        for i in 0..n {
            if graph[i].len() == 1 {
                let a = i;
                let b = graph[i][0].to();
                let mut c = n;
                for e in &graph[b] {
                    if e.to() != a {
                        c = e.to();
                        break;
                    }
                }
                out.print_line((c + 1, b + 1, a + 1));
                return;
            }
        }
        unreachable!();
    }
    let mut start = n;
    for i in 0..n {
        if d2[i] == max && graph[i].len() == 1 && i != end {
            start = i;
            break;
        }
    }
    assert!(start < n);
    let lca = graph.lca();
    for i in 1.. {
        let vert = lca.nth_vert_on_path(start, end, i);
        if graph[vert].len() != 2 {
            let mut to = n;
            let prev = lca.nth_vert_on_path(start, end, i - 1);
            let next = lca.nth_vert_on_path(start, end, i + 1);
            for e in &graph[vert] {
                if e.to() != prev && e.to() != next {
                    to = e.to();
                    break;
                }
            }
            assert!(to < n);
            out.print_line((next + 1, vert + 1, to + 1));
            return;
        }
    }

    unreachable!();
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
