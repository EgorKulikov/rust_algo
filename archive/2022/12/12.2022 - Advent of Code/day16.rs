//{"name":"day16","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day16"}}}

use algo_lib::collections::arr3d::Arr3d;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::all_distances::AllDistances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::{EolString, Input};
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::{out_line, scan};
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    let mut valves = Vec::new();
    let mut id_to_number = HashMap::new();
    let mut non_zero = Vec::new();

    while !input.is_exhausted() {
        scan!(
            input,
            "Valve @ has flow rate=@; tunnel@ lead@ to valve@ @",
            id: String,
            flow: i64,
            _s: EolString,
            _s: EolString,
            _s: EolString,
            tunnels: EolString,
        );
        id_to_number.insert(id, valves.len());
        if flow != 0 {
            non_zero.push(valves.len());
        }
        valves.push((
            flow,
            tunnels.split(", ").map(|x| x.to_string()).collect_vec(),
        ));
        input.skip_whitespace();
    }

    let mut graph = Graph::new(valves.len());
    let mut f = Vec::with_capacity(non_zero.len());
    for (i, (flow, tunnels)) in valves.into_iter().enumerate() {
        if flow != 0 {
            f.push(flow);
        }
        for tunnel in tunnels {
            graph.add_edge(i, WeightedEdge::new(*id_to_number.get(&tunnel).unwrap(), 1));
        }
    }
    let start = id_to_number["AA"];
    let d = graph.all_distances();
    const TIME: usize = 26;
    // const TIME: usize = 30;
    let mut best = vec![0; 1 << non_zero.len()];
    for _ in 0..2 {
        // for _ in 0..1 {
        let mut ans = Arr3d::new(TIME + 1, non_zero.len(), 1 << non_zero.len(), None);
        for i in 0..non_zero.len() {
            for j in 0..best.len() {
                if !j.is_set(i) {
                    let dst = d[(start, non_zero[i])] + 1;
                    if dst <= TIME {
                        ans[(dst, i, j.with_bit(i))]
                            .maxim(best[j] + f[i] * (TIME.into_i64() - dst.into_i64()));
                    }
                }
            }
        }
        for i in 0..ans.d1() {
            for j in 0..ans.d2() {
                for k in 0..ans.d3() {
                    if let Some(x) = ans[(i, j, k)] {
                        for l in 0..ans.d2() {
                            let ni = i + d[(non_zero[j], non_zero[l])] + 1;
                            if ni <= TIME && !k.is_set(l) {
                                ans[(ni, l, k.with_bit(l))]
                                    .maxim(x + f[l] * (TIME.into_i64() - ni.into_i64()));
                            }
                        }
                    }
                }
            }
        }
        for i in 0..best.len() {
            best[i] = 0;
            for j in 0..ans.d1() {
                for k in 0..ans.d2() {
                    if let Some(x) = ans[(j, k, i)] {
                        best[i].maxim(x);
                    }
                }
            }
        }
    }
    out_line!(best.into_iter().max());
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
