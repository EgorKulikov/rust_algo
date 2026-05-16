//{"name":"P6 - Rex","group":"DMOJ - Back to School '24","url":"https://dmoj.ca/problem/bts24p6","interactive":false,"timeLimit":2000,"tests":[{"input":"5 2 3\n1 5\n1 2 2 3\n1 3\n2 3\n7 0\n","output":"6\n5\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P6Rex"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::BackwardSlice;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::hl_decomposition::HLDecomposition;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{
    Callable2, Callable3, RecursiveFunction2, RecursiveFunction3,
};
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let q = input.read_size();
    let t = input.read_size_vec(k).dec().sorted();
    let p = input.read_size_vec(n - 1).dec();
    let mut graph = Graph::new(n);
    for i in 0..n - 1 {
        graph.add_edge(BiEdge::new(i + 1, p[i]));
    }
    let mut closest = vec![None; n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        if t.bin_search(&vert).is_some() {
            closest[vert] = Some((0, vert));
        }
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            if let Some((dist, v)) = f.call(e.to(), vert) {
                closest[vert].minim((dist + 1, v));
            }
        }
        closest[vert]
    });
    dfs.call(0, n);
    let mut dfs = RecursiveFunction3::new(
        |f, vert: usize, prev: usize, from_up: Option<(usize, usize)>| {
            if let Some((dist, v)) = from_up {
                closest[vert].minim((dist + 1, v));
            }
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                f.call(e.to(), vert, closest[vert]);
            }
        },
    );
    dfs.call(0, n, None);
    let closest = closest.into_iter().map(Option::unwrap).collect_vec();
    let lca = graph.lca();
    let (paths, id, pos) = graph.hl_decomposition();
    let mut up = Vec::with_capacity(paths.len());
    let mut down = Vec::with_capacity(paths.len());
    for i in paths.indices() {
        let mut c_up = vec![0; paths[i].len()];
        let mut c_down = vec![0; paths[i].len()];
        let last = paths[i].backward()[0];
        let (mut dist_last, mut closest_last) = closest[last];
        c_up.backward_mut()[0] = (dist_last * 2) as i64;
        for j in paths[i].indices().rev().skip(1) {
            let (dist_cur, closest_cur) = closest[paths[i][j]];
            if closest_last != closest_cur || dist_cur > dist_last {
                c_up[j] = (2 * dist_cur) as i64 + 1;
            } else {
                c_up[j] = -1;
            }
            closest_last = closest_cur;
            dist_last = dist_cur;
        }
        let first = paths[i][0];
        let (mut dist_last, mut closest_last) = closest[first];
        c_down[0] = (dist_last * 2) as i64;
        for j in paths[i].indices().skip(1) {
            let (dist_cur, closest_cur) = closest[paths[i][j]];
            if closest_last != closest_cur || dist_cur > dist_last {
                c_down[j] = (2 * dist_cur) as i64 + 1;
            } else {
                c_down[j] = -1;
            }
            closest_last = closest_cur;
            dist_last = dist_cur;
        }
        up.push(c_up.partial_sums());
        down.push(c_down.partial_sums());
    }

    let mut last_ans = 0;
    for _ in 0..q {
        let mut x = (input.read_size() ^ last_ans) - 1;
        let mut y = (input.read_size() ^ last_ans) - 1;

        let mut ans = (2 * closest[x].0) as i64;
        let l = lca.lca(x, y);
        loop {
            if id[x] == id[l] {
                ans += up[id[x]][pos[x]] - up[id[l]][pos[l]];
                break;
            }
            ans += up[id[x]][pos[x]];
            x = paths[id[x]][0];
            let (dist, cl) = closest[x];
            let par = lca.parent(x).unwrap();
            let (dist_par, cl_par) = closest[par];
            if cl_par != cl || dist_par > dist {
                ans += (2 * dist_par) as i64 + 1;
            } else {
                ans += -1;
            }
            x = par;
        }
        if y != l {
            loop {
                if id[y] == id[l] {
                    ans += down[id[y]][pos[y] + 1] - down[id[l]][pos[l] + 1];
                    break;
                }
                ans += down[id[y]][pos[y] + 1] - down[id[y]][1];
                y = paths[id[y]][0];
                let par = lca.parent(y).unwrap();
                let (dist_par, cl_par) = closest[par];
                let (dist, cl) = closest[y];
                if cl != cl_par || dist > dist_par {
                    ans += (2 * dist) as i64 + 1;
                } else {
                    ans += -1;
                }
                y = par;
            }
        }
        out.print_line(ans);
        last_ans = ans as usize;
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
