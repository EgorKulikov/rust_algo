//{"name":"I. Items and Heroes","group":"Yandex - Stage 10: Grand Prix of Kyoto","url":"https://official.contest.yandex.ru/opencupXXII/contest/35263/problems/I/","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 1\n2 1 3\n3 1 2\n2\n1 1 1\n2 3 1\n","output":"Yes\nNo\nYes\n"},{"input":"5\n1 2 1 3\n1000000000 1000000000 1000000000 1000000000 1000000000\n1 1 1 1 1\n1\n1 1 1\n","output":"Yes\nYes\n"},{"input":"5\n1 2 2 2\n109102235 645590056 708566822 497603443 131863700\n50073184 441114664 164994352 304489019 158100373\n8\n1 5 692234112\n1 3 610338520\n2 4 818442884\n2 4 164762830\n2 4 923652447\n2 4 197720766\n1 1 779302743\n1 1 222486377\n","output":"No\nYes\nYes\nNo\nYes\nNo\nYes\nYes\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IItemsAndHeroes"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::dfs_order::DFSOrder;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::hl_decomposition::HLDecomposition;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::{out_line, zip};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let p = input.read_usize_vec(n - 1).dec_by_one();
    let mut a = input.read_long_vec(n);
    let mut c = input.read_long_vec(n);

    let d = a.iter().zip(c.iter()).map(|(&a, &c)| a - c).collect_vec();
    let mut graph = Graph::new(n);
    for (i, p) in (1..n).zip(p.into_iter()) {
        graph.add_edge(i, BiEdge::new(p));
    }
    let order = graph.dfs_order();
    let mut ft = FenwickTree::new(n);
    for i in 0..n {
        ft.add(order.0[i], d[i]);
    }
    let (parts, part_id, vert_id) = graph.hl_decomposition();
    let mut val: Vec<Vec<i64>> = vec![Vec::new(); parts.len()];
    let mut delta: Vec<Vec<i64>> = vec![Vec::new(); parts.len()];
    let mut bad = 0;
    for (part, v, dlt) in zip!(parts.iter(), val.iter_mut(), delta.iter_mut()) {
        *dlt = vec![0; part.len() * 4];
        *v = vec![0; part.len() * 4];
        let mut init = RecursiveFunction3::new(|f, root: usize, left, right| {
            if left + 1 == right {
                v[root] = ft.get(order.0[part[left]], order.1[part[left]]);
            } else {
                let mid = (left + right) >> 1;
                f.call(2 * root + 1, left, mid);
                f.call(2 * root + 2, mid, right);
                v[root] = v[2 * root + 1].min(v[2 * root + 2]);
            }
        });
        init.call(0, 0, part.len());
        if v[0] < 0 {
            bad += 1;
        }
    }
    let lca = graph.lca();
    out_line!(if bad == 0 { "Yes" } else { "No" });
    let q = input.read_usize();
    for _ in 0..q {
        let t = input.read_usize();
        let mut u = input.read_usize() - 1;
        let v = input.read_long();
        let dlt = if t == 1 { v - a[u] } else { c[u] - v };
        if t == 1 {
            a[u] = v;
        } else {
            c[u] = v;
        }

        loop {
            let pid = part_id[u];
            if val[pid][0] < 0 {
                bad -= 1;
            }
            let mut update = RecursiveFunction3::new(|f, root: usize, left, right| {
                if left > vert_id[u] {
                    return;
                }
                if right <= vert_id[u] + 1 {
                    val[pid][root] += dlt;
                    delta[pid][root] += dlt;
                    return;
                }
                let c_delta = delta[pid][root];
                val[pid][2 * root + 1] += c_delta;
                val[pid][2 * root + 2] += c_delta;
                delta[pid][2 * root + 1] += c_delta;
                delta[pid][2 * root + 2] += c_delta;
                delta[pid][root] = 0;
                let mid = (left + right) >> 1;
                f.call(2 * root + 1, left, mid);
                f.call(2 * root + 2, mid, right);
                val[pid][root] = val[pid][2 * root + 1].min(val[pid][2 * root + 2]);
            });
            update.call(0, 0, parts[pid].len());
            if val[pid][0] < 0 {
                bad += 1;
            }
            match lca.parent(parts[pid][0]) {
                None => {
                    break;
                }
                Some(v) => {
                    u = v;
                }
            }
        }
        out_line!(if bad == 0 { "Yes" } else { "No" });
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
