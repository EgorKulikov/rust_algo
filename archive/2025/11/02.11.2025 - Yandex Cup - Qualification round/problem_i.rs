//{"name":"problem_i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{
    Callable2, Callable3, RecursiveFunction2, RecursiveFunction3,
};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    if n == 1 {
        out.print_line(1);
        out.print_line(0);
        return;
    }
    let graph = Graph::with_biedges(n, &edges);
    // let d = graph.edge_distances(0);
    // let pos = d.max_position();
    // let d = graph.edge_distances(pos);
    // let mut dd = vec![0; n];
    // let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
    //     dd[vert] = d[vert];
    //     for e in &graph[vert] {
    //         if e.to() == prev {
    //             continue;
    //         }
    //         dd[vert].maxim(f.call(e.to(), vert));
    //     }
    //     dd[vert]
    // });
    // dfs.call(pos, n);
    let mut res = vec![vec![0; n]; n];
    for i in 0..n {
        let mut dsu = DSU::new(n - 1);
        // let mut ans = vec![vec![0; n - 1]; n];
        let mut edge = vec![0; n];
        let mut is_opposite = Arr2d::new(n - 1, n - 1, false);
        let mut id = 0;
        let mut dfs = RecursiveFunction3::new(
            |f, vert: usize, prev: usize, direction: usize| -> Option<usize> {
                if prev != n {
                    edge[vert] = direction;
                    // for i in 0..n - 1 {
                    //     ans[vert][i] = ans[prev][i];
                    // }
                    // ans[vert][direction] += 1;
                }
                let mut directions = Vec::new();
                let mut additional = Vec::new();
                for e in &graph[vert] {
                    if e.to() == prev {
                        continue;
                    }
                    id += 1;
                    let cur = id - 1;
                    if let Some(add) = f.call(e.to(), vert, id - 1) {
                        additional.push(add);
                    } else {
                        additional.push(usize::MAX);
                    }
                    directions.push(cur);
                }
                if directions.len() == 0 {
                    return None;
                }
                if directions.len() == 1 {
                    dsu.union(direction, directions[0]);
                    if additional[0] != usize::MAX {
                        return Some(additional[0]);
                    } else {
                        return None;
                    }
                }
                let has_no = additional.contains(&usize::MAX);
                if has_no || prev == n {
                    let mut rem = Vec::new();
                    for i in directions.indices() {
                        let next = directions[(i + 1) % directions.len()];
                        if additional[i] != usize::MAX {
                            is_opposite[(additional[i], next)] = true;
                        } else {
                            rem.push(next);
                        }
                    }
                    if prev != n {
                        let dir = rem.pop().unwrap();
                        dsu.union(dir, direction);
                    }
                    while rem.len() >= 2 {
                        let a = rem.pop().unwrap();
                        let b = rem.pop().unwrap();
                        is_opposite[(a, b)] = true;
                    }
                    return rem.get(0).copied();
                }
                for i in 0..directions.len() - 1 {
                    let next = directions[(i + 1) % directions.len()];
                    is_opposite[(additional[i], next)] = true;
                }
                dsu.union(direction, directions[0]);
                Some(additional[Back(0)])
            },
        );
        dfs.call(i, n, 0);
        let mut opp = Arr2d::new(id, id, false);
        for i in 0..id {
            for j in 0..id {
                if is_opposite[(i, j)] {
                    opp[(dsu.find(i), dsu.find(j))] = true;
                    opp[(dsu.find(j), dsu.find(i))] = true;
                }
            }
        }
        let mut d_id = Vec::new();

        let mut next = 0;
        let mut delta = vec![0; id];
        let mut dir = vec![0; id];
        for j in 0..id {
            if dsu.find(j) == j {
                let mut found = false;
                for k in d_id.indices() {
                    if opp[(j, d_id[k])] {
                        d_id.push(d_id[k]);
                        dir[j] = dir[d_id[k]];
                        delta[j] = -1;
                        found = true;
                        break;
                    }
                }
                if !found {
                    d_id.push(j);
                    dir[j] = next;
                    next += 1;
                    delta[j] = 1;
                }
            }
        }
        if next < res[0].len() {
            res = vec![vec![0; next]; n];
            let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                if prev != n {
                    for i in 0..next {
                        res[vert][i] = res[prev][i];
                    }
                    res[vert][dir[dsu.find(edge[vert])]] += delta[dsu.find(edge[vert])];
                }
                for e in &graph[vert] {
                    if e.to() == prev {
                        continue;
                    }
                    f.call(e.to(), vert);
                }
            });
            dfs.call(i, n);
        }
    }
    out.print_line(res[0].len());
    out.print_per_line(&res);
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
