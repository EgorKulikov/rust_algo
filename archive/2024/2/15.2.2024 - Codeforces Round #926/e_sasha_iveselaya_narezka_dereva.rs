//{"name":"E. Саша и веселая нарезка дерева","group":"Codeforces - Codeforces Round 926 (Div. 2)","url":"https://codeforces.com/contest/1929/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n1 2\n2 3\n2 4\n2\n1 3\n4 1\n6\n1 2\n3 1\n6 1\n5 2\n4 2\n3\n3 1\n3 6\n2 6\n5\n1 2\n2 3\n3 4\n4 5\n4\n1 2\n2 3\n3 4\n4 5\n","output":"1\n2\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ESashaIVeselayaNarezkaDereva"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();
    let k = input.read_size();
    let paths = input.read_size_pair_vec(k).dec();

    let graph = Graph::from_biedges(n, &edges);
    let mut on = vec![0; n];
    let mut off = vec![0; n];
    let lca = graph.lca();
    for (i, (u, v)) in paths.into_iter().enumerate() {
        on[u].set_bit(i);
        on[v].set_bit(i);
        off[lca.lca(u, v)].set_bit(i);
    }
    let mut edges = Vec::new();
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> usize {
        let mut res = on[vert];
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let call = f.call(e.to(), vert);
            if (call & off[vert]) != 0 {
                edges.push(call);
            }
            res |= call;
        }
        res &= !off[vert];
        res
    });
    assert_eq!(dfs.call(0, n), 0);
    let mut ans = vec![None; 1 << k];
    ans[0] = Some(0);
    for i in usize::iter_all(k) {
        if let Some(val) = ans[i] {
            for &j in &edges {
                ans[i | j].minim(val + 1);
            }
        }
    }
    out.print_line(ans[usize::all_bits(k)]);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
