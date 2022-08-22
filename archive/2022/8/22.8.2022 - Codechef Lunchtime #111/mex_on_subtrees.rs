//{"name":"Mex On Subtrees","group":"CodeChef - LTIME111A","url":"https://www.codechef.com/LTIME111A/problems-old/MEXSUBTR","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n0 0\n1\n2\n0 1\n1\n5\n2 2 1 0 0\n1 2 3 4\n6\n2 2 1 1 1 0\n1 2 2 2 3\n","output":"2\n-1\n7\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MexOnSubtrees"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;
use std::collections::BTreeMap;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let b = input.read_usize_vec(n);
    let p = input.read_usize_vec(n - 1).dec_by_one();

    #[derive(Default)]
    struct MultiSet {
        data: BTreeMap<usize, usize>,
    }

    impl MultiSet {
        fn add_some(&mut self, at: usize, some: usize) {
            if let Some(v) = self.data.get_mut(&at) {
                *v += some;
            } else {
                self.data.insert(at, some);
            }
        }

        fn add(&mut self, at: usize) {
            self.add_some(at, 1);
        }

        fn get_last(&mut self) -> bool {
            if let Some((&at, v)) = self.data.iter_mut().rev().next() {
                *v -= 1;
                if *v == 0 {
                    self.data.remove(&at);
                }
                true
            } else {
                false
            }
        }

        fn upgrade(&mut self, at: usize) {
            if let Some(&v) = self.data.get(&at) {
                self.add_some(at + 1, v);
                self.data.remove(&at);
            }
        }

        fn join(&mut self, mut other: Self) {
            if self.data.len() < other.data.len() {
                swap(self, &mut other);
            }
            for (k, v) in other.data {
                self.add_some(k, v);
            }
        }

        fn value(self) -> usize {
            let mut res = 0;
            for (i, v) in self.data {
                res += i * v;
            }
            res
        }
    }

    let mut graph = Graph::new(n);
    for i in 0..n - 1 {
        if b[i + 1] > b[p[i]] {
            out_line!(-1);
            return;
        }
        graph.add_edge(i + 1, BiEdge::new(p[i]));
    }
    let mut dfs =
        RecursiveFunction2::new(|f, vert: usize, prev: usize| -> Option<(usize, MultiSet)> {
            let mut ans = 0;
            let mut good = MultiSet::default();
            good.add(0);
            let mut bad = MultiSet::default();
            let mut has = 0;
            for e in &graph[vert] {
                if e.to() != prev {
                    has.maxim(b[e.to()]);
                }
            }
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                let call = f.call(e.to(), vert);
                if call.is_none() {
                    return None;
                }
                let (call_ans, call_set) = call.unwrap();
                if b[e.to()] == has {
                    bad.join(call_set);
                } else {
                    good.join(call_set);
                }
                ans += call_ans;
            }
            if has < b[vert] {
                assert!(good.get_last());
                ans += has;
                has += 1;
            }
            good.join(bad);
            while has < b[vert] {
                if !good.get_last() {
                    return None;
                }
                ans += has;
                has += 1;
            }
            good.upgrade(b[vert]);
            Some((ans, good))
        });
    let res = dfs.call(0, n);
    if res.is_none() {
        out_line!(-1);
        return;
    }
    let (ans, set) = res.unwrap();
    out_line!(ans + set.value());
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
