//{"name":"E - Reversi","group":"AtCoder - AtCoder Regular Contest 143","url":"https://atcoder.jp/contests/arc143/tasks/arc143_e","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 2\n2 3\n3 4\nWBWW\n","output":"1 2 4 3\n"},{"input":"4\n1 2\n2 3\n3 4\nBBBB\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EReversi"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::collections::{BTreeSet, HashSet};

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let edges = input.read_usize_pair_vec(n - 1).dec_by_one();
    let mut s: Str = input.read();

    let qw = s.iter().filter(|&it| it == b'W').count();
    if qw % 2 == 0 {
        out_line!(-1);
        return;
    }
    let mut bad = vec![HashSet::new(); n];
    let mut white = BTreeSet::new();
    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let mut qty = vec![0; n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        if s[vert] == b'W' {
            qty[vert] += 1;
        }
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert);
            if qty[e.to()] % 2 == 1 {
                bad[vert].insert(e.to());
            }
            qty[vert] += qty[e.to()];
        }
        if vert != 0 && (qw - qty[vert]) % 2 == 1 {
            bad[vert].insert(prev);
        }
        if bad[vert].is_empty() {
            if s[vert] == b'W' {
                white.insert(vert);
            }
        }
    });
    dfs.call(0, 0);
    let mut ans = Vec::with_capacity(n);
    let mut done = BitSet::new(n);
    for _ in 0..n {
        let cur = *white.iter().next().unwrap();
        done.set(cur, true);
        white.remove(&cur);
        ans.push(cur + 1);
        for e in &graph[cur] {
            if done[e.to()] {
                continue;
            }
            bad[e.to()].remove(&cur);
            if s[e.to()] == b'W' {
                s[e.to()] = b'B';
            } else {
                s[e.to()] = b'W';
            }
            if bad[e.to()].is_empty() {
                if s[e.to()] == b'W' {
                    white.insert(e.to());
                } else {
                    white.remove(&e.to());
                }
            }
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
