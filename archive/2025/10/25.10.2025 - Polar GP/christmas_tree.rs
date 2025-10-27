//{"name":"christmas_tree","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
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
    let a = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let root = graph.centers()[0];
    let mut sz = vec![0; n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        sz[vert] = 1;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            sz[vert] += f.call(e.to(), vert);
        }
        sz[vert]
    });
    dfs.call(root, n);
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (Vec<i64>, Vec<i64>) {
        let mut res = Arr2d::new(n + 1, sz[vert] + 1, i64::MAX / 2);
        for i in 1..=n {
            res[(i, 1)] = a[vert] * (i as i64);
        }
        let special = graph[vert].len() == 2 && prev != n;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let (down, up) = f.call(e.to(), vert);
            for i in 1..=n {
                for j in (1..=i.min(sz[vert])).rev() {
                    res[(i, j)] += down[i];
                    res[(i, j)].minim(i64::MAX / 2);
                    let mut from = 1;
                    let mut to = sz[e.to()].min(j);
                    if special {
                        from.maxim(j - 1);
                        to.minim(j - 1);
                    }
                    for k in from..=to {
                        let cand = res[(i, j - k)] + up[k];
                        res[(i, j)].minim(cand);
                    }
                }
            }
        }
        let mut down = vec![i64::MAX / 2; n + 1];
        let mut up = vec![i64::MAX / 2; sz[vert] + 1];
        for i in 1..=sz[vert] {
            up[i] = res[(i, i)];
        }
        for i in 1..=n {
            for j in 1..=i.min(sz[vert]) {
                down[i - j].minim(res[(i, j)]);
            }
        }
        (down, up)
    });
    let (_, up) = dfs.call(root, n);
    out.print_line(up.copy_min());
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
