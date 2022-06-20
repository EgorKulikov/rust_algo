//{"name":"D - Deterministic Placing","group":"AtCoder - AtCoder Regular Contest 142","url":"https://atcoder.jp/contests/arc142/tasks/arc142_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 2\n1 3\n","output":"2\n"},{"input":"7\n1 2\n1 3\n2 4\n2 5\n3 6\n3 7\n","output":"0\n"},{"input":"19\n9 14\n2 13\n1 3\n17 19\n13 18\n12 19\n4 5\n2 10\n4 9\n8 11\n3 15\n6 8\n8 10\n6 19\n9 13\n11 15\n7 17\n16 17\n","output":"100\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDeterministicPlacing"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let edges = input.read_usize_pair_vec(n - 1).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    type Mod = ModIntF;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (Mod, Mod, Mod) {
        let mut mid_0 = Mod::one();
        let mut mid_1 = Mod::zero();
        let mut mid_2 = Mod::zero();
        let mut begin_0 = Mod::one();
        let mut begin_1 = Mod::zero();
        let mut up_0 = Mod::one();
        let mut up_1 = Mod::zero();
        let mut up_end = Mod::one();
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let (call_mid, call_begin, call_up) = f.call(e.to(), vert);
            mid_2 = mid_2 * call_mid + mid_1 * call_up;
            mid_1 = mid_1 * call_mid + mid_0 * call_up;
            mid_0 = mid_0 * call_mid;
            begin_1 = begin_1 * call_begin + begin_0 * call_up;
            begin_0 = begin_0 * call_begin;
            up_1 = up_1 * call_mid + up_0 * call_up;
            up_0 = up_0 * call_mid;
            up_end *= call_begin;
        }
        (mid_2 * Mod::new(2), begin_1, up_1 + up_end)
    });
    let (mid, begin, _) = dfs.call(0, 0);
    out_line!(mid + begin * Mod::new(2));
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
