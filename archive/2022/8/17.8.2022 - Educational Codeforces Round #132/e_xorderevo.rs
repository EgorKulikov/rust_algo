//{"name":"E. XOR дерево","group":"Codeforces - Educational Codeforces Round 132 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1709/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n3 2 1 3 2 1\n4 5\n3 4\n1 4\n2 1\n6 1\n","output":"2\n"},{"input":"4\n2 1 1 1\n1 2\n1 3\n1 4\n","output":"0\n"},{"input":"5\n2 2 2 2 2\n1 2\n2 3\n3 4\n4 5\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EXORDerevo"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;
use std::collections::HashSet;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_unsigned_vec(n);
    let edges = input.read_usize_pair_vec(n - 1).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }

    struct Paths {
        paths: HashSet<u32>,
        base: u32,
    }

    impl Paths {
        fn empty() -> Self {
            Paths {
                paths: HashSet::new(),
                base: 0,
            }
        }

        fn single(a: u32) -> Self {
            Paths {
                paths: HashSet::from([0]),
                base: a,
            }
        }

        fn intersects(&self, other: &Self) -> bool {
            if other.paths.len() < self.paths.len() {
                other.intersects(self)
            } else {
                self.paths
                    .iter()
                    .any(|&x| other.paths.contains(&(x ^ self.base ^ other.base)))
            }
        }

        fn unite(&mut self, other: &mut Self, by: u32) {
            if other.paths.len() > self.paths.len() {
                self.base ^= by;
                other.base ^= by;
                other.unite(self, by);
                swap(self, other);
            } else {
                for &x in &other.paths {
                    self.paths.insert(x ^ self.base ^ other.base ^ by);
                }
            }
        }
    }

    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (usize, Paths) {
        let mut ans = 0;
        let mut is_empty = false;
        let mut paths = Paths::single(a[vert]);
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let (call_ans, mut call_paths) = f.call(e.to(), vert);
            ans += call_ans;
            if !is_empty {
                if paths.intersects(&call_paths) {
                    is_empty = true;
                    ans += 1;
                } else {
                    paths.unite(&mut call_paths, a[vert]);
                }
            }
        }
        (ans, if is_empty { Paths::empty() } else { paths })
    });
    let (ans, _) = dfs.call(0, 0);
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
