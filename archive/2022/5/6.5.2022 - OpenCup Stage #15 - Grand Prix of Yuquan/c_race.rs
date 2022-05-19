//{"name":"C. Race","group":"Yandex - Stage 15: Grand Prix of Yuquan","url":"https://official.contest.yandex.com/opencupXXII/contest/37831/problems/C/","interactive":false,"timeLimit":1000,"tests":[{"input":"7 9 3 4\n1 2 1\n2 3 1\n3 1 2\n1 4 3\n5 6 2\n6 7 1\n6 7 3\n7 7 2\n5 5 1\n6 7\n1 4\n2 4\n2 5\n","output":"Yes\nNo\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRace"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, BoolOutput};
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;
use std::ops::Shl;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let k = input.read_usize();
    let q = input.read_usize();
    let edges: Vec<(usize, usize, usize)> = input.read_vec(m);

    let mut dsu = DSU::new(n);
    let mut graph = Graph::new(n);
    for &(u, v, t) in &edges {
        if dsu.join(u - 1, v - 1) {
            graph.add_edge(u - 1, BiWeightedEdge::new(v - 1, 1i32.shl(t - 1)));
        }
    }
    let mut val = vec![0; n];
    for i in dsu.iter() {
        let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, v: i32| {
            val[vert] = v;
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                f.call(e.to(), vert, v ^ e.weight());
            }
        });
        dfs.call(i, i, 0);
    }
    let mut shifts: Vec<Vec<i32>> = vec![Vec::new(); n];
    let mut present = vec![0; n];
    for (u, v, t) in edges {
        present[dsu.get(u - 1)].set_bit(t - 1);
        let c_val = val[u - 1] ^ val[v - 1] ^ 1.shl(t - 1);
        if c_val != 0 {
            shifts[dsu.get(u - 1)].push(c_val);
        }
    }
    for i in dsu.iter() {
        let mut at = 0;
        for j in (0..k).rev() {
            let mut cur = None;
            for l in at..shifts[i].len() {
                if shifts[i][l].is_set(j) {
                    cur = Some(l);
                    break;
                }
            }
            if let Some(cur) = cur {
                shifts[i].swap(at, cur);
                for l in 0..shifts[i].len() {
                    if l == at {
                        continue;
                    }
                    if shifts[i][l].is_set(j) {
                        shifts[i][l] ^= shifts[i][at];
                    }
                }
                at += 1;
            }
        }
        while let Some(&v) = shifts[i].last() {
            if v != 0 {
                break;
            }
            shifts[i].pop();
        }
    }
    let normalize = |m: &mut i32, i: usize| {
        let p = dsu.get(i);
        for &j in &shifts[p] {
            m.minim((*m) ^ j);
        }
    };
    for i in 0..n {
        normalize(&mut val[i], i);
    }

    // let mut all_bits = vec![i32::all_bits(k); n];
    // for i in dsu.iter() {
    //     let m = &mut all_bits[i];
    //     for &j in &shifts[i] {
    //         m.minim((*m) ^ j);
    //     }
    // }

    output().bool_output = BoolOutput::YesNo;

    for _ in 0..q {
        let s = input.read_usize() - 1;
        let t = input.read_usize() - 1;
        if s == t {
            out_line!(true);
            continue;
        }
        let mut to_normalize = val[s] ^ val[t] ^ i32::all_bits(k);
        normalize(&mut to_normalize, dsu.get(s));
        out_line!(
            present[dsu.get(s)] == usize::all_bits(k)
                && dsu.get(s) == dsu.get(t)
                && (val[s] == val[t] || to_normalize == 0)
        );
    }
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
