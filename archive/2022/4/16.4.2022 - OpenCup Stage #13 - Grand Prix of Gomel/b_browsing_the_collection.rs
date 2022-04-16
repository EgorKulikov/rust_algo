//{"name":"B. Browsing The Collection","group":"Yandex - Stage 13: Grand Prix of Gomel","url":"https://official.contest.yandex.com/opencupXXII/contest/35270/problems/B/","interactive":false,"timeLimit":4000,"tests":[{"input":"9 3\n5 3 7\n5 3 4\n5 3 7\n5 3 2\n5 3 4\n5 3 7\n2 3 7\n5 3 7\n2 3 7\n","output":"0 1 2 1 2 3 1 2 1\n1 0 1 1 2 2 1 3 2\n2 1 0 1 1 2 1 3 2\n3 2 1 0 1 1 1 3 2\n3 2 2 1 0 1 1 3 2\n3 1 2 1 1 0 1 2 2\n2 1 3 1 2 1 0 1 2\n2 1 3 1 2 2 1 0 1\n1 1 3 1 2 3 2 1 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBrowsingTheCollection"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_table::<usize>(n, m);

    let mut graph = Graph::new(n << m);
    let mut mask = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            mask[j] = 0;
            for k in 0..m {
                if a[(i, k)] == a[(j, k)] {
                    mask[j].set_bit(k);
                }
            }
        }
        for j in 0..(1 << m) {
            for k in 0..m {
                if j.is_set(k) {
                    graph.add_edge((i << m) + j, Edge::new((i << m) + j.without_bit(k)));
                } else {
                    let mut was = HashSet::new();
                    for l in 0..n {
                        let id = (l + i) % n;
                        if (mask[id] & j) == j && !was.contains(&a[(id, k)]) {
                            was.insert(a[(id, k)]);
                            graph.add_edge((i << m) + j, Edge::new((id << m) + j.with_bit(k)));
                        }
                    }
                }
            }
            for k in 1..n {
                let id = (k + i) % n;
                if (mask[id] & j) == j {
                    graph.add_edge((i << m) + j, Edge::new((id << m) + j));
                    graph.add_edge((id << m) + j, Edge::new((i << m) + j));
                    break;
                }
            }
        }
    }
    let mut ans = Arr2d::new(n, n, 0);
    for i in 0..n {
        let dist = graph.edge_distances(i << m);
        for j in 0..n {
            ans[(i, j)] = dist[(j << m)..((j + 1) << m)]
                .iter()
                .cloned()
                .min()
                .unwrap();
        }
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
