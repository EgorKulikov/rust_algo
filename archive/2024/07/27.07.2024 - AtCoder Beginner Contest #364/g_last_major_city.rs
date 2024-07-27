//{"name":"G - Last Major City","group":"AtCoder - Japan Registry Services (JPRS) Programming Contest 2024#2 (AtCoder Beginner Contest 364)","url":"https://atcoder.jp/contests/abc364/tasks/abc364_g","interactive":false,"timeLimit":5000,"tests":[{"input":"4 5 3\n1 4 3\n3 4 4\n1 2 4\n2 3 2\n1 3 1\n","output":"3\n6\n"},{"input":"4 3 2\n2 4 28\n1 4 56\n1 3 82\n","output":"84\n82\n56\n"},{"input":"6 12 4\n2 6 68\n2 5 93\n4 6 28\n2 4 89\n3 6 31\n1 3 10\n1 2 53\n3 5 1\n3 5 74\n3 4 22\n4 5 80\n3 4 35\n","output":"85\n64\n94\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GLastMajorCity"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();

    let mut graph = Graph::new(n);
    for (u, v, c) in edges {
        graph.add_edge(BiWeightedEdge::new(u, v, c));
    }
    let mut ans = Arr2d::new(1 << (k - 1), n, i64::MAX / 2);
    for i in 0..k - 1 {
        ans[(1 << i, i)] = 0;
    }
    for i in 1..1 << (k - 1) {
        let mut queue = BTreeSet::new();
        for j in 0..n {
            let mut set = i;
            while set != 0 {
                let cand = ans[(set, j)] + ans[(i - set, j)];
                ans[(i, j)].minim(cand);
                set = (set - 1) & i;
            }
            queue.insert((ans[(i, j)], j));
        }
        while let Some((cur, vert)) = queue.pop_first() {
            for e in &graph[vert] {
                let cand = cur + e.weight();
                if ans[(i, e.to())] > cand {
                    queue.remove(&(ans[(i, e.to())], e.to()));
                    ans[(i, e.to())] = cand;
                    queue.insert((ans[(i, e.to())], e.to()));
                }
            }
        }
    }
    for i in k - 1..n {
        out.print_line(ans[(usize::all_bits(k - 1), i)]);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
}
//END MAIN
