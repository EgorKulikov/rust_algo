//{"name":"T637348 「Diligent-OI R2 D」在水一方","group":"Luogu","url":"https://www.luogu.com.cn/problem/T637348?contestId=241168","interactive":false,"timeLimit":1000,"tests":[{"input":"12 4\n10 10\n9 7\n13 9\n5 6\n3 4\n7 4\n10 4\n11 4\n13 4\n5 1\n8 1\n10 2\n1 2\n1 3\n2 4\n4 5\n4 6\n3 7\n3 8\n3 9\n6 10\n8 11\n8 12\n9\n20\n45\n1\n","output":"7 12\n7 11\n10 11\n7 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let xy = input.read_long_pair_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let d = Arr2d::with_gen(n, n, |i, j| {
        let (x0, y0) = xy[i];
        let (x1, y1) = xy[j];
        let dx = x0 - x1;
        let dy = y0 - y1;
        dx * dx + dy * dy
    });
    let lca = graph.lca();
    let mut min_d = Memoization2d::new(n, n, |mem, u, v| {
        if u == v {
            0
        } else {
            let mut was = i64::MAX;
            if u != lca.lca(u, v) {
                was.minim(mem.call(lca.parent(u).unwrap(), v));
            }
            if v != lca.lca(u, v) {
                was.minim(mem.call(u, lca.parent(v).unwrap()));
            }
            if u != lca.lca(u, v) && v != lca.lca(u, v) {
                was.minim(mem.call(lca.parent(u).unwrap(), lca.parent(v).unwrap()));
            }
            was.max(d[(u, v)])
        }
    });
    let mut d_root = vec![0; n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            d_root[e.to()] = d_root[vert] + d[(vert, e.to())];
            f.call(e.to(), vert);
        }
    });
    dfs.call(0, n);
    let mut variants = Vec::with_capacity(n * n);
    for i in 0..n {
        for j in 0..n {
            variants.push((
                min_d.call(i, j),
                d_root[i] + d_root[j] - 2 * d_root[lca.lca(i, j)],
                Reverse(i),
                Reverse(j),
            ));
        }
    }
    variants.sort();
    let mut best = (0, Reverse(n), Reverse(n));
    let mut good = Vec::new();
    for (d, val, i, j) in variants {
        let cur = (val, i, j);
        if best.maxim(cur) {
            good.push((d, val, i, j));
        }
    }

    for _ in 0..q {
        let d = input.read_long();
        let pos = good.lower_bound(&(d, i64::MAX, Reverse(0), Reverse(0)));
        let (_, _, Reverse(i), Reverse(j)) = good[pos - 1];
        out.print_line((i + 1, j + 1));
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
