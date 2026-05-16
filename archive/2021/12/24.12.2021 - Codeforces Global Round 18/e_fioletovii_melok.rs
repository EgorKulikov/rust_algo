//{"name":"E. Фиолетовый мелок","group":"Codeforces - Codeforces Global Round 18","url":"https://codeforces.com/contest/1615/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2\n1 2\n1 3\n1 4\n","output":"1\n"},{"input":"5 2\n1 2\n2 3\n3 4\n4 5\n","output":"6\n"},{"input":"7 2\n1 2\n1 3\n4 2\n3 5\n6 3\n6 7\n","output":"4\n"},{"input":"4 1\n1 2\n1 3\n1 4\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EFioletoviiMelok"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let k: usize = input.read();
    let edges = input.read_vec::<(usize, usize)>(n - 1).dec_by_one();

    let mut lens = Vec::new();
    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> usize {
        let mut res = 0;
        for e in graph[vert].iter() {
            if e.to() == prev {
                continue;
            }
            let call = f.call(e.to(), vert);
            if call > res {
                if res != 0 {
                    lens.push(res);
                }
                res = call;
            } else {
                lens.push(call);
            }
        }
        res += 1;
        res
    });
    let root_len = dfs.call(0, 0);
    lens.push(root_len);
    lens.sort_unstable();
    lens.reverse();
    if k >= lens.len() {
        let mut ans = lens.len();
        while ans < k && ans * (n - ans) < (ans + 1) * (n - ans - 1) {
            ans += 1;
        }
        out_line!(ans * (n - ans));
        return;
    }
    let r = k;
    let mut b = 0;
    for i in lens.into_iter().skip(k) {
        b += i;
    }
    if r >= b {
        out_line!((r - b) * (n - r - b));
        return;
    }
    let mut w = n - r - b;
    while b > r && (b - r - 1) * (w + 1) > (b - r) * w {
        b -= 1;
        w += 1;
    }
    out_line!(-(((b - r) * w) as isize));
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
