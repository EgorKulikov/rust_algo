//{"name":"D. Дружелюбные пауки","group":"Codeforces - Codeforces Round #843 (Div. 2)","url":"https://codeforces.com/contest/1775/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n2 14 9 6 8 15 11\n5 6\n","output":"3\n5 4 6\n"},{"input":"7\n2 14 9 6 8 15 11\n5 7\n","output":"-1\n"},{"input":"7\n2 14 9 6 8 15 11\n5 5\n","output":"1\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDruzhelyubniePauki"}}}

use algo_lib::collections::vec_ext::Bounds;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::primes::{divisor_table, primes};
use algo_lib::out_line;
use std::iter::once;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let a = input.read_size_vec(n);
    let s = input.read_size() - 1;
    let t = input.read_size() - 1;

    let lim = a.iter().max().copied().unwrap() + 1;
    let pr = primes(lim);
    let d = divisor_table(lim);

    let mut graph = Graph::new(n + pr.len());
    for (i, a) in a.into_iter().enumerate() {
        let mut c = a;
        while c > 1 {
            let p: usize = d[c];
            let j = n + pr.lower_bound(&p);
            graph.add_edge(i, BiWeightedEdge::new(j, 1));
            while c % p == 0 {
                c /= p;
            }
        }
    }

    let res = graph.distance(s, t);
    match res {
        None => {
            out_line!(-1);
        }
        Some((dist, path)) => {
            out_line!(dist / 2 + 1);
            out_line!(path
                .into_iter()
                .filter_map(|(v, _)| if v < n { Some(v + 1) } else { None })
                .chain(once(t + 1))
                .collect::<Vec<_>>());
        }
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
