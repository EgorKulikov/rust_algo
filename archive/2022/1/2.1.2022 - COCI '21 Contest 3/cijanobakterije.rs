//{"name":"#2 - Cijanobakterije","group":"DMOJ - COCI '21 Contest 3","url":"https://dmoj.ca/problem/coci21c3p2","interactive":false,"timeLimit":1000,"tests":[{"input":"100 0\n","output":"100\n"},{"input":"8 6\n1 2\n1 3\n1 4\n5 6\n5 7\n5 8\n","output":"6\n"},{"input":"6 5\n1 2\n2 3\n3 4\n4 6\n4 5\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Cijanobakterije"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let m = input.read_usize();
    let edges = input.read_vec::<(usize, usize)>(m).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    fn calc_dist(graph: &Graph<BiEdge>, dist: &mut [u32], source: usize) -> Vec<usize> {
        dist[source] = 0;
        let mut queue = vec![source];
        let mut at = 0;
        while at < queue.len() {
            for e in graph[queue[at]].iter() {
                if dist[e.to()] == u32::MAX {
                    dist[e.to()] = dist[queue[at]] + 1;
                    queue.push(e.to());
                }
            }
            at += 1;
        }
        queue
    }
    let mut ans = 0;
    let mut dist = vec![u32::MAX; n];
    for i in 0..n {
        if dist[i] == u32::MAX {
            let q = calc_dist(&graph, dist.as_mut_slice(), i);
            let mut v = 0;
            let mut max = None;
            for j in q {
                if max.maxim(dist[j]) {
                    v = j;
                }
                dist[j] = u32::MAX;
            }
            let q = calc_dist(&graph, dist.as_mut_slice(), v);
            let mut max = None;
            for j in q {
                max.maxim(dist[j]);
            }
            ans += max.unwrap() + 1;
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
