//{"name":"#3 - Magenta","group":"DMOJ - COCI '20 Contest 5","url":"https://dmoj.ca/problem/coci20c5p3","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 3\n3 2 magenta\n2 1 magenta\n","output":"Paula\n"},{"input":"5\n3 5\n1 2 magenta\n1 3 magenta\n2 4 plava\n2 5 crvena\n","output":"Marin\n"},{"input":"5\n1 4\n2 1 plava\n1 3 crvena\n5 2 plava\n4 1 magenta\n","output":"Magenta\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Magenta"}}}

use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;
use std::mem::swap;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let mut a = input.read_usize() - 1;
    let mut b = input.read_usize() - 1;
    let edges: Vec<(usize, usize, String)> = input.read_vec(n - 1);

    let mut graph = Graph::new(n);
    for (u, v, color) in edges {
        graph.add_edge(
            u - 1,
            BiWeightedEdge::new(
                v - 1,
                if &color != "crvena" { 1 } else { 0 } + if &color != "plava" { 2 } else { 0 },
            ),
        );
    }
    let mut can_move_paula: Box<dyn Fn(&BiWeightedEdge<i32>) -> bool> =
        Box::new(|e| e.weight().is_set(0));
    let mut can_move_marin: Box<dyn Fn(&BiWeightedEdge<i32>) -> bool> =
        Box::new(|e| e.weight().is_set(1));
    let mut paula = "Paula";
    let mut marin = "Marin";
    if !graph[a].iter().any(|e| can_move_paula(e) && e.to() != b) {
        out_line!(marin);
        return;
    }
    if !graph[b].iter().any(&can_move_marin) {
        out_line!(paula);
        return;
    }
    let lca = graph.lca();
    while lca.path_length(a, b) > 1 {
        let cur_len = lca.path_length(a, b);
        let mut next = None;
        for e in graph[a].iter() {
            if cur_len > lca.path_length(e.to(), b) {
                if can_move_paula(e) {
                    next = Some(e.to());
                }
                break;
            }
        }
        match next {
            None => break,
            Some(new_a) => {
                a = b;
                b = new_a;
                swap(&mut paula, &mut marin);
                swap(&mut can_move_paula, &mut can_move_marin);
            }
        }
    }
    if lca.path_length(a, b) % 2 == 0 {
        out_line!("Magenta");
        return;
    }
    let mut c = a;
    while c != b {
        for e in &graph[c] {
            if lca.path_length(c, b) > lca.path_length(e.to(), b) {
                if !can_move_marin(e) {
                    out_line!("Magenta");
                    return;
                }
                c = e.to();
                break;
            }
        }
    }
    let mut dfs = RecursiveFunction3::new(|f, vert, prev, good| {
        for e in graph[vert].iter() {
            if e.to() == prev || !can_move_paula(e) {
                continue;
            }
            if good || f.call(e.to(), vert, !can_move_marin(e)) {
                return true;
            }
        }
        false
    });
    if dfs.call(a, b, false) {
        out_line!("Magenta");
    } else {
        out_line!(marin);
    }
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
