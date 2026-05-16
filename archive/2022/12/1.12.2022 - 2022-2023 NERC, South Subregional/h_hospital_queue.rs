//{"name":"H. Hospital Queue","group":"Codeforces - 2022-2023 ICPC, NERC, Южный четвертьфинал (онлайн-трансляция, правила ICPC, командное соревнование)","url":"https://codeforces.com/contest/1765/problem/H","interactive":false,"timeLimit":3000,"tests":[{"input":"4 1\n2 3 2 4\n3 1\n","output":"2 3 1 4\n"},{"input":"3 0\n3 3 3\n","output":"1 1 1\n"},{"input":"5 3\n4 3 3 2 5\n3 1\n1 5\n4 2\n","output":"4 2 1 1 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HHospitalQueue"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::permutation::Permutation;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::topological_sort::TopologicalSort;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out_line, when};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let mut p = input.read_size_vec(n);
    let edges = input.read_size_pair_vec(m).dec_by_one();

    let mut graph = Graph::new(n);
    for &(u, v) in &edges {
        graph.add_edge(u, Edge::new(v));
    }
    let order = graph.topological_sort().unwrap();
    let mut has_to_precede = (0..n)
        .map(|i| {
            let mut res = BitSet::new(n);
            res.set(i, true);
            res
        })
        .collect_vec();
    for &i in &order {
        for e in &graph[i] {
            let j = e.to();
            let (i, j) = if i < j {
                let (front, back) = has_to_precede.split_at_mut(j);
                (&front[i], &mut back[0])
            } else {
                let (front, back) = has_to_precede.split_at_mut(i);
                (&back[0], &mut front[j])
            };
            *j |= i;
        }
    }
    for &i in order.iter().rev() {
        for j in 0..n {
            if i != j && has_to_precede[i][j] {
                let cand = p[i] - 1;
                p[j].minim(cand);
            }
        }
    }
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| p[i]);
    let pos = Permutation::new(order.clone()).inv().to_vec();
    let mut ans = Vec::new();
    for i in 0..n {
        let mut pp = pos[i];
        let mut shift = 1;
        for j in (0..pp).rev() {
            when! {
                has_to_precede[i][order[j]] => shift += 1,
                j + shift >= p[order[j]] => break,
                else => pp -= 1,
            }
        }
        ans.push(pp + 1);
    }
    out_line!(ans);
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
