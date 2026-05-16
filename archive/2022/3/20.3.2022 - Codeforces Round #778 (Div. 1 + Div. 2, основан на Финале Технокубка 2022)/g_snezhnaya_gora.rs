//{"name":"G. Снежная гора","group":"Codeforces - Codeforces Round #778 (Div. 1 + Div. 2, основан на Финале Технокубка 2022)","url":"https://codeforces.com/contest/1654/problem/G","interactive":false,"timeLimit":5000,"tests":[{"input":"6\n1 1 0 0 0 0\n1 3\n2 4\n3 4\n4 5\n5 6\n","output":"0 0 1 1 3 5\n"},{"input":"9\n0 0 0 0 0 0 1 1 1\n1 3\n2 3\n2 5\n3 6\n4 5\n4 7\n5 8\n6 9\n","output":"5 3 2 1 1 1 0 0 0\n"},{"input":"14\n0 0 0 0 0 0 0 0 0 1 1 1 1 1\n1 2\n2 5\n3 4\n4 5\n3 6\n4 8\n5 9\n7 8\n6 11\n7 12\n8 13\n9 14\n10 11\n","output":"8 5 4 3 2 2 1 1 1 0 0 0 0 0\n"},{"input":"20\n0 0 0 0 0 0 0 0 0 0 0 0 0 1 1 1 0 1 0 1\n17 3\n11 12\n6 10\n18 19\n8 14\n16 20\n5 3\n2 11\n7 10\n2 15\n8 3\n3 15\n9 16\n7 13\n16 1\n19 2\n2 16\n6 1\n4 17\n","output":"2 2 1 5 3 4 8 1 2 6 4 6 10 0 0 0 3 0 1 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GSnezhnayaGora"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, VecDeque};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let l = input.read_usize_vec(n);
    let edges = input.read_usize_pair_vec(n - 1).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let mut h = vec![n; n];
    let mut q = VecDeque::new();
    for (i, l) in l.into_iter().enumerate() {
        if l == 1 {
            h[i] = 0;
            q.push_back(i);
        }
    }
    while let Some(v) = q.pop_front() {
        for e in &graph[v] {
            if h[e.to()] == n {
                h[e.to()] = h[v] + 1;
                q.push_back(e.to());
            }
        }
    }
    let mut int = vec![Vec::new(); n];
    for (i, &hh) in h.iter().enumerate() {
        for e in &graph[i] {
            if h[e.to()] == hh {
                int[hh].push(i);
                break;
            }
        }
    }
    let mut delta = h.clone();
    let mut heap = BinaryHeap::new();
    for (i, v) in int.into_iter().enumerate() {
        let mut set = HashSet::new();
        for j in v {
            heap.push((Reverse(h[j]), Reverse(0usize), j));
        }
        while let Some((_, need, vert)) = heap.pop() {
            if set.contains(&vert) || delta[vert] < i {
                continue;
            }
            set.insert(vert);
            if need.0 == 0 {
                delta[vert] = i;
            }
            for e in &graph[vert] {
                if h[e.to()] == h[vert] {
                    heap.push((Reverse(h[e.to()]), Reverse(need.0 + 1), e.to()));
                } else if h[e.to()] > h[vert] {
                    heap.push((
                        Reverse(h[e.to()]),
                        Reverse(need.0.saturating_sub(1)),
                        e.to(),
                    ));
                }
            }
        }
    }
    out_line!(h
        .into_iter()
        .zip(delta.into_iter())
        .map(|(a, b)| 2 * a - b)
        .collect_vec());
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
