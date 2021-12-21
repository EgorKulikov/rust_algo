//{"name":"P5 - Graph Generator","group":"DMOJ - DMOPC '21 Contest 4","url":"https://dmoj.ca/problem/dmopc21c4p5","interactive":false,"timeLimit":2000,"tests":[{"input":"2 1 2\n1 1\n1 2\n","output":"35\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P5GraphGenerator"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::{BaseModInt, ModInt7};
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let k = input.read();
    let p: Vec<u8> = input.read_vec(n);
    let edges = input.read_vec::<(usize, usize)>(m).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }

    type Mod = ModInt7;
    let mut inner_num_prod = Mod::zero();
    let mut inner_sum_prod = Mod::zero();
    let mut inner_sum = Mod::zero();
    let mut sum_root = Mod::zero();
    let mut sum_prod_root = Mod::zero();
    let mut inner_sum_bet_prod = Mod::zero();

    for i in 0..n {
        let dst = graph.edge_distances(i);
        sum_root += Mod::new(dst[0] as i32);
        if p[i] == 1 {
            inner_num_prod += Mod::one();
            sum_prod_root += Mod::new(dst[0] as i32);
        }
        for (k, j) in dst.into_iter().enumerate() {
            inner_sum += Mod::new(j as i32);
            if p[i] == 1 {
                inner_sum_prod += Mod::new(j as i32);
                if p[k] == 1 {
                    inner_sum_bet_prod += Mod::new(j as i32);
                }
            }
        }
    }
    inner_sum /= Mod::new(2);
    inner_sum_bet_prod /= Mod::new(2);

    let n = Mod::new(n as i32);
    let mut num_vert = n;
    let mut num_prod = inner_num_prod;
    let mut sum_prod = inner_sum_prod;
    let mut sum_bet_prod = inner_sum_bet_prod;
    let mut sum = inner_sum;

    for _ in 1..k {
        let a1 = num_prod * inner_sum;
        let a2 = (n + sum_root) * num_prod * num_vert;
        let a3 = sum_prod * n;
        let a4 = sum_bet_prod * n * n;
        let a5 = n * (n + sum_root) * num_prod * (num_prod - Mod::one());
        sum += a1 + a2 + a3 + a4 + a5;
        let b1 = num_prod * inner_sum_prod;
        let b2 = (sum_prod_root + inner_num_prod) * num_prod * num_vert;
        let b3 = sum_prod * inner_num_prod;
        let b4 = sum_bet_prod * inner_num_prod * n * Mod::new(2);
        let b5 = (inner_num_prod + sum_prod_root) * n * num_prod * (num_prod - Mod::one());
        let b6 = (sum_root + n) * inner_num_prod * num_prod * (num_prod - Mod::one());
        sum_prod = b1 + b2 + b3 + b4 + b5 + b6;
        let c1 = sum_bet_prod * inner_num_prod * inner_num_prod;
        let c2 = num_prod * inner_sum_bet_prod;
        let c3 =
            (sum_prod_root + inner_num_prod) * num_prod * (num_prod - Mod::one()) * inner_num_prod;
        sum_bet_prod = c1 + c2 + c3;
        num_vert += num_prod * n;
        num_prod *= inner_num_prod;
    }

    out_line!(sum);
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
