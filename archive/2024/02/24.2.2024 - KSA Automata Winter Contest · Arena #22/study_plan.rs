//{"name":"Study Plan","group":"Baekjoon Online Judge","url":"https://www.acmicpc.net/contest/problem/1238/5","interactive":false,"timeLimit":2000,"tests":[{"input":"3 12 15\n1 2 3\n3 5 7\n","output":"9\n"},{"input":"3 19 19\n2 3 3\n1 2 4\n","output":"14\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"StudyPlan"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_size();
    let y = input.read_size();
    let a = input.read_size_vec(n).dec();
    let b = input.read_size_vec(n);

    let mut graph = Graph::new(n);
    for i in 0..n {
        graph.add_edge(Edge::new(a[i], i));
    }
    let mut dist = Vec::with_capacity(n);
    for i in 0..n {
        dist.push(graph.edge_distances(i));
    }
    let mut dsu = DSU::new(n);
    for i in 0..n {
        for j in 0..i {
            if dist[i][j] != u32::MAX && dist[j][i] != u32::MAX {
                dsu.join(i, j);
            }
        }
    }
    let mut delta = Vec::new();
    for i in 0..n {
        if dsu.get(i) != i {
            continue;
        }
        let mut cur = 0;
        for j in 0..n {
            if dist[i][j] != u32::MAX {
                cur += b[j];
            }
        }
        delta.push(cur);
    }
    type Mod = ModIntF;
    let mut ways = vec![Mod::zero(); y + 1];
    ways[0] = Mod::one();
    for i in delta {
        for j in i..=y {
            let add = ways[j - i];
            ways[j] += add;
        }
    }
    let ans = ways.into_iter().skip(x).fold(Mod::zero(), |a, b| a + b);
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
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
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
