//{"name":"E. Новогодние конфеты","group":"Codeforces - Codeforces Round #761 (Div. 2)","url":"https://codeforces.com/contest/1617/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n5 6 7 8 9\n","output":"2 5 5\n"},{"input":"2\n4 8\n","output":"1 2 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ENovogodnieKonfeti"}}}

use algo_lib::collections::id::Id;
use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a: Vec<u32> = input.read_vec(n);

    let mut id = Id::new();
    id.get(0);
    for mut i in a.iter().cloned() {
        while i != 0 {
            id.get(i);
            i = i.next_power_of_two() - i;
        }
    }
    let mut graph = Graph::new(id.len());
    for i in id.by_id() {
        if i != 0 {
            graph.add_edge(id.get(i), BiEdge::new(id.get(i.next_power_of_two() - i)));
        }
    }
    let mut depth = vec![(0, 0); graph.vertex_count()];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (i32, usize) {
        let mut res = (0, vert);
        for e in graph[vert].iter() {
            if e.to() != prev {
                let mut call = f.call(e.to(), vert);
                call.0 += 1;
                res.maxim(call);
            }
        }
        depth[vert] = res;
        res
    });
    dfs.call(id.get(0), id.get(0));
    let mut ans = 0;
    let (mut from, mut to) = (a[0], a[0]);
    let by_id = id.by_id();
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        let mut down = 0;
        let mut down_vert = vert;
        for e in graph[vert].iter() {
            if e.to() != prev {
                let cand = down + depth[e.to()].0 + 1;
                if cand > ans {
                    ans = cand;
                    from = by_id[down_vert];
                    to = by_id[depth[e.to()].1];
                }
                let cand = depth[e.to()].0 + 1;
                if cand > down {
                    down = cand;
                    down_vert = depth[e.to()].1;
                }
                f.call(e.to(), vert);
            }
        }
    });
    dfs.call(id.get(0), id.get(0));
    let x = a.iter().find(&from).unwrap() + 1;
    let y = a.iter().find(&to).unwrap() + 1;
    out_line!(x, y, ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
