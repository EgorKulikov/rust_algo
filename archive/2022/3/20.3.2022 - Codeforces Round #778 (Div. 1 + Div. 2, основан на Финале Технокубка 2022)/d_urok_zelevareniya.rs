//{"name":"D. Урок зельеварения","group":"Codeforces - Codeforces Round #778 (Div. 1 + Div. 2, основан на Финале Технокубка 2022)","url":"https://codeforces.com/contest/1654/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n4\n3 2 3 4\n1 2 4 3\n1 4 2 4\n8\n5 4 2 3\n6 4 5 4\n1 3 5 2\n6 8 2 1\n3 5 3 4\n3 2 2 5\n6 7 4 3\n17\n8 7 4 16\n9 17 4 5\n5 14 13 12\n11 1 17 14\n6 13 8 9\n2 11 3 11\n4 17 7 2\n17 16 8 6\n15 5 1 14\n16 7 1 10\n12 17 13 10\n11 16 7 2\n10 11 6 4\n13 17 14 6\n3 11 15 8\n15 6 12 8\n","output":"69\n359\n573672453\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DUrokZelevareniya"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{
    Callable2, Callable3, RecursiveFunction2, RecursiveFunction3,
};
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::numbers::rational::Rational;
use algo_lib::out_line;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let edges: Vec<(usize, usize, i32, i32)> = input.read_vec(n - 1);

    let mut graph = Graph::new(n);
    for (i, j, x, y) in edges {
        graph.add_edge(i - 1, WeightedEdge::new(j - 1, Rational::new(x, y)));
        graph.add_edge(j - 1, WeightedEdge::new(i - 1, Rational::new(y, x)));
    }

    type Mod = ModIntF;

    struct Ratio(DefaultMap<i32, usize>);
    impl Ratio {
        pub fn new() -> Self {
            Self(DefaultMap::new())
        }

        fn dec(&mut self, i: i32) {
            if self.0[i] > 0 {
                self.0[i] -= 1;
            }
        }

        fn inc(&mut self, i: i32) {
            self.0[i] += 1;
        }

        pub fn add_edge(&mut self, edge: Rational<i32>) {
            let mut den = edge.den();
            for i in 2.. {
                if i * i > den {
                    break;
                }
                while den % i == 0 {
                    den /= i;
                    self.dec(i);
                }
            }
            if den != 1 {
                self.dec(den);
            }
            let mut num = edge.num();
            for i in 2.. {
                if i * i > num {
                    break;
                }
                while num % i == 0 {
                    num /= i;
                    self.inc(i);
                }
            }
            if num != 1 {
                self.inc(num);
            }
        }

        pub fn join(&mut self, mut other: Self) {
            if other.0.len() > self.0.len() {
                swap(self, &mut other);
            }
            for (i, v) in other.0 {
                self.0[i].maxim(v);
            }
        }

        pub fn value(&self) -> Mod {
            let mut res = Mod::one();
            for (&i, &v) in self.0.iter() {
                res *= Mod::new(i).power(v);
            }
            res
        }
    }

    let mut dfs = RecursiveFunction2::new(|f, vert, prev| -> Ratio {
        let mut res = Ratio::new();
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let mut call = f.call(e.to(), vert);
            call.add_edge(e.weight());
            res.join(call);
        }
        res
    });
    let base = dfs.call(0, 0).value();
    let mut ans = Mod::zero();
    let mut dfs2 = RecursiveFunction3::new(|f, vert, prev, value| {
        ans += value;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(
                e.to(),
                vert,
                value * Mod::new(e.weight().den()) / Mod::new(e.weight().num()),
            );
        }
    });
    dfs2.call(0, 0, base);
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
