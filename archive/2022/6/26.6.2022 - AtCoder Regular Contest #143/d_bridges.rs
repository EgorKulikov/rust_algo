//{"name":"D - Bridges","group":"AtCoder - AtCoder Regular Contest 143","url":"https://atcoder.jp/contests/arc143/tasks/arc143_d","interactive":false,"timeLimit":2000,"tests":[{"input":"2 2\n1 1\n2 2\n","output":"01\n"},{"input":"6 7\n1 1 2 3 4 4 5\n2 3 3 4 5 6 6\n","output":"0100010\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBridges"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdgeWithId;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_usize_vec(m).dec_by_one();
    let b = input.read_usize_vec(m).dec_by_one();

    let mut graph = Graph::new(n);
    for (&u, &v) in a.iter().zip(b.iter()) {
        graph.add_edge(u, BiEdgeWithId::new(v));
    }
    let mut done = BitSet::new(n);
    let mut on_stack = BitSet::new(n);
    let mut ans = Str::from(vec![b'0'; m]);
    let mut edges = BitSet::new(m);
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        if done[vert] {
            return;
        }
        done.set(vert, true);
        on_stack.set(vert, true);
        for e in &graph[vert] {
            if e.id() == prev || edges[e.id()] {
                continue;
            }
            assert!(!edges[e.id()]);
            edges.set(e.id(), true);
            if b[e.id()] == vert {
                ans[e.id()] = b'1';
            }
            f.call(e.to(), e.id());
        }
        on_stack.set(vert, false);
    });
    for i in 0..n {
        dfs.call(i, m);
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
